fn main() {
    let text1 = String::from("Bad Bunny");
    let text2 = &String::from("Losing my religion");
    let text3 = "Can you read my mind?";
    println!("{}", first_word(&text1));
    println!("{}", first_word(text2));
    println!("{}", first_word(text3));
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
