pub struct Response {
    pub headers: Vec<String>,
    pub content: String,
    pub content_type: String,
}

impl Response {
    pub fn new() -> Self {
        Response {
            headers: Vec::new(),
            content: String::from(""),
            content_type: String::from("text/html"),
        }
    }
}

impl From<String> for Response {
    fn from(value: String) -> Self {
        Response {
            headers: Vec::new(),
            content: value,
            content_type: String::from("text/html"),
        }
    }
}

impl From<&str> for Response {
    fn from(value: &str) -> Self {
        Response::from(value.to_string())
    }
}
