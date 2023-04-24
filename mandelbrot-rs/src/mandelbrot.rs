pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("{} + {}i", self.re, self.im)
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self { re: 0., im: 0. }
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn id() -> Self {
        Self::default()
    }

    pub fn mandelbrot_iter(&self, c: &Self) -> Self {
        Self {
            re: (self.re - self.im) * (self.re + self.im) + c.re,
            im: 2. * self.re * self.im + c.im,
        }
    }

    pub fn abs_value_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn escape_count(&self, z_0: Self, bound: f64, max_iters: isize) -> isize {
        if z_0.abs_value_sq() > bound {
            return 0;
        }
        let bound_sq = bound.powf(2.);
        let mut z_iter = z_0;
        for iter in 1..=max_iters {
            z_iter = z_iter.mandelbrot_iter(self);
            if z_iter.abs_value_sq() > bound_sq {
                return iter;
            }
        }
        return max_iters;
    }
}
