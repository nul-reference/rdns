#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Format of packet is not what was expected")]
    FormatError,
}
