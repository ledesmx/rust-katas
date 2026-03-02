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
    // println!("{:?}", validate_config(config));
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

