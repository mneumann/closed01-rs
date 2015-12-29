extern crate rand;

use rand::{Rand, Rng};
use rand::Closed01 as RandClosed01;

/// Encapsulates a floating point number in the range [0, 1] including both endpoints.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Closed01<F>(F);

impl Closed01<f32> {
    #[inline(always)]
    pub fn new(f: f32) -> Closed01<f32> {
        assert!(f >= 0.0 && f <= 1.0);
        Closed01(f)
    }

    #[inline(always)]
    fn new_debug_checked(f: f32) -> Closed01<f32> {
        debug_assert!(f >= 0.0 && f <= 1.0);
        Closed01(f)
    }

    #[inline(always)]
    pub fn zero() -> Closed01<f32> {
        Closed01::new_debug_checked(0.0)
    }

    #[inline(always)]
    pub fn center() -> Closed01<f32> {
        Closed01::new_debug_checked(0.5)
    }

    #[inline(always)]
    pub fn one() -> Closed01<f32> {
        Closed01::new_debug_checked(1.0)
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
        Closed01::new_debug_checked((self.0 - other.0).abs())
    }

    #[inline(always)]
    pub fn get(self) -> f32 {
        debug_assert!(self.0 >= 0.0 && self.0 <= 1.0);
        self.0
    }

    #[inline(always)]
    /// The average of two values.
    pub fn average(self: Closed01<f32>, other: Closed01<f32>) -> Closed01<f32> {
        Closed01::new_debug_checked((self.get() + other.get()) / 2.0)
    }

    #[inline(always)]
    /// Saturating add
    pub fn saturating_add(self, other: Closed01<f32>) -> Closed01<f32> {
        let mut sum = self.0 + other.0;
        if sum > 1.0 {
            sum = 1.0;
        }
        Closed01::new_debug_checked(sum)
    }

    #[inline(always)]
    /// Saturating sub
    pub fn saturating_sub(self, other: Closed01<f32>) -> Closed01<f32> {
        let mut sub = self.0 - other.0;
        if sub < 0.0 {
            sub = 0.0;
        }
        Closed01::new_debug_checked(sub)
    }

    #[inline(always)]
    /// Multiplies both numbers
    pub fn mul(self, scalar: Closed01<f32>) -> Closed01<f32> {
        Closed01::new_debug_checked(self.get() * scalar.get())
    }

    #[inline(always)]
    pub fn approx_eq(self, other: Closed01<f32>, eps: Closed01<f32>) -> bool {
        self.distance(other) < eps
    }

    #[inline(always)]
    /// This scales `self` towards 1.0
    pub fn scale_up(self, other: Closed01<f32>) -> Closed01<f32> {
        Closed01::new_debug_checked(self.0 + (1.0 - self.0) * other.0)
    }

    #[inline(always)]
    /// This scales `self` towards 0.0
    pub fn scale_down(self, other: Closed01<f32>) -> Closed01<f32> {
        Closed01::new_debug_checked(self.0 - self.0 * other.0)
    }

    #[inline(always)]
    /// Invert the number (Mirror at 0.5; 1.0 - number).
    pub fn inv(self) -> Closed01<f32> {
        Closed01::new_debug_checked(1.0 - self.0)
    }

    #[inline(always)]
    /// Round the number to 0.0 or 1.0
    pub fn round(self) -> Closed01<f32> {
        if self.0 < 0.5 {
            Closed01::zero()
        } else {
            Closed01::one()
        }
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

    let eps = Closed01::new(0.001);
    assert!(c.saturating_sub(b).approx_eq(Closed01::new(0.1), eps));
    assert!(c.saturating_sub(a).approx_eq(Closed01::new(0.2), eps));
}

#[test]
fn test_scale_up() {
    let a = Closed01::new(0.0);
    let b = Closed01::new(1.0);
    let c = Closed01::new(0.5);

    assert_eq!(b, a.scale_up(b));
    assert_eq!(c, a.scale_up(c));

    assert_eq!(c, c.scale_up(a));
    assert_eq!(b, c.scale_up(b));
}

#[test]
fn test_scale_down() {
    let a = Closed01::new(0.0);
    let b = Closed01::new(1.0);
    let c = Closed01::new(0.5);

    assert_eq!(a, c.scale_down(b));
    assert_eq!(b, b.scale_down(a));
    assert_eq!(c, b.scale_down(c));
}

#[test]
fn test_invert() {
    let a = Closed01::new(0.0);
    let b = Closed01::new(1.0);
    let c = Closed01::new(0.5);

    assert_eq!(a, a.inv().inv());
    assert_eq!(b, b.inv().inv());
    assert_eq!(c, c.inv().inv());


    assert_eq!(b, a.inv());
    assert_eq!(a, b.inv());
    assert_eq!(c, c.inv());
}

#[test]
fn test_round() {
    assert_eq!(Closed01::zero(), Closed01::new(0.0).round());
    assert_eq!(Closed01::zero(), Closed01::new(0.1).round());
    assert_eq!(Closed01::zero(), Closed01::new(0.2).round());
    assert_eq!(Closed01::zero(), Closed01::new(0.3).round());
    assert_eq!(Closed01::zero(), Closed01::new(0.4).round());
    assert_eq!(Closed01::zero(), Closed01::new(0.49999).round());
    assert_eq!(Closed01::one(), Closed01::new(0.5).round());
    assert_eq!(Closed01::one(), Closed01::new(0.6).round());
    assert_eq!(Closed01::one(), Closed01::new(0.8).round());
    assert_eq!(Closed01::one(), Closed01::new(0.9).round());
    assert_eq!(Closed01::one(), Closed01::new(1.0).round());
}
