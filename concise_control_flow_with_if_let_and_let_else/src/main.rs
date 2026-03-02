fn main() {
    let conf = ServerConfig {
        max_connections: Some(3066),
        timeout_seconds: Some(200),
    };
    print_max_connections(&conf);
    println!("Timeout: {}", effective_timeout(&conf));
    let conf = validate_config(conf);
    match conf {
        Err(msg) => println!("{}", msg),
        Ok(conf) => println!("{} {}", conf.0, conf.1),
    }
    
    process_command(Some("start"));
    process_command(Some("pause"));
    process_command(Some("stop"));
    process_command(None);

    let potion = Item::Potion(66);
    let sword = Item::Sword;
    println!("Potion: {:?}", consume_potion(potion));
    use_item(sword);
}
// Ejercicio 1 - Configuracion del servidor
// Estas creando un sistema de servidor
// Tienes un struct ServerConfig: max_connections: Option<u32>, timeout_seconds: Option<u32>
// Crea una funcion print_max_connections(config: &ServerConfig)
// - Si max_connections es some imprimir el valor, si no no hacer nada
// Crea una funcion effective_timeout(config: &ServerConfig): u32
// - Si temeout_seconds es some, devolver el valor, sino devolver 30 por defecto
// Crea validate_config(config: ServerConfig): Result<ServerConfig, String>
// - Si max_connection es None, debe retornar error
// - Si timeout_seconds es None, retornar error
// - Si ambos existen, devolver el config valido 
// Usar if let, if let else
#[derive(Debug)]
struct ServerConfig {
    max_connections: Option<i32>,
    timeout_seconds: Option<i32>,
}
fn print_max_connections(config: &ServerConfig) {
    if let Some(connections) = config.max_connections {
        println!("Connections {connections}");
    }
}
fn effective_timeout(config: &ServerConfig) -> i32 {
    if let Some(timeout) = config.timeout_seconds {
        timeout
    } else {
        30
    }
}
fn validate_config(config: ServerConfig) -> Result<(i32, i32), String> {
    let Some(max_connections) = config.max_connections else {
        return Err(String::from("Connection error"));
    };
    let Some(timeout_seconds) = config.timeout_seconds else {
        return Err(String::from("Timeout error"));
    };
    Ok((max_connections, timeout_seconds))
}

// Ejercicio 2 - Parser simple de comando
// Simula que recibes un comando opcional: Option<&str>
// Puede venir:
// Some("start")
// Some("stop")
// Some("pause")
// None
// Escribe la funcion proccess_command(cmd: Option<&str>)
// - Si es None -> Imprimir "No command received"
// - Si es Some("start") -> Imprimir "System starting"
// - Si es Some("stop") -> Imprimir "System stopping"
// - Para cualquier otro -> Iprimir "Unknown command"
// Combinar if let
fn process_command(cmd: Option<&str>) {
    if let Some(c) = cmd {
        match c {
            "start" => println!("System starting"),
            "stop" => println!("System stopping"),
            _ => println!("Unknown command"),
        }
    } else {
        println!("No command received");
    }
}

// Ejercicio 3 - Inventario con happy path
// tienes un enum: Item: Potion(u32), Sword, Shield
// Escribe la funcion consume_potion(itme: Item) -> Option<u32>
// - Si es Potion(candidad) -> devolver la cantidad
// - Si no es Potion -> Devolver None
// Usa let else
// Crea funcion use_item(item: Item)
// - Si es Potion -> Imprimir "You healed X HP"
// - Si no -> Imprimir "Item cannot be consumed"
// Usa if let else
enum Item {
    Potion(u32),
    Sword,
    _Shield,
}
fn consume_potion(item: Item) -> Option<u32> {
    let Item::Potion(hp) = item else {
        return None;
    };
    Some(hp)
}
fn use_item(item: Item) {
    if let Item::Potion(hp) = item {
        println!("You healed {hp}");
    } else {
        println!("Item cannot be consumed");
    }
}