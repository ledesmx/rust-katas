fn main() {
    // 1. Intercambio sin variables temporales
    let a = inter((3, 5));
    println!("{} {}", a.0, a.1);

    let a = inter2((3, 5.99, 'a', true));
    println!("{} {} {} {}", a.0, a.1, a.2, a.3);

    // 2. Analizador de punto en el plano
    println!("{}", which_plane((3, -5)));
    println!("{}", which_plane((0, 0)));
    println!("{}", which_plane((3, 5)));
    println!("{}", which_plane((-4, -5)));
    println!("{}", which_plane((0, -5)));

    // 6. Suma de elementos
    println!("{}", array_sum([4, 2, 5, 2, 4]));

    // 7. Buscar el valor máximo
    println!("{}", array_max([4.3, 8.0, 1.9, 3.0, 0.3]));
    println!("{}", array_max([-4.0, -8.0, 1.0, -3.0, 0.0]));
    println!("{}", array_max([44.3, 44.4, 44.1, -33.3, 10.0]));
}

// 1. Intercambio sin variables temporales

// Dada una tupla (i32, i32) que representa (a, b), escribe un programa que intercambie los valores y devuelva una nueva tupla (b, a).

// 👉 Restricción:

// No puedes usar variables temporales adicionales (solo destructuring).

// Pregunta clave:
// ¿Cómo puedes usar el destructuring para “reordenar” valores?

fn inter(x: (i32, i32)) -> (i32, i32) {
    let (n1, n2) = x;
    (n2, n1)
}
fn inter2(nums: (i32, f64, char, bool)) -> (bool, char, i32, f64) {
    (nums.3, nums.2, nums.0, nums.1)
}

// 2. Analizador de punto en el plano

// Representa un punto en 2D como una tupla (i32, i32).

// Escribe un programa que:

// Determine si el punto está:

// en el origen

// sobre el eje X

// sobre el eje Y

// o en un cuadrante

// 👉 Usa match sobre la tupla.

// Pregunta clave:
// ¿Cómo puedes usar patrones como (0, y) o (x, 0)?
fn which_plane(vector: (i32, i32)) -> String {
    let (x, y) = vector;

    let result: String;

    if x > 0 {
        if y > 0 {
            result = String::from("right up quadrant");
        } else if y < 0 {
            result = String::from("right down quadrant");
        } else {
            result = String::from("on axis x right");
        }
    } else if x < 0 {
        if y > 0 {
            result = String::from("left up quadrant");
        } else if y < 0 {
            result = String::from("left down quadrant");
        } else {
            result = String::from("on axis x left");
        }
    } else {
        if y > 0 {
            result = String::from("on axis y up");
        } else if y < 0 {
            result = String::from("on axis y down");
        } else {
            result = String::from("center");
        }
    }
    result
}

// 6. Suma de elementos

// Dado un array [i32; 5], calcula la suma total de sus elementos usando índices, no for.

// 👉 Pista:
// Accede manualmente a cada posición.

// Pregunta clave:
// ¿Qué pasa si el tamaño del array cambia?

fn array_sum(arr: [i32; 5]) -> i32 {
    arr[0] + arr[1] + arr[2] + arr[3] + arr[4]
}

// 7. Buscar el valor máximo

// Dado un array [i32; 5], encuentra el valor más grande.

// 👉 Restricciones:

// No uses métodos avanzados

// No uses vectores

// Pregunta clave:
// ¿Cómo mantienes el “máximo actual” mientras recorres el array?
fn array_max(arr: [f64; 5]) -> f64 {
    let mut max = arr[0];

    if arr[1] > max {
        max = arr[1];
    }
    if arr[2] > max {
        max = arr[2];
    }
    if arr[3] > max {
        max = arr[3];
    }
    if arr[4] > max {
        max = arr[4];
    }

    max
}
