#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Default)]
pub enum HTTPMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    UPDATE,
    CONFIG,
    HEAD,
    TRACE,
}

impl TryFrom<String> for HTTPMethod {
    type Error = UnknownMethodError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}


impl TryFrom<&str> for HTTPMethod {
    type Error = UnknownMethodError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "UPDATE" => Ok(Self::UPDATE),
            "CONFIG" => Ok(Self::CONFIG),
            "HEAD" => Ok(Self::HEAD),
            "TRACE" => Ok(Self::TRACE),

            m => Err(UnknownMethodError {
                method: m.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct UnknownMethodError {
    method: String,
}

impl std::fmt::Display for UnknownMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to convert method: `{}`", self.method)
    }
}
