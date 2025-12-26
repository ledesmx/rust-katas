fn main() {
    aux1();
    aux2();
}
// 1. Orden de ejecución

// Escribe un programa con:

// main

// dos funciones auxiliares

// Cada función imprime un mensaje distinto.

// 👉 Llama a las funciones desde main en un orden específico.

// Pregunta clave:
// ¿El orden de definición de las funciones afecta el orden de ejecución?
fn aux1() {
    println!("Mensaje uno");
}
fn aux2() {
    println!("Mensaje dos");
}
