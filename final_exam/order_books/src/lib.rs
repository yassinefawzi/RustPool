pub mod library {
    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: u64,
        }
    }

    pub mod writers {
        use crate::library::books::Book;
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
}

use library::writers::*;
pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
}
