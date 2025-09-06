#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        return Self {
            center: Point(x, y),
            radius,
        };
    }

	pub fn area(&self) -> f64 {
		return std::f64::consts::PI * self.radius.powi(2);
	}

	pub fn diameter(&self) -> f64 {
		return self.radius * 2.0;
	}

	pub fn intersect(&self, other: Circle) -> bool {
		let distance = self.center.distance(other.center);
		return distance <= (self.radius + other.radius);
	}
}
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, point: Point) -> f64 {
        ((self.0 - point.0).powi(2) + (self.1 - point.1).powi(2)).sqrt()
    }
}
