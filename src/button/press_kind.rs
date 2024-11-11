use derive_more::Display;
use embassy_time::Duration;

use crate::shared_const::LONG_PRESS_DURATION_IN_MS;

// Instead of having a vague API describing a short vs. a long button press as a `bool`, we define
// an `enum` to clarify what each state represents.  The compiler should compile this down to the
// very same `boolean` that we might have coded by hand.
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PressKind {
    #[default]
    Short,
    Long,
}

// Make `PressKind` solely responsible for the distinction in `Duration` between a short and long
// button press.
impl From<Duration> for PressKind {
    fn from(duration: Duration) -> Self {
        match duration >= Duration::from_millis(LONG_PRESS_DURATION_IN_MS) {
            true => PressKind::Long,
            false => PressKind::Short,
        }
    }
}
