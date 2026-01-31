fn main() {
    let text1 = String::from("Bad Bunny");
    let text2 = &String::from("Losing my religion");
    let text3 = "Can you read my mind?";
    println!("first {}", first_word(&text1));
    println!("first {}", first_word(text2));
    println!("first {}", first_word(text3));

    println!("last {}", last_word(&text1));
    println!("last {}", last_word(&text2));
    println!("last {}", last_word(&text3));

    println!("prefix {}", prefix("LOG: The message"));
    println!("prefix {}", prefix("Just the message"));
}
// 🟢 Ejercicio 1 — Primera palabra (slice básico)

// Objetivo: Practicar &str como retorno.

// Enunciado

// Implementa una función que devuelva la primera palabra de un texto (separado por espacios).

// Firma obligatoria
// fn first_word(s: &str) -> &str

// Reglas

// No retornes índices

// No clones

// Usa slices (&s[..i], &s[..])
fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }    
    }
    s
}

// 🟢 Ejercicio 2 — Última palabra

// Objetivo: Pensar slices desde el final.

// Enunciado

// Devuelve la última palabra de una cadena.

// Firma
// fn last_word(s: &str) -> &str

// Ejemplo
// let s = "rust es genial";
// assert_eq!(last_word(s), "genial");

// Restricciones

// Recorre los bytes

// Usa slices, no split

// 👉 Aquí practicas rangos tipo [i+1..].
fn last_word(s: &str) -> &str {
    for (i, c) in s.char_indices().rev() {
        if c == ' ' {
            return &s[i+1..];
        }
        // println!("{} {}", i, c);
    }
    s
}

// 🟡 Ejercicio 3 — Prefijo válido

// Objetivo: Usar slices como vistas parciales.

// Enunciado

// Escribe una función que devuelva:

// El prefijo antes del primer ':'

// O todo el texto si no hay ':'

// Firma
// fn prefix(s: &str) -> &str

// Ejemplo
// assert_eq!(prefix("user:password"), "user");
// assert_eq!(prefix("admin"), "admin");


// 👉 Muy similar a first_word, pero con otro separador.
fn prefix(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ':' {
            return &s[..i];
        }
    }
    s
}

// 🟡 Ejercicio 4 — Slice vs mutación (error intencional)

// Objetivo: Entender por qué slices previenen bugs.

// Código base
// fn main() {
//     let mut s = String::from("hola mundo");

//     let w = first_word(&s);

//     s.clear();

//     println!("{w}");
// }

// Tu tarea

// Explica por qué no compila

// Reescribe el código para que:

// Compile

// Siga usando slices

// No use clone

// 👉 No lo “arregles” quitando el slice: razona el borrowing.
fn _fix_error() {
    let mut s = String::from("hola mundo");

    let w = first_word(&s);

    // s.clear(); Para que esto funcione no puede haber una referencia mutable coexistiendo con el slice

    println!("{w}");

    s.clear();
}