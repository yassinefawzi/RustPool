pub enum AccessLevel {
	Guest,
	Normal,
	Admin,
}

pub struct User {
	name: String,
	level: AccessLevel,	
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
	User {
		name,
		level: level,
	}
  }
  pub fn send_name(&self) -> Option<&str> {
	match self.level{
		AccessLevel::Guest => None,
		AccessLevel::Admin | AccessLevel::Normal => Some(&self.name),
	}
  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
	match user.send_name() {
		Some(name) => (true, name),
		None => (false, "ERROR: User is guest")
	}
}