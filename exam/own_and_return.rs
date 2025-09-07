pub struct Film {
    pub name: String,
}

pub fn take_film_name(film: Film) -> String {
	return film.name;
}

pub fn read_film_name(film: &Film) -> String {
	return film.name.clone();
}
