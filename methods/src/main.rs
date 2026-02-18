fn main() {
    let mut jesse = BankAccount{
        owner: String::from("Jesse Adams"),
        balance: 460,
    };
    jesse.deposit(60);
    println!("Jesse account: {jesse:#?}");
    println!("Retirar {}", dbg!(jesse.withdraw(200)));
    println!("Retirar {}", dbg!(jesse.withdraw(600)));
}

// Ejercicio 1 - Cuenta bancaria con estado
// Modela una cuenta bancaria, con owner y balance
// Implementa los metodos:
// - deposit(&mut self, amount: i32)
// - withdraw(&mut self, amount: i32) -> bool
// - is_overdrawn(&self) -> bool
// No debe permitir balance negativo
// No usar clone. Practicar &mut self, mutabilidad interna, control de flujo, retornar bool 
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: i32,
}
impl BankAccount {
    fn deposit(&mut self, amount: i32) {
        self.balance = self.balance + amount;
    }
    fn withdraw(&mut self, amount: i32) -> bool {
        if amount > self.balance {
            return false;
        } else {
            self.balance  = self.balance - amount;
        }
        true
    }
}
