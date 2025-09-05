pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
	return student.0;
}

pub fn first_name(student: &Student) -> &str {
	return &student.1;
}

pub fn last_name(student: &Student) -> &str {
	return &student.2;
}
