fn main() {
    let mut laptop = Product {
        name: String::from("Huawai 700"),
        price: 33000.0,
        stock: StockStatus::InStock(3),
    };
    println!("{}", laptop.stock_text());
    println!("Transaccion: {}", laptop.buy(30));
    println!("Esta aun disponible? {}", laptop.is_available());
    println!("Transaccion: {}", laptop.buy(3));
    println!("{}", laptop.stock_text());

    let actions = [
        Command::Move(6),
        Command::Attack(String::from("Diablo")),
        Command::Quit,
        Command::Heal(2),
    ];
    process_commands(&actions);

}

// Ejercicio 1 - Producto con estado de inventario
// Modela el estado de inventario usando un enum
// StockStatus: InStock i32, OutOfStock
// Product: name: String, price: f64, stock: StockStatus
// Metodos:
// - is_available: regresa true si hay stock, false si no hay
// - buy: si hay inventario reduce la cantidad, sino regresa false
// - stock_text: regresa la info del producto, y si hay stock disponible
// No puedes asumir que siempre hay cantidad
// No puedes restar sin verificar
// Practicas Enum con datos numericos, mutabilidad, evitar estados invalidos

enum StockStatus {
    InStock(i32),
    OutOfStock,
}
struct Product {
    name: String,
    price: f64,
    stock: StockStatus,
}
impl Product {
    fn is_available(&self) -> bool {
        match self.stock {
            StockStatus::OutOfStock => false,
            StockStatus::InStock(..) => true,
        }
    }
    fn buy(&mut self, quantity: i32) -> bool {
        match self.stock {
            StockStatus::OutOfStock => false,
            StockStatus::InStock(stock) => {
                if stock < quantity {
                    false
                } else {
                    let res = stock - quantity;
                    if res == 0 {
                        self.stock = StockStatus::OutOfStock;
                    } else {
                        self.stock = StockStatus::InStock(res);
                    } 
                    true
                }
            },
        }
    }
    fn stock_text(&self) -> String {
        let stock = match self.stock {
            StockStatus::OutOfStock => format!("No disponible"),
            StockStatus::InStock(quantity) => format!("{} disponibles", quantity),
        };
        format!("{}: {} --- {}", self.name, self.price, stock)
    }
}

// Ejercicio 2 - Sistema de comandos de juego
// Crea un enum Command: Move: i32, Attack: String, Heal: u32, Quit
// Crea una funcion execute_command(c: Command)
// - Move debe imprimir "moving n spaces"
// - Attack debe imprimir "attackin target"
// - Heal debe imprimir "healing amount HP"
// - Quit debe imprimir "exiting game"
// Crea una funcion process_commands: &[Command]
// Recorre el slice y ejecuta cada comando, si encuentra quit deja de procesar los siguientes
enum Command {
    Move(i32),
    Attack(String),
    Heal(u32),
    Quit,
}
fn execute_command(cmd: &Command) {
    match cmd {
        Command::Move(n_spaces) => println!("Moving {} spaces", n_spaces),
        Command::Attack(target) => println!("Attacking {}", target),
        Command::Heal(hp) => println!("Healing {} hp", hp),
        Command::Quit => println!("Exiting game"),
    }
}
fn process_commands(cmds: &[Command]) {
    for cmd in cmds {
        match cmd {
            Command::Quit => {
                execute_command(cmd);
                break;
            }
            _ => {}
        }
        execute_command(cmd);
    }
}

// Ejercicio 3 - Sistema seguro con option
// Implementa 
// - plus_one(x: Option<i32>) -> Option<i32>
// - safe_double(x: Option<i32>) -> Option<i32>
// - add_options(x: Option<i32>, y: Option<i32>) -> Option<i32>
