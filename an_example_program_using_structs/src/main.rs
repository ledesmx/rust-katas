// Proyecto: Sistema de presupuestos personales
// Crear un pequeno programa que modele un gasto mensual y permita:
// - Calcular el total gastado
// - Detectar si estas en deficit
// - Imprimir informacion en modo debug
// - Usar borrowing correctamente
// - Usar #[derive(Debug)]
// - Usar dbg! estrategicamente
// Sin metodos, solo funciones que reciben &Struct
fn main() {
    let width = 50;
    let height = 60;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    let rect = (5, 64);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect)
    );

    let rect2 = Rectangle{
        width: 23,
        height: 80,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect2)
    );
    println!("rect2 is {rect2:?}");
    println!("rect2 is {rect2:#?}");
    dbg!(&rect2);
}
// It's not clear aniwhere in our program that the parameters are related
fn area(width: u32, height: u32) -> u32 {
    width * height
}
// Refactoring with tuple
// This version let us add a bit of structure, but is less clear
// If we want do draw the rectantgle the clearness wouid matter
fn area_tuple(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}
// Refactoring with structs
// We use structs to add meaning by labeling the data
// include de derive to print out debuggin information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
