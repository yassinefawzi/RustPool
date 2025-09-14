#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    let jacket = match formality_level {
        None => Jacket::Flowers,
        Some(level) if level > 0 => Jacket::White,
        Some(_) => Jacket::Black,
    };

    let hat = match (formality_level, invitation_message) {
        (None, Err(_)) => Hat::Baseball,     // Special case
        (_, Ok(_)) => Hat::Fedora,
        _ => Hat::Snapback,
    };

    Outfit { jacket, hat }
}
