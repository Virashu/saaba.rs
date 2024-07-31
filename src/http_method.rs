#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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
        value.as_str().into()
    }
}

impl From<&str> for HTTPMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
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
