fn main() {
    let s1 = String::from("hola");
    let s2 = String::from("si tu me diste un vide de suerte");
    println!("hola -> {}", count_vowels(&s1));
    println!("si tu me diste un vide de suerte -> {}", count_vowels(&s2));
    println!("2x -> {}", count_vowels(&String::from("2x")));
}
// 🟢 Ejercicio 1 — Contador de vocales (borrow inmutable)

// Objetivo: Practicar referencias inmutables y evitar mover el String.

// Enunciado

// Escribe una función que cuente cuántas vocales tiene un texto.

// La función NO debe tomar ownership del String

// El String debe seguir siendo usable en main

// Firma esperada
// fn count_vowels(text: &String) -> usize

// Reglas

// Usa un for y match o if

// No clones el String

// No lo modifiques
fn count_vowels(text: &String) -> usize {
    let mut vowels: usize = 0;
    for c in text.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
        }
    }
    vowels
}