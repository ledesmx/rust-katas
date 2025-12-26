fn main() {
    // 1. Intercambio sin variables temporales
    let a = inter((3, 5));
    println!("{} {}", a.0, a.1);

    let a = inter2((3, 5.99, 'a', true));
    println!("{} {} {} {}", a.0, a.1, a.2, a.3);
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
