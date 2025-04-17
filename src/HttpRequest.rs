#[allow(dead_code)]
pub enum MethodKind {
    GET,
    POST,
    PUT,
}

pub struct HttpRequest {
    method: MethodKind,
    url: String,
    protocol: String,
    // headers: Vec<String>,
}

impl HttpRequest {
    pub fn new(method: MethodKind, url: String, protocol: Option<String>) -> HttpRequest {
        HttpRequest {
            method: method,
            url: url,
            protocol: match protocol {
                Some(p) => p,
                None => "HTTP/1.1".to_string(),
            },
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // to bytes
        /*
            GET / HTTP/1.1\r\n
            Host: bleps.ch\r\n
            <Headers>
        */
        
        let meth = match self.method {
            MethodKind::GET => "GET",
            MethodKind::POST => "POST",
            MethodKind::PUT => "PUT",
        };
        
        let mut out_str = format!("{} / {}\r\n", meth, &self.protocol);
        out_str = format!("{}Host: {}\r\n", out_str, self.url);

        // TODO: add headers
        
        out_str = format!("{}\r\n", out_str);
        out_str.into_bytes()
    }
}
