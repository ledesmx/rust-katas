fn main() {
    // 1. Sistema de acceso (if básico)
    println!("{}", mayor_de_edad(33));
    println!("{}", mayor_de_edad(13));
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
