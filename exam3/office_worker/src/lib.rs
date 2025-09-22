#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
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

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split(',').collect();
        let name = parts[0].to_string();
        let age = parts[1].parse::<u32>().unwrap();
        let role: WorkerRole = parts[2].into();

        OfficeWorker { name, age, role }
    }
}