// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::*;

fn to_milli(d: &Duration) -> u64 {
    match d {
        &MilliSeconds(m) => m,
        &Seconds(s) => (s as u64) * 1_000u64,
        &Minutes(min) => (min as u64) * 60_000u64,
    }
}

impl PartialEq<Duration> for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let other_milli = to_milli(other);
        let self_milli = to_milli(self);

        self_milli == other_milli
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
