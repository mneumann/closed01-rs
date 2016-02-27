extern crate rand;
extern crate nalgebra;

use rand::{Rand, Rng};
use rand::Closed01 as RandClosed01;
use std::fmt::Debug;
use nalgebra::BaseFloat;

/// Encapsulates a floating point number in the range [0, 1] including both endpoints.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Closed01<F>(F) where F: Copy + Clone + Debug + PartialEq + PartialOrd + BaseFloat;

impl<F> Closed01<F>
    where F: Copy + Clone + Debug + PartialEq + PartialOrd + BaseFloat
{
    #[inline(always)]
    pub fn new(f: F) -> Self {
        assert!(f >= F::zero() && f <= F::one());
        Closed01(f)
    }

    #[inline(always)]
    fn new_debug_checked(f: F) -> Self {
        debug_assert!(f >= F::zero() && f <= F::one());
        Closed01(f)
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Closed01::new_debug_checked(F::zero())
    }

    #[inline(always)]
    pub fn center() -> Self {
        Closed01::new_debug_checked(F::one() / (F::one() + F::one()))
    }

    #[inline(always)]
    pub fn one() -> Self {
        Closed01::new_debug_checked(F::one())
    }

    #[inline(always)]
    /// Returns the smaller of the two.
    pub fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    /// Returns the greater of the two.
    pub fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    /// Returns the distance between the two numbers.
    pub fn distance(self, other: Self) -> Self {
        let dist = self.0 - other.0;
        if dist < F::zero() {
            Closed01::new_debug_checked(-dist)
        } else {
            Closed01::new_debug_checked(dist)
        }
    }

    #[inline(always)]
    pub fn get(self) -> F {
        debug_assert!(self.0 >= F::zero() && self.0 <= F::one());
        self.0
    }

    #[inline(always)]
    /// The average of two values.
    pub fn average(self: Self, other: Self) -> Self {
        Closed01::new_debug_checked((self.get() + other.get()) / (F::one() + F::one()))
    }

    #[inline(always)]
    /// Saturating add
    pub fn saturating_add(self, other: Self) -> Self {
        let sum = self.0 + other.0;
        if sum > F::one() {
            Closed01::new_debug_checked(F::one())
        } else {
            Closed01::new_debug_checked(sum)
        }
    }

    #[inline(always)]
    /// Saturating sub
    pub fn saturating_sub(self, other: Self) -> Self {
        let sub = self.0 - other.0;
        if sub < F::zero() {
            Closed01::new_debug_checked(F::zero())
        } else {
            Closed01::new_debug_checked(sub)
        }
    }

    #[inline(always)]
    /// Multiplies both numbers
    pub fn mul(self, scalar: Self) -> Self {
        Closed01::new_debug_checked(self.get() * scalar.get())
    }

    #[inline(always)]
    pub fn approx_eq(self, other: Self, eps: Self) -> bool {
        self.distance(other) < eps
    }

    #[inline(always)]
    /// This scales `self` towards 1.0
    pub fn scale_up(self, other: Self) -> Self {
        Closed01::new_debug_checked(self.0 + (F::one() - self.0) * other.0)
    }

    #[inline(always)]
    /// This scales `self` towards 0.0
    pub fn scale_down(self, other: Self) -> Self {
        Closed01::new_debug_checked(self.0 - self.0 * other.0)
    }

    #[inline(always)]
    /// Invert the number (Mirror at 0.5; 1.0 - number).
    pub fn inv(self) -> Self {
        Closed01::new_debug_checked(F::one() - self.0)
    }

    #[inline(always)]
    /// Round the number to 0.0 or 1.0
    pub fn round(self) -> Self {
        if self < Closed01::center() {
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

impl Into<f64> for Closed01<f32> {
    fn into(self) -> f64 {
        self.get() as f64
    }
}

impl Into<f32> for Closed01<f64> {
    fn into(self) -> f32 {
        self.get() as f32
    }
}

impl Into<f64> for Closed01<f64> {
    fn into(self) -> f64 {
        self.get()
    }
}

impl Rand for Closed01<f32> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Closed01::new(RandClosed01::<f32>::rand(rng).0)
    }
}

impl Rand for Closed01<f64> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Closed01::new(RandClosed01::<f64>::rand(rng).0)
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

#[test]
fn test_f64_minmax() {
    let a = Closed01::new(0.4f64);
    let b = Closed01::new(0.5f64);
    assert_eq!(a, a.min(b));
    assert_eq!(a, b.min(a));
    assert_eq!(b, a.max(b));
    assert_eq!(b, b.max(a));
}
