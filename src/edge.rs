type Point = geo::Coordinate<f64>;

#[derive(Debug, Clone)]
pub struct Edge {
    pub current: Point,
    pub next: Point,
}

#[derive(Debug, Clone)]
pub enum EdgeError {
    VerticesOverlap,
}

impl Edge {
    pub fn new(current: &Point, next: &Point) -> Self {
        Self {
            current: *current,
            next: *next,
        }
    }

    pub fn new_with_offset(current: &Point, next: &Point, dx: f64, dy: f64) -> Self {
        Self {
            current: (current.x + dx, current.y + dy).into(),
            next: (next.x + dx, next.y + dy).into(),
        }
    }

    pub fn inwards_normal(&self) -> Result<Point, EdgeError> {
        let dx = self.next.x - self.current.x;
        let dy = self.next.y - self.current.y;
        let edge_length = (dx * dx + dy * dy).sqrt();
        let x = -dy / edge_length;
        let y = dx / edge_length;

        if x.is_finite() && y.is_finite() {
            Ok((x, y).into())
        } else {
            Err(EdgeError::VerticesOverlap)
        }
    }

    pub fn outwards_normal(&self) -> Result<Point, EdgeError> {
        let inwards = self.inwards_normal()?;
        Ok((-inwards.x, -inwards.y).into())
    }

    pub fn with_offset(&self, dx: f64, dy: f64) -> Self {
        Self::new_with_offset(&self.current, &self.next, dx, dy)
    }

    pub fn inverse_with_offset(&self, dx: f64, dy: f64) -> Self {
        Self::new_with_offset(&self.next, &self.current, dx, dy)
    }

    pub fn inverse(&self) -> Self {
        Self::new(&self.next, &self.current)
    }
}
