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
