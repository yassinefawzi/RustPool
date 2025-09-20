#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
		List {
			head: None,
		}
    }

    pub fn push(&mut self, value: T) {
		let node = Box::new(Node {
			value,
			next: self.head.take(),
		});
		self.head = Some(node);
    }

    pub fn pop(&mut self) {
		if let Some(node) = self.head.take() {
			self.head = node.next;
		}
    }

    pub fn len(&self) -> usize {
		let mut ret = 0;
		let mut current = &self.head;
		while let Some(node) = current {
			ret += 1;
			current = &node.next;
		}
		ret
    }
}