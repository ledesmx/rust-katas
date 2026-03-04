// Modulo catalog
// - Debe contener pub struct Book: title:String, author: String, available: bool
// - Metodo mark_unavailable(&mut self)
// - Metodo mark_available(&mut self)
#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    available: bool,
}
impl Book {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: String::from(title),
            author: String::from(author),
            available: true,
        }
    }
    fn mark_unavailable(&mut self) {
        self.available = false;
    }
    fn mark_available(&mut self) {
        self.available = true;
    }
}