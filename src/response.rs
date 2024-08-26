use crate::constants::CRLF;
use std::collections::HashMap;
use std::fs;

fn get_status_text(status: u32) -> String {
    match status {
        200 => String::from("OK"),
        404 => String::from("Not Found"),
        500 => String::from("Internal Server Error"),
        _ => String::from(""),
    }
}

pub struct Response {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 200,
            headers: HashMap::new(),
            content: Vec::new(),
        }
    }

    /* From */
    pub fn from_status(status: u32) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            content: Vec::new(),
        }
    }

    pub fn from_content_string(content: String) -> Self {
        Response {
            status: 200,
            headers: HashMap::from([("Content-Length".into(), content.len().to_string())]),
            content: content.into(),
        }
    }

    pub fn from_content_bytevec(content: Vec<u8>) -> Self {
        Response {
            status: 200,
            headers: HashMap::from([("Content-Length".into(), content.len().to_string())]),
            content,
        }
    }

    pub fn file(path: &str) -> Self {
        let content_wrapped = fs::read(path);

        if let Ok(content) = content_wrapped {
            Response::from_content_bytevec(content)
        } else {
            Response::from_status(500)
        }
    }

    pub fn html<StringLike: Into<String>>(content: StringLike) -> Self {
        let mut res = Response::from_content_string(content.into());
        res.set_header("Content-Type", "text/html");
        res
    }

    /* Quick responses */
    pub fn not_found() -> Self {
        Response::from_status(404)
    }

    /* Set */
    pub fn set_content(&mut self, content: Vec<u8>) {
        self.headers
            .insert("Content-Length".into(), content.len().to_string());
        self.content = content;
    }

    pub fn set_status(&mut self, status: u32) {
        self.status = status;
    }

    pub fn set_header<StringLike: Into<String>>(&mut self, key: StringLike, value: StringLike) {
        self.headers.insert(key.into(), value.into());
    }

    /* Inline set */
    pub fn with_header<StringLike: Into<String>>(
        mut self,
        key: StringLike,
        value: StringLike,
    ) -> Self {
        self.set_header(key, value);
        self
    }

    pub fn with_status(mut self, status: u32) -> Self {
        self.set_status(status);
        self
    }

    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.set_content(content);
        self
    }

    /* Build */
    pub fn build(self) -> Vec<u8> {
        let headers_string = self
            .headers
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<String>>()
            .join(CRLF);

        let status = self.status;
        let status_text = get_status_text(status);

        let response_headers = format!(
            "HTTP/1.1 {} {}{CRLF}{}{CRLF}{CRLF}",
            status, status_text, headers_string,
        )
        .into_bytes();

        let full_response_text = [response_headers, self.content].concat();

        full_response_text
    }
}

impl From<String> for Response {
    fn from(value: String) -> Self {
        Response::from_content_string(value)
    }
}

impl From<&str> for Response {
    fn from(value: &str) -> Self {
        Response::from(value.to_string())
    }
}

impl From<Vec<u8>> for Response {
    fn from(value: Vec<u8>) -> Self {
        Response::from_content_bytevec(value)
    }
}
