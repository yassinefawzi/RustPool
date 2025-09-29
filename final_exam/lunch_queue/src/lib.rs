#[derive(Debug, Clone)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: i32,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let p = Box::new(
			Person{
				name,
				discount,
				next_person: self.node.take(),
			}
		);
		self.node = Some(p);
    }

    pub fn invert_queue(&mut self) {
        let mut prev: Link = None;
		let mut current = self.node.take();
		while let Some(mut p) = current {
			let next = p.next_person.take();
			p.next_person = prev;
			prev = Some(p);
			current = next;
		}
		self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        if self.node.is_none() {
			return None;
		}
		if self.node.as_ref().unwrap().next_person.is_none() {
			let p = self.node.take().unwrap();
			return Some((p.name, p.discount));
		}
		let mut current = self.node.as_mut().unwrap();
		while current.next_person.as_ref().unwrap().next_person.is_some() {
			current = current.next_person.as_mut().unwrap();
		}
		let last = current.next_person.take().unwrap();
		Some((last.name, last.discount))
	}

    pub fn search(&self, name: &str) -> Option<(&String, &i32)> {
        let mut current = &self.node;
		while let Some(person) = current {
			if person.name == name {
				return Some((&person.name, &person.discount))
			}
			current = &person.next_person;
		}
		None
    }
}