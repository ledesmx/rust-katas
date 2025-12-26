fn main() {
    // 1. Orden de ejecución
    aux1();
    aux2();

    // 2. Parámetros obligatorios
    imprimir(3, 't');
    // imprimir('n', 4); incorrect order

    // 3. Función silenciosa
    let any = nothing(32);
    println!("{:?}", any);
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

// 2. Parámetros obligatorios

// Crea una función que reciba:

// un i32

// un char

// Imprime ambos valores en una sola línea.

// Luego:

// intenta llamarla pasando los argumentos en orden incorrecto.

// Pregunta clave:
// ¿Qué tan estricta es Rust con el orden y tipo de los parámetros?
fn imprimir(val1: i32, val2: char) {
    println!("Valor_1: {} - Valor_2: {}", val1, val2);
}

// 3. Función silenciosa

// Escribe una función que:

// reciba un número

// no imprima nada

// no devuelva nada

// Luego:

// asigna su resultado a una variable

// imprime esa variable

// Pregunta clave:
// ¿Qué tipo devuelve realmente una función “vacía”?
fn nothing(_n: i32) {}
