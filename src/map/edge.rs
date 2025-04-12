#[derive(Debug, Clone)]
pub struct Edge {
    pub from: &'static str,
    pub to: &'static str,
    pub distance: f64,
}

impl Edge {
    pub fn new(from: &'static str, to: &'static str, distance: f64) -> Self {
        Self { from, to, distance }
    }
}
