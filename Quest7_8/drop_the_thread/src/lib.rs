use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
			drops: Cell::new(0),
			states: RefCell::new(Vec::new()),
		}
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
		let pid = self.thread_len();
		self.states.borrow_mut().push(false);
		(pid, Thread::new(pid, c, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        let mut check = self.states.borrow_mut();
		if check[id]{
			panic!("{} is already dropped", id);
		}
		check[id] = true;
		self.drops.set(self.drops.get() + 1)
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
	pub pid: usize,
	pub cmd: String,
	pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread {
			pid: p,
			cmd: c,
			parent: t,
		}
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
	fn drop(&mut self) {
		self.parent.drop_thread(self.pid);
	}
}
