
enum Pet {
    DOG,
    CAT,
    FISH,
}

impl Pet {
    fn what_am_i(&self) -> &'static str {
        match self {
            Pet::DOG => "I am a dog",
            Pet::CAT => "I am a cat",
            Pet::FISH=> "I am a fish"
        }
    }
}

enum IpAddressKind{
    IP4(String),
    IP6,
}


struct IpAddress{
    kind:IpAddressKind,
    address:String,
}

fn main() {
 let dog = Pet::DOG;
    println!("{}",dog.what_am_i());

 let home = IpAddress{
     kind: IpAddressKind::IP4(String::from("127.0.0.1")),
     address: String::from("127.0.0.1"),
 };

 let loopback = IpAddress{
       kind:IpAddressKind::IP6,
       address:String::from("::1"),
 };

 let some_number = Some(5);
 let some_string = Some("I am a string");
 let nothing: Option<i32> = None;  // Option<T> let x = 5 == i32
 let x:i32 = 5;

 let five = Some(5);
 let six = plus_one(five);
 let none =plus_one(None);

    println!("{:?}",six);
    what_pet("dog");
    what_pet("Cow");
    what_pet("Cat");

    let dog2 = Some(Pet::DOG);
    if let Some(Pet::DOG) = dog2 {  // this is like one case match
        println!("The animal is dog!")
    } else {
        println!("Not a dog")
    }

    let mut stack = Vec::new(); // last in first out
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    let x = 5;
    match x {
        1 | 2 => println!("One or two"),
        _ => println!("Not one or two"),
    }

    match x {
        1..=5 => println!("Matches!"),
        _ => println!("Not matching")
    }

    let u = Some(5);
    let t = 5;
    match u {
        Some(10) => println!("10!"),
        Some(u) if u == t => println!("Matches!!"),
        _ => println!("Nothing matches")
    }
}

// if option you can do stuff
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog!"),
        "Cat" => println!("I have a cat!"),
        "Fish" => println!("I have a fish!"),
          _ => println!("I have no clue"),
    }
}
