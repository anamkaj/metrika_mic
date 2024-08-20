use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponseMetrika {
    pub errors: Vec<ErrorDetail>,
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub error_type: String,
    pub message: String,
}
