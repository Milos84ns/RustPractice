use std::fmt::Display;
use std::prelude::v1;

enum VehicleColor {
    RED,
    BLUE,
    WHITE
}

enum VehicleType {
    CAR,
    TRUCK,
    BOAT,
    TRAIN,
    AIRPLANE
}

impl VehicleType {
    fn newPlane() ->VehicleType {
        VehicleType::AIRPLANE
    }
}


impl VehicleColor {
    fn new() -> VehicleColor {
        VehicleColor::BLUE
    }
}


fn main() {

    println!("Hello, world!");
    let v1 = VehicleColor::new();

    let v2 = VehicleType::newPlane();
    match v1 {
        VehicleColor::BLUE => println!("Its blue"),
        _ => println!("Not BLUE")
    }

    match v2 {
        VehicleType::AIRPLANE => println!("Its flying thing!"),
        _ => println!("No idea what it is.")
    }

}
