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

// 🟢 Ejercicio 2 — Normalizador de texto (borrow mutable)

// Objetivo: Entender cuándo necesitas &mut.

// Enunciado

// Crea una función que:

// Convierta el texto a minúsculas

// Agregue ! al final si no existe

// Firma esperada
// fn normalize(text: &mut String)
fn normalize(text: &mut String) {
    for (i, c) in text.chars().enumerate() {
        match c {
            'A' => {text.insert(i, 'a');}
            'B' => {text.insert(i, 'b');}
            'C' => {text.insert(i, 'c');}
            // 'D' => {}
            // 'E' => {}
            // 'F' => {}
            // 'G' => {}
            // 'H' => {}
            // 'I' => {}
            // 'J' => {}
            // 'K' => {}
            // 'L' => {}
            // 'M' => {}
            // 'N' => {}
            // 'O' => {}
            // 'P' => {}
            // 'Q' => {}
            // 'R' => {}
            // 'S' => {}
            // 'T' => {}
            // 'U' => {}
            // 'V' => {}
            // 'W' => {}
            // 'X' => {}
            // 'Y' => {}
            // 'Z' => {}
            _ => {}
        }
    }
}

// 🟡 Ejercicio 3 — Longitud máxima sin mover datos

// Objetivo: Comparar datos usando referencias.

// Enunciado

// Escribe una función que reciba dos Strings prestados y devuelva la longitud del más largo.

// Firma
// fn max_length(a: &String, b: &String) -> usize

// Ejemplo
// fn main() {
//     let a = String::from("hola");
//     let b = String::from("hola mundo");

//     let m = max_length(&a, &b);
//     println!("La longitud mayor es {m}");
// }

// Restricciones

// No clones

// No muevas ownership

// No retornes referencias (solo el número)

// 👉 Aquí practicas múltiples referencias inmutables al mismo tiempo.

// 🟡 Ejercicio 4 — Editor controlado (scopes + mutable borrow)

// Objetivo: Resolver conflictos de borrowing con scopes.

// Enunciado

// Dado un String, realiza:

// Leer su longitud

// Luego agregar texto al final

// Este código NO compila tal como está:

// let mut s = String::from("hola");

// let len_ref = &s;
// s.push_str(" mundo");

// println!("{len_ref}");

// Tu tarea

// Reescríbelo para que:

// Compile

// No clones el String

// Respete las reglas de borrowing

// 👉 Pista: piensa en cuándo deja de usarse la referencia.

// 🟡 Ejercicio 5 — Reemplazo seguro

// Objetivo: Pensar como el borrow checker.

// Enunciado

// Escribe una función que:

// Reemplace todas las letras 'a' por '@'

// Solo si el texto tiene más de 5 caracteres

// Firma
// fn replace_a(text: &mut String)

// Ejemplo
// fn main() {
//     let mut s = String::from("banana");
//     replace_a(&mut s);

//     println!("{s}"); // "b@n@n@"
// }

// Restricciones

// Primero lee, luego modifica

// No tengas referencias activas al modificar

// 👉 Este ejercicio es 100% mental, no sintáctico.

// 🔴 Ejercicio 6 — Arregla el error (diagnóstico de borrowing)

// Objetivo: Leer errores del compilador y entenderlos.

// Código roto
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &mut s;

//     println!("{r1} {r2}");
// }

// Tu tarea

// Explica por qué NO compila

// Modifica el código para que:

// Use referencias

// Compile

// Imprima el contenido final

// 👉 No borres referencias “a lo bruto”. Arréglalo bien.

// 🔴 Ejercicio 7 — Anti-dangling (pensamiento de ownership)

// Objetivo: Detectar referencias inválidas.

// Enunciado

// Este diseño es incorrecto:

// fn make_message() -> &String {
//     let s = String::from("hola");
//     &s
// }

// Tu tarea

// Explica por qué es peligroso

// Reescribe la función correctamente, usando solo lo visto en la lección

// 👉 No menciones lifetimes, aún no existen para ti 😄

// 🧠 Reto mental extra (opcional)

// Sin escribir código:

// ¿Por qué Rust sí permite muchas referencias inmutables?

// ¿Qué bug real evita la regla de “solo un &mut”?

// Explícalo con tus palabras.

// Si quieres, en el próximo mensaje puedo:

// ✅ Revisar tus soluciones

// ❌ Dar soluciones paso a paso

// 🧪 Convertir estos ejercicios en tests fallidos

// 🎮 Hacer mini-proyectos (ej: editor de texto seguro)

// Tú mandas, Rustacean 🦀🔥