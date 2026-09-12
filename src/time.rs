#[cfg(target_arch = "wasm32")]
pub(crate) struct Timer(f64);

#[cfg(target_arch = "wasm32")]
impl Timer {
    pub(crate) fn now() -> Self {
        Self(web_sys::window().unwrap().performance().unwrap().now())
    }

    pub(crate) fn elapsed_ms(&self) -> f64 {
        web_sys::window().unwrap().performance().unwrap().now() - self.0
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct Timer(std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Timer {
    pub(crate) fn now() -> Self {
        Self(std::time::Instant::now())
    }

    pub(crate) fn elapsed_ms(&self) -> f64 {
        self.0.elapsed().as_secs_f64() * 1000.0
    }
}
