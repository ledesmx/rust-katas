fn main() {
    // 🧩 PARTE 1 — Ejercicios iniciales (entender ownership)
    entender_ownership();

    // 🔥 RETO — Procesar un mensaje correctamente
    let mensaje = crear_prioridad(6);
    let mensaje = agregar_tipo(String::from("LOG"), mensaje);
    let mensaje = agregar_mensaje(String::from(""), mensaje);
    println!("{}", mensaje);
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

// 🔥 RETO — Procesar un mensaje correctamente
// 🧩 Problema

// Un programa recibe un mensaje de texto y debe usarlo una sola vez.

// El mensaje:

// se crea

// se valida

// se muestra

// Después de mostrarse, no debe poder volver a usarse.

// ✅ Qué debes validar

// El mensaje es válido si tiene más de 5 caracteres.

// 📌 Qué debes hacer

// Crear el mensaje (String)

// Pasarlo a una función que lo valide

// Si es válido, pasarlo a otra función que lo muestre

// Si no es válido, terminar el programa

// 📜 Reglas

// Cada paso es una función

// No copies ni clones el mensaje

// El mensaje debe moverse entre funciones

// El compilador debe impedir usos incorrectos

// 🎯 Objetivo

// Usar ownership para evitar:

// usar mensajes sin validar

// usar mensajes más de una vez

// 🤔 Pregunta clave

// ¿Por qué después de mostrar el mensaje ya no puede existir?

fn crear_prioridad(prioridad: i32) -> String {
    let mensaje = if prioridad > 0 && prioridad < 10 {
        format!("{} - ", prioridad)
    } else {
        String::from("N - ")
    };
    mensaje
}

fn agregar_tipo(tipo: String, prev: String) -> String {
    let mensaje = if tipo == "TIP" || tipo == "LOG" {
        prev + &tipo
    } else {
        prev + "000"
    };
    mensaje
}
fn agregar_mensaje(men: String, prev: String) -> String {
    let mensaje: String = if men.is_empty() {
        prev + " - ~No Message~"
    } else if men.len() > 11 {
        prev + " - ~Too long..~"
    } else {
        prev + " - " + &men
    };
    mensaje
}
