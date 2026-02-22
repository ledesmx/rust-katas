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

    let mut p1 = Player {
        name: String::from("Vegetta777"),
        score: 0,
    };
    p1.add_points(77);
    println!("{} {} {}", p1.name(), p1.score(), p1.is_winner());
    p1.add_points(100);
    println!("{} {} {}", p1.name(), p1.score(), p1.is_winner());
    p1.reset();
    println!("{} {} {}", p1.name(), p1.score(), p1.is_winner());


    let temp = Temperature::new(14.0);
    let temp2 = Temperature::from_fahrenheit(0.0);
    println!("celsius {} fahrenheit {}", temp.celsius, temp.to_fahrenheit());
    println!("celsius {} fahrenheit {}", temp2.celsius, temp2.to_fahrenheit());



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

// Ejercicio 3 - Sistema de puntuacion
#[derive(Debug)]
struct Player {
    name: String,
    score: i32,
}
// Implementa metodos:
// - add_points para agregar puntos a score
// - reset para volver score a 0
// - is_winner devuleve true si score > 100
// - score y name para devolver el valor del campo score y name
// Aqui practicas getter manual, nombre del metono igual que el campo, &self y &mut self
impl Player {
    fn add_points(&mut self, points: i32) {
        self.score = self.score + points;
    }
    fn reset(&mut self) {
        self.score = 0;
    }
    fn is_winner(&self) -> bool {
        self.score > 100
    }
    fn score(&self) -> i32 {
        self.score
    }
    fn name(&self) -> &String {
        &self.name
    }
}

// Ejercicio 4 - Constructor personalizado
struct Temperature {
    celsius: f64,
}
// Cear funciones asociadas
// - new(celsius: f64) -> Self
// - from_fahrenheit(f: f64) -> Self
// Y crea el metodo
// - to_fahrenheit(&self) -> f64
// Aqui practicas Self, funciones asociadas, uso de :: sintaxis
impl Temperature {
    fn new(celsius: f64) -> Self {
        Self {
            celsius: celsius,
        }
    }
    fn from_fahrenheit(fahrenheit: f64) -> Self {
        Self {
            celsius: (fahrenheit - 32.0) * 0.5556,
        }
    }
    fn to_fahrenheit(&self) -> f64 {
        (self.celsius * 1.8) + 32.0
    }
}


