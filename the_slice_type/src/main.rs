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

    let arr = [5, 6, 9, 6, 6, 2, 1, 9, 0, 5, 6];
    println!("Sum of slice {}", sum_slice(&arr[3..9]));
    println!("Sum of slice {}", sum_slice(&arr[..]));

    print_window(&arr[5..]);
    print_window(&arr);

    let data1 = [1, 2, 3, 4, 5, 6];
    let w1 = first_window_above(&data1, 3, 10);
    println!("{:?} WS:{} L:{}, ANSWER:{:?}", data1, 3, 10, w1);
    let data1 = [1, 2, 3, 4, 5, 6];
    let w1 = first_window_above(&data1, 2, 13);
    println!("{:?} WS:{} L:{}, ANSWER:{:?}", data1, 2, 13, w1);
    let data1 = [1, 2, 3, 4, 5, 6];
    let w1 = first_window_above(&data1, 4, 18);
    println!("{:?} WS:{} L:{}, ANSWER:{:?}", data1, 4, 18, w1);

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

// 🟡 Ejercicio 5 — Contador usando slice de array

// Objetivo: Aprender que slices no son solo para strings.

// Enunciado

// Dado un arreglo de números, devuelve la suma de una sección.

// Firma
// fn sum_slice(slice: &[i32]) -> i32

// Ejemplo
// fn main() {
//     let nums = [1, 2, 3, 4, 5];
//     let part = &nums[1..4];

//     assert_eq!(sum_slice(part), 2 + 3 + 4);
// }

// Reglas

// No copies el array

// Usa for

// 👉 Aquí el concepto es idéntico a &str, solo cambia el tipo.
fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for n in slice.iter() {
        sum = sum + n;
    }
    sum
}

// 🟡 Ejercicio 6 — Ventana deslizante (lógica + slices)

// Objetivo: Pensar slices dinámicos.

// Enunciado

// Dado un array y un tamaño k, imprime todas las ventanas contiguas de tamaño k.

// Ejemplo
// let a = [1, 2, 3, 4];

// print_windows(&a, 2);


// Salida esperada:

// [1, 2]
// [2, 3]
// [3, 4]

// Firma sugerida
// fn print_windows(a: &[i32], k: usize)


// 👉 No devuelvas nada, solo imprime.
// 👉 Piensa en índices + slices (&a[i..i+k]).
fn print_window(nums: &[i32]) {
    if nums.len() < 1 {
        return;
    }
    let mut prev_n = &nums[0];
    for (i, n) in nums.iter().enumerate() {
        if i == 0 {
            println!("-- Starting --");
            continue;
        }
        println!("Window [{}, {}]", prev_n, n);
        prev_n = n;
    }
}

// 🔴 Ejercicio 8 — Detectar diseño incorrecto

// Objetivo: Pensar como diseñador de APIs.

// Código sospechoso
// fn word_range(s: &str) -> (usize, usize)

// Tu tarea

// Explica por qué esta API es peligrosa

// Propón una versión segura usando slices

// Explica qué bug evita Rust con esto

// 👉 No escribas código primero: razona el problema.

// 🧠 Reto mental (muy importante)

// Responde con tus palabras:

// ¿Por qué un &str no puede existir sin el texto original?

// ¿Por qué un slice es mejor que devolver índices?

// ¿Qué tipo de bug real evita Rust aquí?

// Si puedes explicarlo sin mencionar “porque Rust lo prohíbe”, ya lo entendiste 😉

// Esta API es mejor porque con el slice Rust se encarga de mantener ligado el slice a la referencia original, si
// esta cambia y tratamos de usar el slice Rust no compilara, Rust nos cuida de errores en tiempo de compilacion.
// De lo contrario debemos mantener nosotros is indices actualizados lo cual es mas propenso a errores
fn _word_range(s: &str) -> &str { s }

// 🧩 Reto 2 — Filtro de ventana segura (slices + lógica)
// Problema

// Tienes sensores que producen lecturas numéricas.
// Quieres analizar ventanas consecutivas del sensor sin copiar datos.

// Objetivo

// Escribe una función que:

// Recorra un array usando ventanas deslizantes

// Devuelva la primera ventana cuya suma sea mayor a un umbral

// Firma obligatoria
// fn first_window_above(nums: &[i32], window_size: usize, limit: i32) -> &[i32]

// Reglas

// Si no existe ventana válida, devuelve un slice vacío &[]

// No copies el array

// No uses Vec

// Ejemplo
// let data = [1, 2, 3, 4, 5, 6];

// let w = first_window_above(&data, 3, 10);
// assert_eq!(w, &[4, 5, 6]);

// Lo difícil aquí

// Manejar rangos correctamente (i..i+window_size)

// Evitar out-of-bounds

// Entender que un slice puede ser resultado de lógica compleja

// 👉 Este patrón se usa muchísimo en sistemas y data processing.

fn first_window_above(nums: &[i32], window_size: usize, limit: i32) -> &[i32] {
    let mut first_index = 0;
    let mut last_index = window_size;
    
    while last_index <= nums.len() as usize {
        let answer = &nums[first_index..last_index];
        if sum_slice(answer) >= limit {
            return answer;
        } else {
            last_index = last_index + 1;
            first_index = first_index + 1;
        }
    }

    &[]
}
