fn main() {
    let u1 = create_user(String::from("zymmer"), String::from("zym@gmail.com"));
    println!("{} {} {} {}", u1.username, u1.email, u1.active, u1.sign_in_count);

    let mut u1 = u1;
    login(&mut u1);
    login(&mut u1);
    println!("{} {} {} {}", u1.username, u1.email, u1.active, u1.sign_in_count);

    let u1 = change_email(u1, String::from("zynzyn@gmail.com"));
    println!("{} {}", u1.username, u1.email);

    let mut u2 = User{
        username: String::from("juan_alameda"),
        email: String::from("alameda@gmail.com"),
        active: false,
        sign_in_count: 0,
    };
    println!("{}", username_prefix(&u1));
    println!("{}", username_prefix(&u2));

    let color = Color(0, 0, 0);
    let point = Point(1, 6, 0);
    // println!("{}", is_black(point)); Aunque practicamente son el mismo tuple, son diferentes tipos y rust te protege de estos errores
    println!("Is black {}", is_black(color));
    println!("Is origin{}", is_origin(point));

    println!("{} {} {} is valid {}", u1.username, u1.email, u1.active, is_valid_user(&u1));
    println!("{} {} {} is valid {}", u2.username, u2.email, u2.active, is_valid_user(&u2));


    let mut u3 = User{
        username: String::from("admin_juan"),
        email: String::from("superadmin@gmail.com"),
        active: true,
        sign_in_count: 3,
    };
    process_user(&mut u2);
    process_user(&mut u3);
    println!("User:{} Email:{} Active:{} Count:{}", u3.username, u3.email, u3.active, u3.sign_in_count);
    println!("User:{} Email:{} Active:{} Count:{}", u2.username, u2.email, u2.active, u2.sign_in_count);

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

// Ejercicio 4 - Vista segura del username (slices + borrowing)
// Objetivo: Conecctar structs con slices
// Escribe una funcion que devuelva el primer segmento del username antes del _
fn username_prefix(user: &User) -> &str {
    let mut last_index = user.username.len();
    for (i, c) in user.username.char_indices() {
        if c == '_' {
            last_index = i;
            break;
        }
    }

    &user.username[..last_index]
}

// Ejercicio 5 - Tuple structs para tipos seguros
// Objetivo: Entender tigos distintos aueque tengan los mismos datos
// Define dos tuples structs: Color(i32. i32. i32), Point(i32, i32, i32)
// Escribe dos funciones: is_black(color: Color) -> bool, is_origin(point: Point) -> bool
// No mezclar tipos, usa destructuring. Rust te protege de errores semanticos.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn is_black(color: Color) -> bool {
    let Color(r, g, b) = color;
    if r == 0 && g == 0 && b == 0 {
        true
    } else {
        false
    }
}
fn is_origin(point: Point) -> bool {
    let Point(x, y, z) = point;
    if x == 0 && y == 0 && z == 0 {
        true
    } else {
        false
    }
}

// Ejercicio 6 - Validacion de usuario
// Objetivo: Crear funciones que leen sin tomar ownership
// Escribe una funcion que valide un usuario. Username no debe estar vacio, email debe contener @, active debe ser true
// Usa metodos de String que devuelvan booleanos o slices. Recibe un referencia del struct User
fn is_valid_user(user: &User) -> bool {
//     struct User {
//     username: String,
//     email: String,
//     active: bool,
//     sign_in_count: u64,
// }
    if user.username.is_empty() {
        return false;
    }
    if !user.email.contains("@") {
        return false;
    }
    if !user.active {
        return false
    }

    true
}

// Ejercicio 7 - Error de design intencional (razonamiento ownership)
// Considera este struct
// Explica por que Rust no lo permite
// Explica que bug real evitaria
// Justifica por que String es mejor aqui en este capitulo
// struct User1 {
//     username: &str,
//     email: &str,
// }
// 1. Rust no permite usar referencias como tipo de datos en los structs. Solo hacer referencias a Struct completos
// 2. Al evitar referencias, garantizas que la instancia tiene el ownership de los datos, sino fuera asi y si
// el dato original de la referencia es limpiado tendriamos un referencia a un dato inexistente.
// 3. En este caso String es mejor porque permite que cada instncia del struct tenga el ownership de sus datos


// Ejercicio 8 - Editor de usuarios
// Objetivo: Controlar lectura + escritura sin romper borrowing
// Escribe una funcion que:
// 1. Lea el prefijo del username
// 2. So le prefiji es "admin", desactive el usuario
// 3. Si no, incremente sign_in_count
// Firma: fn process_user(user: &mut User)
fn process_user(user: &mut User) {
    let prefix_user = &user.username[..5];
    if prefix_user == "admin" {
        user.active = false;
    } else {
        user.sign_in_count = user.sign_in_count + 1;
    }
}