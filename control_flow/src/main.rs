fn main() {
    // 1. Sistema de acceso (if básico)
    println!("{}", mayor_de_edad(33));
    println!("{}", mayor_de_edad(13));

    // 2. Clasificador de número (else if)
    println!("{}", ifnum(4));
    println!("{}", ifnum(-5));
    println!("{}", ifnum(0));

    // 🧩 BLOQUE 2: loop y control explícito
    println!("{}", doble_incremento(10));
    println!("{}", doble_incremento(8));
    println!("{}", doble_incremento(-8));
}
// 🧭 Ejercicios — Control Flow (if, loops)
// 🧩 BLOQUE 1: if como toma de decisiones (problemas reales)
// 1. Sistema de acceso (if básico)

// 📌 Problema
// Un sistema debe decidir si una persona puede entrar a un evento.

// Reglas:

// Si la edad es menor a 18 → acceso denegado

// Si es 18 o más → acceso permitido

// 📌 Requisitos

// Usa if y else

// La decisión debe estar en una función

// main solo imprime el resultado

// 🤔 Piensa
// ¿Por qué es mejor que la función devuelva el resultado en vez de imprimirlo?
fn mayor_de_edad(edad: i32) -> String {
    if edad > 17 {
        String::from("Acceso aprobado")
    } else {
        String::from("Acceso denegado")
    }
}

// 2. Clasificador de número (else if)

// 📌 Problema
// Dado un número entero, el programa debe indicar si:

// es negativo

// es cero

// es positivo

// 📌 Requisitos

// Usa if / else if / else

// La función debe devolver un &str

// 🤔 Piensa
// ¿Por qué todas las ramas deben devolver el mismo tipo?

fn ifnum(n: i32) -> String {
    if n > 0 {
        String::from("Es positivo")
    } else if n < 0 {
        String::from("Es negativo")
    } else {
        String::from("Es cero")
    }
}

// 🧩 BLOQUE 2: loop y control explícito
// 4. Contador con condición de salida

// 📌 Problema
// Simula un contador que empieza en 0 y se incrementa hasta llegar a 10.

// Cuando llegue a 10:

// detén el loop

// devuelve el valor final multiplicado por 2

// 📌 Requisitos

// Usa loop

// Usa break devolviendo un valor

// El resultado debe asignarse a una variable

// 🤔 Piensa
// ¿Por qué loop puede devolver un valor?

fn doble_incremento(limit: i32) -> i32 {
    let mut counter = 0;
    let mut sum = 0;
    let result = loop {
        if counter >= limit {
            break sum;
        }
        sum = (sum + counter) * 2;
        counter += 1;
    };
    result
}
