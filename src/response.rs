use crate::constants::CRLF;
use crate::header::Header;
use crate::utils::construct_message;
use crate::ResponseCode;
use std::collections::HashMap;
use std::fs;

pub struct Response {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: ResponseCode::OK.into(),
            headers: HashMap::new(),
            content: Vec::new(),
        }
    }

    /* From */
    pub fn from_status<StatusLike: Into<u32>>(status: StatusLike) -> Self {
        Response {
            status: status.into(),
            headers: HashMap::new(),
            content: Vec::new(),
        }
    }

    pub fn from_content_string(content: String) -> Self {
        Response {
            status: ResponseCode::OK.into(),
            headers: HashMap::from([(Header::ContentLength.into(), content.len().to_string())]),
            content: content.into(),
        }
    }

    pub fn from_content_bytevec(content: Vec<u8>) -> Self {
        Response {
            status: ResponseCode::OK.into(),
            headers: HashMap::from([(Header::ContentLength.into(), content.len().to_string())]),
            content,
        }
    }

    pub fn file(path: &str) -> Self {
        let content_wrapped = fs::read(path);

        if let Ok(content) = content_wrapped {
            Response::from_content_bytevec(content)
        } else {
            Response::from_status(ResponseCode::InternalServerError)
        }
    }

    pub fn html(content: impl Into<String>) -> Self {
        Response::from_content_string(content.into()).with_header(Header::ContentType, "text/html")
    }

    /* Quick responses */
    pub fn not_found() -> Self {
        let message = construct_message(format!("{:?}", ResponseCode::NotFound));
        Response::from_status(ResponseCode::NotFound).with_content(message.into_bytes())
    }

    pub fn redirect(url: impl Into<String>) -> Self {
        Response::from_status(ResponseCode::TemporaryRedirect).with_header(Header::Location, url)
    }

    /* Set */
    pub fn set_content(&mut self, content: Vec<u8>) {
        self.set_header(Header::ContentLength, content.len().to_string());
        self.content = content;
    }

    pub fn set_status(&mut self, status: u32) {
        self.status = status;
    }

    pub fn set_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into(), value.into());
    }

    /* Inline set */
    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.set_content(content);
        self
    }

    pub fn with_status(mut self, status: u32) -> Self {
        self.set_status(status);
        self
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set_header(key, value);
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
        let status_text = ResponseCode::try_from(status).map_or(String::new(), |r| format!("{:?}", r));

        let response_headers = format!(
            "HTTP/1.1 {} {}{CRLF}{}{CRLF}{CRLF}",
            status, status_text, headers_string,
        )
        .into_bytes();

        // Full response text
        [response_headers, self.content].concat()
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
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
