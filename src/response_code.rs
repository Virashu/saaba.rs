#[derive(Debug)]
pub enum ResponseCode {
    OK = 200,
    MovedPermanently = 301,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}

impl From<ResponseCode> for u32 {
    fn from(value: ResponseCode) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for ResponseCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(Self::OK),
            301 => Ok(Self::MovedPermanently),
            307 => Ok(Self::TemporaryRedirect),
            308 => Ok(Self::PermanentRedirect),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            402 => Ok(Self::PaymentRequired),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            500 => Ok(Self::InternalServerError),

            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OK => "OK",
            Self::MovedPermanently => "Moved Permanently",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Required",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        };

        write!(f, "{}", s)
    }
}
