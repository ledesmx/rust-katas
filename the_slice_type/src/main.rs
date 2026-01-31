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