use lib::{self, Result};

#[expect(
    clippy::missing_const_for_fn,
    reason = "Remove this attribute when function body is implemented"
)]
#[termination::display]
fn main() -> Result<()> {
    Ok(())
}
