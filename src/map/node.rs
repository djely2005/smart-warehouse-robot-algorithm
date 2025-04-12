#[derive(Debug, Clone)]
pub struct Node {
    pub id: &'static str,
    pub x: f64,
    pub y: f64,
}

impl Node {
    pub fn new(id: &'static str, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }

    pub fn distance(&self, other: &Node) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
