//! A tiny HTTP mock server, so the tests can assert the exact requests the
//! client makes without reaching the network.

use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// A request the mock server received.
#[derive(Debug, Clone, Default)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl RecordedRequest {
    /// Look a header up, case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        let lower = name.to_ascii_lowercase();
        self.headers
            .iter()
            .find(|(key, _)| key.to_ascii_lowercase() == lower)
            .map(|(_, value)| value.as_str())
    }

    /// Decode the request body as JSON.
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.body).unwrap_or(serde_json::Value::Null)
    }

    /// The HTTP Basic credentials, decoded.
    pub fn basic_credentials(&self) -> Option<(String, String)> {
        use base64::Engine;
        let header = self.header("authorization")?;
        let encoded = header.strip_prefix("Basic ")?;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .ok()?;
        let raw = String::from_utf8(decoded).ok()?;
        let (user, password) = raw.split_once(':')?;
        Some((user.to_string(), password.to_string()))
    }
}

/// A mock HTTP server that records requests and replays canned responses.
pub struct MockServer {
    base_url: String,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

impl MockServer {
    /// Start a server that always answers with `status` and `body`.
    pub async fn start(status: u16, body: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let address = listener.local_addr().expect("local addr");
        let base_url = format!("http://{}", address);
        let requests: Arc<Mutex<Vec<RecordedRequest>>> = Arc::new(Mutex::new(Vec::new()));

        let recorder = Arc::clone(&requests);
        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status,
            body.len(),
            body
        );

        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let recorder = Arc::clone(&recorder);
                let response = response.clone();
                tokio::spawn(async move {
                    let Some(request) = read_request(&mut stream).await else {
                        return;
                    };
                    recorder.lock().unwrap().push(request);
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.flush().await;
                });
            }
        });

        Self { base_url, requests }
    }

    /// The base URL to point a client at.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Every request received so far.
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }

    /// The most recent request.
    pub fn last(&self) -> RecordedRequest {
        self.requests().pop().expect("no request was made")
    }

    /// How many requests were received.
    pub fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}

async fn read_request(stream: &mut tokio::net::TcpStream) -> Option<RecordedRequest> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 1024];

    // Read until the end of the headers.
    let header_end = loop {
        let read = stream.read(&mut chunk).await.ok()?;
        if read == 0 {
            return None;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(position) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            break position;
        }
    };

    let header_text = String::from_utf8_lossy(&buffer[..header_end]).to_string();
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_string();
    let target = parts.next()?.to_string();

    let mut headers = Vec::new();
    let mut content_length = 0usize;
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            }
            headers.push((name, value));
        }
    }

    let body_start = header_end + 4;
    while buffer.len() < body_start + content_length {
        let read = stream.read(&mut chunk).await.ok()?;
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
    }

    let body = String::from_utf8_lossy(&buffer[body_start..]).to_string();
    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path.to_string(), Some(query.to_string())),
        None => (target, None),
    };

    Some(RecordedRequest {
        method,
        path,
        query,
        headers,
        body,
    })
}
