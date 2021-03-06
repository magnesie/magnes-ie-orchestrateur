/// This enum represents every type of error that a service can throw
pub enum ServiceError{
    /// Basic error containing only a string
    BasicError(String),
    /// Error thrown during request, here using the Reqwest library
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        ServiceError::RequestError(error)
    }
}

impl From<&str> for ServiceError {
    fn from(error_message: &str) -> Self {
        ServiceError::BasicError(String::from(error_message))
    }
}

impl From<String> for ServiceError {
    fn from(error_message: String) -> Self {
        ServiceError::BasicError(error_message)
    }
}

impl std::fmt::Display for ServiceError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceError::BasicError(error_message) => write!(f, "{}", error_message),
            ServiceError::RequestError(error) => write!(f, "{}", error.to_string())
        }
    }
}

impl From<ServiceError> for String {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::BasicError(err) => err,
            ServiceError::RequestError(err) => err.to_string()
        }
    }
}

