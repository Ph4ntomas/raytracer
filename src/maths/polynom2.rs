///
/// Second degree polynomial.
///
pub struct Polynom2<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Polynom2<T> {
    ///
    /// Create a new Polynom of degree 2.
    ///
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl Polynom2<f32> {
    ///
    /// Compute the `Polynom2` roots (where it p(x) = 0).
    ///
    /// The formula is as follow:
    /// First, compute the discriminant delta as `delta = b^2 - 4ac`
    ///
    /// If `delta < 0` no roots exists.
    /// If `delta == 0` the polynomial is tangent to 0 at `r = -b / 2a`
    /// if `delta > 0` the polynomial has two roots:
    /// - `r1 = (-b - sqrt(delta)) / 2a`
    /// - `r1 = (-b + sqrt(delta)) / 2a`
    ///
    /// Returned roots are sorted in ascending order.
    ///
    pub fn roots(&self) -> Option<[f32; 2]> {
        let delta = self.b.powi(2) - 4. * self.a * self.c;

        if delta < 0. {
            None
        } else if delta <= 1e-7 {
            // This should be changed to f32::next_up(0.) once stabilized.
            let r = -self.b / 2. * self.a;
            Some([r, f32::NAN])
        } else {
            let sqr = delta.sqrt();

            let r1 = (-self.b - sqr) / (2. * self.a);
            let r2 = (-self.b + sqr) / (2. * self.a);

            if r1 > r2 {
                Some([r2, r1])
            } else {
                Some([r1, r2])
            }
        }
    }
}
