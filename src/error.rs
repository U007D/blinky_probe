#[expect(clippy::wildcard_imports, reason = "Ok to use wildcard import on `shared_consts`.")]
use crate::shared_consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {}: ", msg::ERR_IO, 0)]
    Io(#[from] std::io::Error),
}
