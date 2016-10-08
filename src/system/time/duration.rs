// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const MILLIS_PER_SEC: u64 = 1_000;

/// A duration type to represent a span of time, typically used for system
/// timeouts.
///
/// Each duration is composed of a number of seconds and nanosecond precision.
/// APIs binding a system timeout will typically round up the nanosecond
/// precision if the underlying system does not support that level of precision.
///
/// Durations implement many common traits, including `Add`, `Sub`, and other
/// ops traits. Currently a duration may only be inspected for its number of
/// seconds and its nanosecond precision.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// let five_seconds = Duration::new(5, 0);
/// let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
///
/// assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
/// assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);
///
/// let ten_millis = Duration::from_millis(10);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Duration {
    secs: u64,
    nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
}

impl Duration {
    /// Creates a new `Duration` from the specified number of seconds and
    /// additional nanosecond precision.
    ///
    /// If the nanoseconds is greater than 1 billion (the number of nanoseconds
    /// in a second), then it will carry over into the seconds provided.
    ///
    /// # Panics
    ///
    /// This constructor will panic if the carry from the nanoseconds overflows
    /// the seconds counter.
    #[inline]
    pub fn new(secs: u64, nanos: u32) -> Duration {
        let secs = secs.checked_add((nanos / NANOS_PER_SEC) as u64)
            .expect("overflow in Duration::new");
        let nanos = nanos % NANOS_PER_SEC;
        Duration {
            secs: secs,
            nanos: nanos,
        }
    }

    /// Creates a new `Duration` from the specified number of seconds.
    #[inline]
    pub fn from_secs(secs: u64) -> Duration {
        Duration {
            secs: secs,
            nanos: 0,
        }
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    #[inline]
    pub fn from_millis(millis: u64) -> Duration {
        let secs = millis / MILLIS_PER_SEC;
        let nanos = ((millis % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI;
        Duration {
            secs: secs,
            nanos: nanos,
        }
    }

    /// Returns the number of whole seconds represented by this duration.
    ///
    /// The extra precision represented by this duration is ignored (i.e. extra
    /// nanoseconds are not represented in the returned value).
    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.secs
    }

    /// Returns the nanosecond precision represented by this duration.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by nanoseconds. The returned number always represents a
    /// fractional portion of a second (i.e. it is less than one billion).
    #[inline]
    pub fn subsec_nanos(&self) -> u32 {
        self.nanos
    }

    /// Checked duration addition. Computes `self + other`, returning `None`
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(duration_checked_ops)]
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 0).checked_add(Duration::new(0, 1)), Some(Duration::new(0, 1)));
    /// assert_eq!(Duration::new(1, 0).checked_add(Duration::new(std::u64::MAX, 0)), None);
    /// ```
    #[inline]
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        if let Some(mut secs) = self.secs.checked_add(rhs.secs) {
            let mut nanos = self.nanos + rhs.nanos;
            if nanos >= NANOS_PER_SEC {
                nanos -= NANOS_PER_SEC;
                if let Some(new_secs) = secs.checked_add(1) {
                    secs = new_secs;
                } else {
                    return None;
                }
            }
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration {
                secs: secs,
                nanos: nanos,
            })
        } else {
            None
        }
    }

    /// Checked duration subtraction. Computes `self + other`, returning `None`
    /// if the result would be negative or if underflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(duration_checked_ops)]
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 1).checked_sub(Duration::new(0, 0)), Some(Duration::new(0, 1)));
    /// assert_eq!(Duration::new(0, 0).checked_sub(Duration::new(0, 1)), None);
    /// ```
    #[inline]
    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        if let Some(mut secs) = self.secs.checked_sub(rhs.secs) {
            let nanos = if self.nanos >= rhs.nanos {
                self.nanos - rhs.nanos
            } else {
                if let Some(sub_secs) = secs.checked_sub(1) {
                    secs = sub_secs;
                    self.nanos + NANOS_PER_SEC - rhs.nanos
                } else {
                    return None;
                }
            };
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration {
                secs: secs,
                nanos: nanos,
            })
        } else {
            None
        }
    }

    /// Checked duration multiplication. Computes `self * other`, returning
    /// `None` if underflow or overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(duration_checked_ops)]
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 500_000_001).checked_mul(2), Some(Duration::new(1, 2)));
    /// assert_eq!(Duration::new(std::u64::MAX - 1, 0).checked_mul(2), None);
    /// ```
    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Duration> {
        // Multiply nanoseconds as u64, because it cannot overflow that way.
        let total_nanos = self.nanos as u64 * rhs as u64;
        let extra_secs = total_nanos / (NANOS_PER_SEC as u64);
        let nanos = (total_nanos % (NANOS_PER_SEC as u64)) as u32;
        if let Some(secs) = self.secs
            .checked_mul(rhs as u64)
            .and_then(|s| s.checked_add(extra_secs)) {
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration {
                secs: secs,
                nanos: nanos,
            })
        } else {
            None
        }
    }

    /// Checked duration division. Computes `self / other`, returning `None`
    /// if `other == 0` or the operation results in underflow or overflow.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(duration_checked_ops)]
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(2, 0).checked_div(2), Some(Duration::new(1, 0)));
    /// assert_eq!(Duration::new(1, 0).checked_div(2), Some(Duration::new(0, 500_000_000)));
    /// assert_eq!(Duration::new(2, 0).checked_div(0), None);
    /// ```
    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Duration> {
        if rhs != 0 {
            let secs = self.secs / (rhs as u64);
            let carry = self.secs - secs * (rhs as u64);
            let extra_nanos = carry * (NANOS_PER_SEC as u64) / (rhs as u64);
            let nanos = self.nanos / rhs + (extra_nanos as u32);
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration {
                secs: secs,
                nanos: nanos,
            })
        } else {
            None
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("overflow when adding durations")
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs).expect("overflow when subtracting durations")
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u32) -> Duration {
        self.checked_mul(rhs).expect("overflow when multiplying duration by scalar")
    }
}

impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, rhs: u32) -> Duration {
        self.checked_div(rhs).expect("divide by zero error when dividing duration by scalar")
    }
}

impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Duration;

    #[test]
    fn creation() {
        assert!(Duration::from_secs(1) != Duration::from_secs(0));
        assert_eq!(Duration::from_secs(1) + Duration::from_secs(2),
                   Duration::from_secs(3));
        assert_eq!(Duration::from_millis(10) + Duration::from_secs(4),
                   Duration::new(4, 10 * 1_000_000));
        assert_eq!(Duration::from_millis(4000), Duration::new(4, 0));
    }

    #[test]
    fn secs() {
        assert_eq!(Duration::new(0, 0).as_secs(), 0);
        assert_eq!(Duration::from_secs(1).as_secs(), 1);
        assert_eq!(Duration::from_millis(999).as_secs(), 0);
        assert_eq!(Duration::from_millis(1001).as_secs(), 1);
    }

    #[test]
    fn nanos() {
        assert_eq!(Duration::new(0, 0).subsec_nanos(), 0);
        assert_eq!(Duration::new(0, 5).subsec_nanos(), 5);
        assert_eq!(Duration::new(0, 1_000_000_001).subsec_nanos(), 1);
        assert_eq!(Duration::from_secs(1).subsec_nanos(), 0);
        assert_eq!(Duration::from_millis(999).subsec_nanos(), 999 * 1_000_000);
        assert_eq!(Duration::from_millis(1001).subsec_nanos(), 1 * 1_000_000);
    }

    #[test]
    fn add() {
        assert_eq!(Duration::new(0, 0) + Duration::new(0, 1),
                   Duration::new(0, 1));
        assert_eq!(Duration::new(0, 500_000_000) + Duration::new(0, 500_000_001),
                   Duration::new(1, 1));
    }

    #[test]
    fn checked_add() {
        assert_eq!(Duration::new(0, 0).checked_add(Duration::new(0, 1)),
                   Some(Duration::new(0, 1)));
        assert_eq!(Duration::new(0, 500_000_000).checked_add(Duration::new(0, 500_000_001)),
                   Some(Duration::new(1, 1)));
        assert_eq!(Duration::new(1, 0).checked_add(Duration::new(::u64::MAX, 0)),
                   None);
    }

    #[test]
    fn sub() {
        assert_eq!(Duration::new(0, 1) - Duration::new(0, 0),
                   Duration::new(0, 1));
        assert_eq!(Duration::new(0, 500_000_001) - Duration::new(0, 500_000_000),
                   Duration::new(0, 1));
        assert_eq!(Duration::new(1, 0) - Duration::new(0, 1),
                   Duration::new(0, 999_999_999));
    }

    #[test]
    fn checked_sub() {
        let zero = Duration::new(0, 0);
        let one_nano = Duration::new(0, 1);
        let one_sec = Duration::new(1, 0);
        assert_eq!(one_nano.checked_sub(zero), Some(Duration::new(0, 1)));
        assert_eq!(one_sec.checked_sub(one_nano),
                   Some(Duration::new(0, 999_999_999)));
        assert_eq!(zero.checked_sub(one_nano), None);
        assert_eq!(zero.checked_sub(one_sec), None);
    }

    #[test]
    #[should_panic]
    fn sub_bad1() {
        Duration::new(0, 0) - Duration::new(0, 1);
    }

    #[test]
    #[should_panic]
    fn sub_bad2() {
        Duration::new(0, 0) - Duration::new(1, 0);
    }

    #[test]
    fn mul() {
        assert_eq!(Duration::new(0, 1) * 2, Duration::new(0, 2));
        assert_eq!(Duration::new(1, 1) * 3, Duration::new(3, 3));
        assert_eq!(Duration::new(0, 500_000_001) * 4, Duration::new(2, 4));
        assert_eq!(Duration::new(0, 500_000_001) * 4000,
                   Duration::new(2000, 4000));
    }

    #[test]
    fn checked_mul() {
        assert_eq!(Duration::new(0, 1).checked_mul(2),
                   Some(Duration::new(0, 2)));
        assert_eq!(Duration::new(1, 1).checked_mul(3),
                   Some(Duration::new(3, 3)));
        assert_eq!(Duration::new(0, 500_000_001).checked_mul(4),
                   Some(Duration::new(2, 4)));
        assert_eq!(Duration::new(0, 500_000_001).checked_mul(4000),
                   Some(Duration::new(2000, 4000)));
        assert_eq!(Duration::new(::u64::MAX - 1, 0).checked_mul(2), None);
    }

    #[test]
    fn div() {
        assert_eq!(Duration::new(0, 1) / 2, Duration::new(0, 0));
        assert_eq!(Duration::new(1, 1) / 3, Duration::new(0, 333_333_333));
        assert_eq!(Duration::new(99, 999_999_000) / 100,
                   Duration::new(0, 999_999_990));
    }

    #[test]
    fn checked_div() {
        assert_eq!(Duration::new(2, 0).checked_div(2),
                   Some(Duration::new(1, 0)));
        assert_eq!(Duration::new(1, 0).checked_div(2),
                   Some(Duration::new(0, 500_000_000)));
        assert_eq!(Duration::new(2, 0).checked_div(0), None);
    }
}
