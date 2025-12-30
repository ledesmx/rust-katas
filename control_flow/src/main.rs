use rand;

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

    // 5. Búsqueda simple (loop + if)
    println!("{}", has_number(8, [4, 8, 5, 8, 0]));
    println!("{}", has_number(24, [4, 8, 5, 8, 0]));
    println!("{}", has_number(0, [4, 8, 5, 8, 0]));

    // 🧩 Ejercicio — Simulación de control de intentos
    adivinar_numeros([4, 5, 1]);

    // 6. Cuenta regresiva
    cuenta_regresiva(7);

    // 8. Suma de valores (for)
    println!("Suma array {}", suma_array([6, 6, 6, 6]));
    println!("Suma array {}", suma_array([1, 2, 3, 4]));
    println!("Suma array {}", suma_array([1, 20, -16, -5]));
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

// 5. Búsqueda simple (loop + if)

// 📌 Problema
// Tienes un número secreto.
// El programa prueba números empezando desde 1 hasta encontrarlo.

// 📌 Requisitos

// Usa loop

// Usa if para verificar

// Cuando lo encuentres, imprime cuántos intentos tomó

// 🤔 Piensa
// ¿En qué se parece esto al guessing game?
fn has_number(n: i32, array: [i32; 5]) -> bool {
    let mut i = 0;
    loop {
        if i >= array.len() {
            break;
        }
        if n == array[i] {
            return true;
        } else {
            i += 1;
        }
    }
    false
}

// 🧩 Ejercicio — Simulación de control de intentos
// 🧠 Problema

// Estás programando un sistema que simula intentos de acceso por día.

// Reglas:

// El sistema revisa varios días

// Cada día tiene hasta 3 intentos de acceso

// Si en un día ocurre un acceso exitoso, se dejan de evaluar los intentos de ese día

// El sistema continúa con el siguiente día

// 📌 Comportamiento esperado

// El loop externo representa los días

// El loop interno representa los intentos

// Cuando ocurre un acceso exitoso:

// se usa break sin etiqueta

// solo se rompe el loop interno

// El programa NO debe terminar completamente, solo pasar al siguiente día
fn adivinar_numeros(numeros: [i32; 3]) {
    let mut dia = 0;
    loop {
        if dia < 3 {
            let mut intentos = 0;
            loop {
                if intentos >= 3 {
                    println!("Dia {} No adivino {}", dia + 1, numeros[dia]);
                    break;
                }
                let n = rand::random_range(0..6); // del 0 al 5
                if numeros[dia] == n {
                    println!("Dia {} Adivino {} en intento {}", dia + 1, n, intentos + 1);
                    break;
                }
                intentos += 1;
            }
        } else {
            break;
        }
        dia += 1;
    }
}

// 🧩 BLOQUE 3: while como condición natural
// 6. Cuenta regresiva

// 📌 Problema
// Imprime una cuenta regresiva desde un número dado hasta 1, y luego imprime "DESPEGUE".

// 📌 Requisitos

// Usa while

// No uses loop

// 🤔 Piensa
// ¿Por qué while hace este código más claro que loop?

fn cuenta_regresiva(mut i: i32) {
    println!("Comenzando...");
    while i > 0 {
        println!("{i}");
        i -= 1;
    }
    println!("DESPEGUE");
}

// 🧩 BLOQUE 4: Arrays + loops (problemas prácticos)
// 8. Suma de valores (for)

// 📌 Problema
// Dado un array de números:

// calcula la suma total

// 📌 Requisitos

// Usa for

// No uses índices manuales

// 🤔 Piensa
// ¿Por qué este método es más seguro que usar while con índices?
fn suma_array(valores: [i32; 4]) -> i32 {
    let mut sum = 0;
    for v in valores {
        sum += v;
    }
    sum
}
