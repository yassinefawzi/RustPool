use std::cell::{ RefCell, Cell };

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    pub fn new() -> Blog {
        Blog {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_article(&self, body: String) -> (usize, Article) {
        let id = self.new_id();
        self.states.borrow_mut().push(false);
        (id, Article::new(id, body, self))
    }
    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        self.drops.set(self.drops.get() + 1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    pub id: usize,
    pub body: String,
    pub blog: &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Article {
        Article {
            id,
            body,
            blog,
        }
    }
    pub fn discard(self) {}
}

impl<'a> Drop for Article<'a> {
    fn drop(&mut self) {
        self.blog.add_drop(self.id);
    }
}
