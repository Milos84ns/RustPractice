use std::env;
use std::io::Error;

pub enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    BLACK,
    WHITE,
}

 pub enum GasType {
     DEASEL,
     GASOLINE,
     ELECTRICAL,

 }



pub struct Car {
    color: Color,
    max_speed: u16,
    mileage: u16,
    gas_consumption: u16,
    gas_type: GasType,
}

impl Car {
    fn new() -> Self {
        Car{
            color: Color::BLACK,
            max_speed: 160,
            mileage: 0,
            gas_consumption: 3,
            gas_type: GasType::DEASEL
        }
    }


}

fn main() {
    println!("Hello, world!");


}