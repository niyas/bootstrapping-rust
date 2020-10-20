use std::io;
//use rand::Rng;

fn main() {
    println!("Choose an option \n 1. Celsius to Farent Converter \n 2. Farent to Celsius Converter");
    //let mut farent = String::new();
    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read the value");
    
    let option: i32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };     

    if option == 1 {
        println!("Enter a value in Celsius");
        println!("The result is {}F", celsius_to_farent());
        // farent_to_celsius()
    }    
    else if option == 2 {
        println!("Enter a value in Farent");
        println!("The result is {}C", farent_to_celsius());
    } else {
        println!("Invalid option enterd");
    }
   
}

fn farent_to_celsius() -> f32 {
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read the value");

    let val: f32 = match val.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };  
    
    (val - 32.0) * 5.0/9.0
}

fn celsius_to_farent() -> f32 {
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read the value");

    let val: f32 = match val.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };  
    
    (val * 9.0/5.0) + 32.0
}