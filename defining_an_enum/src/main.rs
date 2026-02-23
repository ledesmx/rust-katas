fn main() {
    let mut order1 = Order::new(77, "bad bunny");
    println!("{}", order1.status_text());
    order1.ship();
    println!("{}", order1.status_text());
    order1.deliver();
    println!("{}", order1.status_text());

    let mut order2 = Order::new(44, "black eyed peas");
    order2.cancel();
    println!("{}", order2.status_text());

    let mut user = User::new("Dan Dan");
    println!("{}", user.contact_info());
    user.set_email("daniel@gmail.com");
    println!("Tiene email {}", user.has_email());
    println!("{}", user.contact_info());
}

// Ejercicio 1 - Sistema de estados de pedido
// Modela un pedido en una tienda en linea
// Crea enum para el estod del pedido: pending, shipped, delivered, cancelled
// Crea struct para el pedido con: id, customer: status (el enum del status)
// Implementa los metodos:
// - new(id, customer) Self - crea un nuevo pedido
// - ship - cambia al estado shipped
// - deliver - cambia al estado delivered
// - cancel - cambia al estado canceled
// - status_text() String - Te da el estatus del pedido en texto
// No se puede entregar si no esta enviado
// No se puede enviar si esta cancelado
// No se puede cancelar si ya esta enviado o entregado

#[derive(PartialEq)]
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

struct Order {
    id: i32,
    customer: String,
    status: OrderStatus,
}

impl Order {
    fn new(id: i32, customer: &str) -> Self {
        Self {
            id,
            customer: String::from(customer),
            status: OrderStatus::Pending,
        }
    }
    fn ship(&mut self) {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Shipped;
        }
    }
    fn deliver(&mut self) {
        if self.status == OrderStatus::Shipped {
            self.status = OrderStatus::Delivered;
        }
    }
    fn cancel(&mut self) {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Cancelled;
        }
    }
    fn status_text(&self) -> String {
        if self.status == OrderStatus::Pending {
            format!("{}'s order with id {}: PENDING", self.customer, self.id)
        } else if self.status == OrderStatus::Shipped {
            format!("{}'s order with id {}: SHIPPED", self.customer, self.id)
        } else if self.status == OrderStatus::Delivered {
            format!("{}'s order with id {}: DELIVERED", self.customer, self.id)
        } else {
            format!("{}'s order with id {}: CANCELLED", self.customer, self.id)
        }
    }
}

// Ejercicio 2 - Sistema de Usuario con Email opcional
// Modela un usuario que puede o no tener email
// User: username: String, emali: option, active: bool
// Metodos:
// - new: crean nuevo usuario a partir del username
// - set_email: agrega email al usuario
// - has_email: true si tiene un email
// - contact_info: regresa la info del usuario en un String
// Si tiene email contact_info debe incluirlo, sino "No email registrado"
// No usar valor interno sin verificar antes
// Practicar Option, is_some(), unwrap(), revisar documentacion de Option

struct User {
    username: String,
    email: Option<String>,
    active: bool,
}
impl User {
    fn new(username: &str) -> Self {
        Self {
            username: String::from(username),
            email: None,
            active: true,
        }
    }
    fn set_email(&mut self, email: &str) {
        self.email = Some(String::from(email));
    }
    fn has_email(&self) -> bool {
        self.email.is_some()
    }
    fn contact_info(&self) -> String {
        let active = if self.active {
            "Activo"
        } else {
            "Inactivo"
        };
        let email = self.email.clone().unwrap_or(String::from("No Email registrado"));

        format!("{} {} {}", self.username, email, active)
    }
}