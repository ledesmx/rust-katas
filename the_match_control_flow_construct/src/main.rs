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
