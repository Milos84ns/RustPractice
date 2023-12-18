#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct User {
    username: String,
    password: String,
    email: String,
    userid: u16,
}

pub trait CommonUserTraits {
    fn must_have_email() -> bool;
    fn isAdmin(&self)-> bool;
}


impl Display for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} : {}",self.username,self.email)
    }
}

impl User {
    fn new()-> User{
      return User {
          username: String::from("NewUser"),
          password: String::from("NewPassword"),
          email: String::from("newuser@somemail.com"),
          userid: 1000
      }
    }

    fn example()-> User {
        return User {
            username: String::from("NewUser"),
            password: String::from("NewPassword"),
            email: String::from("newuser@somemail.com"),
            userid: 1000
        }
    }
    fn newName(name: String) -> User {
        return User{
            username: String::from(name),
            password: String::from("NewPassword"),
            email: String::from("newuser@somemail.com"),
            userid: 1000
        }
    }
}

impl Default for User {
    fn default() -> Self {
        return User{
            username: String::from("TestName"),
            password: String::from("NewPassword"),
            email: String::from("newuser@somemail.com"),
            userid: 1000
        }
    }
}


impl CommonUserTraits for User {
    fn must_have_email() -> bool {
        todo!()
    }

    fn isAdmin(&self) -> bool {
        return self.userid == 1000;
    }
}

fn main() {
    let user1 = User::new();
    let user2 = User::newName(String::from("Milos"));
    let user3 = User::default();
    let user4 = User{
        username: String::from("Milos"),
            ..User::example()
    };
    println!("User is {}" ,user1.username);
    println!("New user is {}", user2.username);
    println!("{} is admin: {}",user2.username,user2.isAdmin());
    println!("Default user is : {}", user3.username);
    println!("Display user 3 : {}",user3);
}
