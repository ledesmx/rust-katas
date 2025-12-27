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

    // 🧩 Statements vs Expressions
    // 4. ¿Por qué esto no funciona?

    // Intenta (mentalmente o en código) algo como:

    // let x = (let y = 10);

    // Luego:

    // reescribe el código para que sí compile, usando un bloque {}.

    // Pregunta clave:
    // ¿Por qué un bloque puede devolver un valor pero un let no?
    let x = {
        let y = 11;
        y
    };
    println!("{x}");

    // 5. Bloque como expresión
    let no = no_parameters();
    println!("{no}");

    // 8. Retorno anticipado
    println!("Paso 33 -> {}", zero_one(33));
    println!("Paso -5 -> {}", zero_one(-5));
    println!("Paso  0 -> {}", zero_one(0));

    // 🧩 PROBLEMA 1 — Función como expresión (nivel real)
    println!("{}", ajustar_cal(promedio([30.0, 42.0, 45.9, 87.0, 98.3])));
    println!("{}", ajustar_cal(promedio([99.0, 95.0, 99.9, 100.0, 98.3])));
    println!("{}", ajustar_cal(promedio([0.0, 2.0, 2.9, 0.0, 0.3])));

    // 🧩 PROBLEMA 2 — Encadenamiento (qué es y para qué sirve)
    let final_score = multiplicador(recompensa(bono_inicial(777)));
    println!("{final_score}");
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

// 5. Bloque como expresión

// Crea una función que:

// no reciba parámetros

// dentro tenga un bloque {} que:

// declare una variable

// haga una operación

// devuelva un valor

// Asigna ese valor a una variable en main.
// Pregunta clave:
// ¿Qué línea del bloque es la que realmente “devuelve” el valor?
fn no_parameters() -> i32 {
    {
        let x = 3;
        let y = 5;
        x + y
    }
}

// 8. Retorno anticipado

// Crea una función que:

// reciba un i32

// si el número es negativo, devuelva 0

// si no, devuelva el número + 1

// 👉 Usa return solo en uno de los caminos.

// Pregunta clave:
// ¿Qué pasa con el flujo cuando se ejecuta return?
fn zero_one(x: i32) -> i32 {
    if x < 0 {
        return 0;
    }
    x + 1
}

// 🧩 PROBLEMA 1 — Función como expresión (nivel real)
// 🧠 Problema

// Estás escribiendo un programa para evaluar la calificación final de un estudiante.

// Reglas:

// Una función calcula el promedio base de dos exámenes.

// Otra función ajusta la calificación:

// Si el promedio es mayor o igual a 60 → se suma un bono de 5

// Si es menor → se resta una penalización de 5

// El resultado final se imprime en main

// ⚠️ Restricción importante:

// No puedes guardar el resultado intermedio en una variable

// Debes usar el resultado de una función directamente dentro de otra expresión

// 📌 Qué debes implementar

// Una función que reciba dos i32 y devuelva el promedio

// Una función que reciba un i32 y devuelva la calificación ajustada

// En main, resuelve todo en una sola expresión

// Ejemplo conceptual (NO solución):

// let final = ajustar( calcular_promedio(70, 80) );

fn promedio(cal: [f64; 5]) -> f64 {
    (cal[0] + cal[1] + cal[2] + cal[3] + cal[4]) / 5.0
}
fn ajustar_cal(cal: f64) -> f64 {
    let mut result = cal;
    if cal >= 60.0 {
        result += 5.0;
    } else {
        result -= 5.0;
    }
    if result > 100.0 {
        return 100.0;
    }
    if result < 0.0 {
        return 0.0;
    }
    result
}

// 🧩 PROBLEMA 2 — Encadenamiento (qué es y para qué sirve)
// ❓ ¿Qué es encadenamiento?

// Encadenar funciones significa:

// Tomar el resultado de una función y pasarlo directamente a la siguiente, sin detener el flujo

// No es algo “especial” de Rust, es pensamiento funcional básico.

// Visualmente:

// valor → función A → función B → función C → resultado final

// 🧠 Problema

// Tienes un sistema de puntos para un videojuego.

// Reglas:

// El jugador empieza con una puntuación base

// Se le da un bono inicial

// Luego se aplica una recompensa extra

// Finalmente se aplica un multiplicador final

// Cada paso debe ser una función distinta.

// 📌 Funciones requeridas

// bono_inicial(puntos) → suma 1

// recompensa(puntos) → suma 2

// multiplicador(puntos) → suma 3

// ⚠️ Restricciones:

// Ninguna función imprime

// Cada función devuelve un valor

// En main, encadena las funciones

// Resuelve el problema completo desde main

// Ejemplo conceptual:

// let puntos_finales = multiplicador(
//     recompensa(
//         bono_inicial(puntos_base)
//     )
// );

fn bono_inicial(puntos: i32) -> i32 {
    puntos + 15
}
fn recompensa(puntos: i32) -> i32 {
    puntos + 33
}
fn multiplicador(puntos: i32) -> i32 {
    let puntosf = puntos as f64 * 1.5;
    puntosf as i32
}
