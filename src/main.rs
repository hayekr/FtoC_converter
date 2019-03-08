use std::io;

// C to F: F = C*(9/5) + 32
// F to C: C = (F-32)*(5/9)

/**********Converts between Fahrenheit and Celsius*********/

fn main() -> () {
    println!("Do you want to convert to Celsius or Fahrenheit? Input C or F");
    let mut convert_type = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to conversion type.");

    let t = String::from(convert_type);

    println!("You want to convert to: {}", t);
    println!("What temperature would you like to convert?");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature.");

    let temp: i32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_e) => -1,
    };

    match t.as_str() {
        "C\n" => println!("{}", ftoc(temp)),
        "F\n" => println!("{}", ctof(temp)),
        _ => println!("t = {:?}", t),
    }
}

// Celsius to Fahrenheit
fn ctof(c: i32) -> i32 {
    (c * (9 / 5)) + 32
}

//Fahrenheit to Celsius
fn ftoc(f: i32) -> i32 {
    (f - 32) * (5 / 9)
}
