struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}


// tuple struct
struct Coordinates(i32,i32,i32);

struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

struct MyString<'a> {
    text: &'a str,
}

impl Square {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn whats_my_witdh(&self) -> u32 {
        return self.width;
    }

    fn change_width(&mut self,new_width:u32) {
        self.width = new_width;
    }
}

fn main() {
    let user1 = User { active: true, username: String::from("Tyler"), sign_in_count: 0 };
    println!("{}", user1.username);

    let user2 = build_user(String::from("Tyler2"));
    println!("{}", user2.username);

    let cords = Coordinates(1, 2, 3);

    let mut sq = Square { width: 5, height: 5 };
    println!("{}", sq.area());
    println!("My width {}:", sq.whats_my_witdh());
    sq.change_width(10);
    println!("My new width {}", sq.whats_my_witdh());


    // dangling reference
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r); //`a

    // &i32
    // &'a i32
    // &'a mut i32


    // lifetime

    let str1 = String::from("This is my string");

    let x = MyString{text: str1.as_str()};

    let s: &'static str = "I have static lifetime"; // lifetime lives forever

}

fn example<'a,'b>(x: &'a str, y: &'b str) -> &'b str {
    x
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count:1,
    }
}
