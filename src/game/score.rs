#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score {
    black: f64,
    white: f64,
}

impl Score {
    pub fn new(black: f64, white: f64) -> Self {
        Self { black, white }
    }

    pub fn black(&self) -> f64 {
        self.black
    }

    pub fn white(&self) -> f64 {
        self.white
    }
}
