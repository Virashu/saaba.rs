#[derive(Eq, Hash, PartialEq, Clone)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    UPDATE,
    CONFIG,
}

impl From<String> for HTTPMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "DELETE" => HTTPMethod::DELETE,
            "UPDATE" => HTTPMethod::UPDATE,
            "CONFIG" => HTTPMethod::CONFIG,
            _ => HTTPMethod::GET,
        }
    }
}
