fn main() {
    let u1 = create_user(String::from("zymmer"), String::from("zym@gmail.com"));
    println!("{} {} {} {}", u1.username, u1.email, u1.active, u1.sign_in_count);

    let mut u1 = u1;
    login(&mut u1);
    login(&mut u1);
    println!("{} {} {} {}", u1.username, u1.email, u1.active, u1.sign_in_count);

    let u1 = change_email(u1, String::from("zynzyn@gmail.com"));
    println!("{} {}", u1.username, u1.email);
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

// Ejercicio 2 - Contador de inicios de sesion
// Objetivo: Practicar mutabilidad en structs, paso por referencioa mutable
// Escribe una funcion que incremente el contador de inicio de sesion de un usuario, usa  &mut User
fn login(user: &mut User) {
    user.sign_in_count = user.sign_in_count + 1;
}

// Ejercicio 3 - Clon parcial con struct update syntax
// Objetivo: Entender que se mueve y que no en structs
// Dado un usuario existente, crea uno nuevo que tenga el mismo username, active y sign_in_count. Pero otro email.
fn change_email(user: User, new_email: String) -> User {
    User{
        email: new_email,
        ..user
    }
}