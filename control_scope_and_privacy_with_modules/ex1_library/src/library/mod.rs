mod catalog;
mod users;
// mod loans;

pub fn create_book(title: &str, author: &str) -> catalog::Book {
    catalog::Book::new(title, author)
}

pub fn create_user(name: &str) -> (String, u32) {
    let user = users::create_user(name, 1);
    (String::from(user.name()), user.id())
}

// pub fn loan_book(id_book: u32, id_user: u32, )
