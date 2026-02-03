fn main() {
    let u1 = create_user(String::from("zymmer"), String::from("zym@gmail.com"));
    println!("{} {} {} {}", u1.username, u1.email, u1.active, u1.sign_in_count);
}

// Ejercicio 1 - Constructor seguro de usuario
// Objetivo: Practicar structs, ownership, funciones que retornan strutch
// Define un struct User con los campos: active, username, email, sign_in_count
// Escribe una funcion que reciba un username y email, cree un usuario activo, inicialice sign_in_count en 1
// Usa field init shorthand
// No clones strings
// Prueba mental: Que pasa con username y email despues de llamar la funcion?
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}