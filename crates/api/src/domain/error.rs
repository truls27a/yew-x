#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{resource_type} with {field} '{value}' not found")]
    NotFound {
        resource_type: &'static str,
        field: &'static str,
        value: String,
    },

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: &'static str },

    #[error("Forbidden: {reason}")]
    Forbidden { reason: &'static str },

    #[error("Conflict on {resource_type}: {reason}")]
    Conflict {
        resource_type: &'static str,
        reason: &'static str,
    },

    #[error("Validation failed on {field}: {reason}")]
    Validation {
        field: &'static str,
        reason: &'static str,
    },

    #[error("Internal error: {message}")]
    Internal {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

