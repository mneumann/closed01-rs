extern crate rand;

use rand::{Rand, Rng};
use rand::Closed01 as RandClosed01;

/// Encapsulates a floating point number in the range [0, 1] including both endpoints.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Closed01<F>(F);

impl Closed01<f32> {
    #[inline(always)]
    pub fn new(f: f32) -> Closed01<f32> {
        assert!(f >= 0.0 && f <= 1.0);
        Closed01(f)
    }

    #[inline(always)]
    pub fn zero() -> Closed01<f32> {
        Closed01(0.0)
    }

    #[inline(always)]
    pub fn middle() -> Closed01<f32> {
        Closed01(0.5)
    }

    #[inline(always)]
    pub fn one() -> Closed01<f32> {
        Closed01(1.0)
    }

    #[inline(always)]
    /// Returns the smaller of the two.
    pub fn min(self, other: Closed01<f32>) -> Closed01<f32> {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    /// Returns the greater of the two.
    pub fn max(self, other: Closed01<f32>) -> Closed01<f32> {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    /// Returns the distance between the two numbers.
    pub fn distance(self, other: Closed01<f32>) -> Closed01<f32> {
        let d = (self.0 - other.0).abs();
        debug_assert!(d >= 0.0 && d <= 1.0);
        Closed01(d)
    }

    #[inline(always)]
    pub fn get(self) -> f32 {
        debug_assert!(self.0 >= 0.0 && self.0 <= 1.0);
        self.0
    }

    #[inline(always)]
    /// The average of two values.
    pub fn avg(a: Closed01<f32>, b: Closed01<f32>) -> Closed01<f32> {
        let avg = (a.get() + b.get()) / 2.0;
        debug_assert!(avg >= 0.0 && avg <= 1.0);
        Closed01(avg)
    }

    #[inline(always)]
    /// Saturating add
    pub fn saturating_add(self, other: Closed01<f32>) -> Closed01<f32> {
        let mut sum = self.0 + other.0;
        if sum > 1.0 {
            sum = 1.0;
        }
        debug_assert!(sum >= 0.0 && sum <= 1.0);
        Closed01(sum)
    }

    #[inline(always)]
    /// Saturating sub
    pub fn saturating_sub(self, other: Closed01<f32>) -> Closed01<f32> {
        let mut sub = self.0 - other.0;
        if sub < 0.0 {
            sub = 0.0;
        }
        debug_assert!(sub >= 0.0 && sub <= 1.0);
        Closed01(sub)
    }

    #[inline(always)]
    /// Multiplies both numbers.
    pub fn mul(&self, scalar: Closed01<f32>) -> Closed01<f32> {
        let s = self.get() * scalar.get();
        debug_assert!(s >= 0.0 && s <= 1.0);
        Closed01(s)
    }
}

impl Into<f32> for Closed01<f32> {
    fn into(self) -> f32 {
        self.get()
    }
}

impl Rand for Closed01<f32> {
    fn rand<R: Rng>(rng: &mut R) -> Closed01<f32> {
        Closed01::new(RandClosed01::<f32>::rand(rng).0)
    }
}

#[test]
fn test_into() {
    assert_eq!(0.5f32, Closed01::new(0.5).into());
}

#[test]
fn test_minmax() {
    let a = Closed01::new(0.4);
    let b = Closed01::new(0.5);
    assert_eq!(a, a.min(b));
    assert_eq!(a, b.min(a));
    assert_eq!(b, a.max(b));
    assert_eq!(b, b.max(a));
}

#[test]
fn test_saturation() {
    let a = Closed01::new(0.4);
    let b = Closed01::new(0.5);
    let c = Closed01::new(0.6);

    assert_eq!(0.9, a.saturating_add(b).get());
    assert_eq!(1.0, a.saturating_add(c).get());
    assert_eq!(1.0, b.saturating_add(c).get());

    assert_eq!(0.0, a.saturating_sub(b).get());
    assert_eq!(0.0, a.saturating_sub(c).get());
    assert_eq!(0.0, b.saturating_sub(c).get());

    assert!((c.saturating_sub(b).get() - 0.1).abs() < 0.001);
    assert!((c.saturating_sub(a).get() - 0.2).abs() < 0.001);
}
