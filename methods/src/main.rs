fn main() {
    let mut jesse = BankAccount{
        owner: String::from("Jesse Adams"),
        balance: 460,
    };
    jesse.deposit(60);
    println!("Jesse account: {jesse:#?}");
    println!("Retirar {}", dbg!(jesse.withdraw(200)));
    println!("Retirar {}", dbg!(jesse.withdraw(600)));

    let box1 = Box3D{
        width: 10,
        height: 5,
        depth: 10,
    };
    let box2 = Box3D{
        width: 3,
        height: 3,
        depth: 3,
    };
    let box3 = Box3D{
        width: 20,
        height: 5,
        depth: 20,
    };
    println!("{box1:#?} contains {box2:#?} {}", box1.can_hold(&box2));
    println!("{box1:#?} contains {box3:#?} {}", box1.can_hold(&box3));
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

// Ejerccio 2 - Comparacion entre instancias
#[derive(Debug)]
struct Box3D {
    width: i32,
    height: i32,
    depth: i32,
}
// Implementa metodo para calcular volumen 
// y metodo para saber si este Box puede contener otro Box
// Debes comparar volumen y dimensiones individuales
// Practicar: metodos con mas parametros, borrowing automatico, razonamiento logico
impl Box3D {
    fn volume(&self) -> i32 {
        self.width * self.height * self.depth
    }
    fn can_hold(&self, other: &Box3D) -> bool {
        if other.volume() > self.volume() 
        || other.width > self.width 
        || other.height > self.height
        || other.depth > self.depth {
            false
        } else {
            true
        }
    }
}


