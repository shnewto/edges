#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to convert bevy image {0:?}")]
    BevyImageConversion(String),
}
