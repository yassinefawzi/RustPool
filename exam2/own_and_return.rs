pub struct Film {
    pub name: String,
}

// Consumes the Film and returns its name
pub fn take_film_name(film: Film) -> String {
    film.name
}

// Borrows the Film and returns a clone of the name
pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}
