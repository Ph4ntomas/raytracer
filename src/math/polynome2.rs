#[derive(Default, Copy, Clone, Debug)]
pub struct Polynome2<T> {
    pub a:T,
    pub b:T,
    pub c:T,
}

impl Polynome2<f32> {
    pub fn root(&self) -> Option<[f32;2]> {
        let delta = self.b * self.b - 4. * self.a * self.c;

        if delta < 0. { return None; }
        else if delta == 0. {
            let r = -self.b / 2. * self.a;
            return Some([r, r]);
        }

        let r1 = (-self.b - delta.sqrt()) / (2. * self.a);
        let r2 = (-self.b - delta.sqrt()) / (2. * self.a);

        if r1 > r2 { return Some([r2, r1]); }

        Some([r1, r2])
    }
}

