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

    println!("None++ = {}", plus_one(None).unwrap_or_default());
    println!("6 * 2 = {}", safe_double(Some(6)).unwrap_or_default());
    println!("8 + 1 = {}", add_options(Some(8), Some(1)).unwrap_or_default());
    println!("None + 5 = {}", add_options(None, Some(5)).unwrap_or_default());
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

// Ejercicio 3 - Operaciones con option
// Implementa 
// - plus_one(x: Option<i32>) -> Option<i32>
// - safe_double(x: Option<i32>) -> Option<i32>
// - add_options(x: Option<i32>, y: Option<i32>) -> Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}
fn safe_double(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value * 2),
        None => None,
    }
}
fn add_options(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(v1), Some(v2)) => Some(v1 + v2),
        (Some(v), None) => Some(v),
        (None, Some(v)) => Some(v),
        (None, None) => None,
    }
}

// Ejercicio 4 - Sistema de combate 2D8, Jugador vs GM
// Se lanzan 2 dados de 8 caras, y se suma el resultado, el valor determina una accion
// Crea la funcion handle_player_roll(total: u8)
// - 2 -> Fallo critico. Te tropiezas con tu propia espada,
// - 16 -> Golpe legendario. El mundo tiembla ante ti.
// - 8 -> Golpe perfectamente calculado.
// - 12 -> Combo especias desbloquado.
// - d < 5 -> Ataque debil. apenas rozas al enemigo.
// - d > 13 -> Ataque devastador cargado de energia.
// - 9 < d > 11 -> Ataque solido y consistente.
// - Catch-all -> Ataque normal.
// Crea la funcion handle_enemy_roll(total: u8)
// - 2 -> El enemigo falla miserablemente
// - 15 -> El enemigo invoca una sombra oscura
// - 7 -> El enemigo te estudia y espera.
// - d > 12 -> El enemigo ejecuta un ataque brutal
// - Catch-all -> Nada
// Crea la funcion resolve_turn(plajer_roll: u8, enemy_roll: u8)
// - Llama primero al jugador, luego al enemigo
// - Imprime "--- Player Turn ---" "--- Enemy Turn ---"
// Usa match en todas las funciones
// Practicas match con valores literales, con rangos, orden de evaluacion,
// catch-all, binding del valor capturado, etc