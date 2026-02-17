fn main() {
    println!("Hello, world!");
}

// Ejercicio 1 - Cuenta bancaria con estado
// Modela una cuenta bancaria, con owner y balance
// Implementa los metodos:
// - deposit(&mut self, amount: i32)
// - withdraw(&mut self, amount: i32) -> bool
// - is_overdrawn(&self) -> bool
// Withdraw devuelve false si no hay suficiente dinero
// No debe permitir balance negativo
// No usar clone. Practicar &mut self, mutabilidad interna, control de flujo, retornar bool 
