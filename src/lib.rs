/// Encapsulates a floating point number in the range [0, 1] including both endpoints.
#[derive(Copy, Clone, Debug)]
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
    pub fn one() -> Closed01<f32> {
        Closed01(1.0)
    }

    #[inline(always)]
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
    pub fn average(a: Closed01<f32>, b: Closed01<f32>) -> Closed01<f32> {
        let avg = (a.get() + b.get()) / 2.0;
        debug_assert!(avg >= 0.0 && avg <= 1.0);
        Closed01(avg)
    }

    #[inline(always)]
    pub fn scale(&self, scalar: Closed01<f32>) -> Closed01<f32> {
        let s = self.get() * scalar.get();
        debug_assert!(s >= 0.0 && s <= 1.0);
        Closed01(s)
    }
}
