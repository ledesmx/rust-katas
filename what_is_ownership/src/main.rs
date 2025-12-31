fn main() {
    entender_ownership();
}

// 🧩 PARTE 1 — Ejercicios iniciales (entender ownership)
// 1️⃣ Vida útil de un mensaje

// 📌 Problema
// Un programa muestra un mensaje temporal dentro de un bloque.

// Crea un String

// Imprímelo dentro de un bloque { }

// Intenta usarlo fuera del bloque

// 📌 Objetivo mental
// Identificar cuándo una variable deja de ser válida.

// 🤔 Piensa
// ¿Por qué el compilador sabe exactamente dónde liberar la memoria?
fn entender_ownership() {
    let s = String::from("I love Rust");
    {
        println!("{s}");
    }
    println!("{s}");

    {
        let z = s;
        println!("{z}");
    }
    // println!("{z}"); // z out of scope
    // println!("{s}"); // borrow of moved
}
