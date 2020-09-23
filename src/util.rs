use std::sync::Arc;

pub trait IntoArc: Sized {
    fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}
