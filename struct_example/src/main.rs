
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool 
}
#[derive(Debug)]
struct Dimension {
    height: u32,
    width: u32
}

fn main() {
  let user1 = get_user();  
  let user2 = get_another_user(String::from("sumi@beldex.io"),String::from("sumi2109"), user1);
  println!("Email: {} \n Username: {} \n SigninCount: {} \n Active: {}",user2.email, user2.username, user2.sign_in_count, user2.active );

  let dim = Dimension {
      height: 32,
      width: 15
  };

  println!("The area of a rectangle with Height:{} and width:{} is {} square pixals", dim.height, dim.width, area(&dim));

  println!("Dimension Struct: {:#?}", dim);
}

fn get_user() -> User {
    User {
        email: String::from("mohammed.niyas@belex.io"),
        username: String::from("niyas2109"),
        sign_in_count: 3,
        active: true
    }
}

fn get_another_user(email: String, username: String, user1: User) -> User {
    User {
        email,
        username,
        ..user1
    }
}

fn area(dim: &Dimension) -> u32 {
    dim.height * dim.height
}
