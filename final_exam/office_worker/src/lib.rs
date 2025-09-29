#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let h: Vec<_> = s.split(',').collect();
        let name = h[0].to_string();
        let age = h[1].parse::<u32>().unwrap();
        let role = WorkerRole::from(h[2]);
        OfficeWorker { name, age, role }
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => unreachable!(),
        }
    }
}
