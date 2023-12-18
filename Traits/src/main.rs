enum AnimalColor {
    BLACK,
    WHITE,
    STRIPES,
    GRAY,
    BROWN
}

enum AnimalType{
    MAMMAL,
    REPTILE,
    BIRD,
    FISH,
    AMPHIBIAN
}

struct Animal {
    name:String,
    atype:AnimalType,
    age:u16
}

pub trait AnimalTrait {
   fn sound();
}

fn main() {

}
