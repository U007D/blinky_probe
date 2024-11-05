mod en_us;
#[expect(clippy::wildcard_imports, reason = "Ok to use wildcard import on `shared_consts`.")]
pub use en_us::*;
