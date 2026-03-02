// Ejercicio 1 - Sistema de biblioteca (Organizacion por dominios)
// Objetivo: Organizar un sistema de biblioteca usando modulos
// Estructura esperada del crate:
// - Desde main, pub mod library
// - Dentro de library, mod catalog, mod users, mod loans
// Modulo catalog
// - Debe contener pub struct Book: title:String, author: String, available: bool
// - Metodo mark_unavailable(&mut self)
// - Metodo mark_available(&mut self)
// Decide que debe ser publico y que no
// Modulo  users
// - Debe contener struc User: name: String, id: u32
// Modulo loans
// - Debe contener enum LoanStatus: Active, Returned
// - pub struct Loan: book: Book, user: User, status: LoanStatus
// - metodo return_book(&mut self)
// Problema a resolver:
// - Crear un libro
// - Crear un usuario
// - Crear un prestamo
// - Marcar el libro como no disponible
// - Devolver el libro
// Practicar organizacion modular real, que necesita pub y que no, usar use, evitar hacer todo pub

mod library;

fn main() {
    let book = library::catalog::Book::new("The Name of The Wind", "Patrick Rofus");
    println!("Hello, world!");
}
