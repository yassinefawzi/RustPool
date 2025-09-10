/*
Red		Color::rgba(255, 0, 0, 255)
Green	Color::rgba(0, 255, 0, 255)
Blue	Color::rgba(0, 0, 255, 255)
Yellow	Color::rgba(255, 255, 0, 255)
Cyan	Color::rgba(0, 255, 255, 255)
Magenta	Color::rgba(255, 0, 255, 255)
Black	Color::rgba(0, 0, 0, 255)
White	Color::rgba(255, 255, 255, 255)
*/

use raster::Color;
use raster::Image;

pub trait Drawable {
	fn draw(&self, image: &mut Image);
	fn color(&self) -> Color;
}

pub trait Displayable {
	fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Self {
		Self {x, y}
	}

	pub fn random(width: i32, height: i32) -> Self {
		Self {
			x: rand::thread_rng().gen_range(0..width),
			y: rand::thread_rng().gen_range(0..height),
		}
	}
}

impl Drawable for Point {
	fn draw(&self, image: &mut Image) {
		image.display(self.x, self.y, self.color());
	}
	fn color(&self) -> Color {
		Color::rgba(255, 0, 0, 255)
	}
}


// Line
pub struct Line {
	pub start: Point,
	pub end: Point,
}

impl Line {
	pub fn new(start: &Point, end: &Point) -> Self {
		Self {
			(start: Point::new(start.x, start.y), end: Point::new(end.x, end.y))
		}
	}

	pub fn random(width: i32, height: i32) -> Self {
		Self {
			start: Point::random(width, height),
			end: Point::random(width, height),
		}

	}
}

impl Drawable for Line {
	fn draw(&self, image: &mut Image) {
		let mut x = self.start.x;
		let mut y = self.start.y;
		let distancex = (self.end.x - x).abs();
		let distancey = -(self.end.y - y).abs();
		let stepx = if x < self.end.x {1} else {-1};
		let stepy = if y < self.end.y {1} else {-1};
		let mut distance = distancex + distancey;

		loop {
			image.display(x, y, self.color());
			if x == self.end.x && y == self.end.y {
				break;
			}

			let distance2 = 2 * distancey;
			if distance2 >= distancey {
				distance += distancey;
				x += stepx;
			}
			if distance2 <= distancex {
				distance += distancex;
				y += stepy;
			}
		}
	}
	fn color(&self) -> Color {
		color: Color::rgba(255, 255, 255, 255)
	}
}

// Rectangle
pub struct Rectangle {
	pub p1: Point,
	pub p2: Point,
}

impl Rectangle {
	pub fn new(p1: &Point, p2: &Point) -> Self {
		Self {
			p1: Point::new(p1.x, p1.y),
			p2: Point::new(p2.x, p2.y),
		}
	}
}

impl Drawable for Rectangle {
	fn draw(&self, image: &mut Image) {
		let x_min = self.p1.x.min(self.p2.x);
		let x_max = self.p1.x.max(self.p2.x);
		let y_min = self.p1.y.min(self.p2.y);
		let y_max = self.p1.y.max(self.p2.y);

		for x in x_min..=x_max {
			image.display(x, y_min, self.color());
			image.display(x, y_max, self.color());
		}
		for y in y_min..=y_max {
			image.display(x_min, y, self.color());
			image.display(x_max, y, self.color());
		}
	}

	fn color(&self, image: &mut Image) -> Color {
		Color::rgba(0, 255, 255, 255)
	}
}

// Triangle
pub struct Triangle {
	pub p1: Point,
	pub p2: Point,
	pub p3: Point,
}

impl Triangle {
	pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
		Self {
			p1: Point::new(p1.x, p1.y),
			p2: Point::new(p2.x, p2.y),
			p3: Point::new(p3.x, p3.y),
		}
	}
}

impl Drawable for Triangle {
	fn draw(&self, image: &mut Image) {
		Line::new(&self.p1, &self.p2).draw(image);
		Line::new(&self.p2, &self.p3).draw(image);
		Line::new(&self.p3, &self.p1).draw(image);
	}

	fn color(&self) -> Color {
        Color::rgba(0, 255, 0, 255)
    }
}

// Circle
pub struct Circle {
	pub center: Point,
    pub radius: i32,
}


impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self { center: Point::new(center.x, center.y), radius }
    }

	pub fn random(width: i32, height: i32) -> Self {
		Self {
			center: Point::random(width, height),
			radius: rand::thread_rng().gen_range(10..40),
		}
	}
}