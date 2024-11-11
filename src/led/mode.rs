use enum_iterator::Sequence;

/// Define the "operation" modes of the LED.  States may be added or removed, and once their
/// behavior is specified in/removed from `led_driver()`, they should "just work".
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Sequence)]
pub enum LedMode {
    #[default]
    FastFlash,
    SlowFlash,
    On,
    Off,
}