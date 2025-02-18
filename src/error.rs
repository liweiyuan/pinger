#[derive(Debug, Clone, thiserror::Error)]
pub enum PingError {
    #[error("invaild config")]
    InvalidConfig(String),

    #[error("internal error")]
    InternalError,

    #[error("invalid buffer size")]
    InvalidBufferSize,

    #[error("invalid packet")]
    InvalidPacket,

    #[error("timeout")]
    Timeout,
}
