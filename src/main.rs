

fn main() {
    println!("Enter a unit to convert from: C or F. Enter 'Quit' to exit the program.");
    loop {
        let mut unit: String = String::new();
        std::io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit: String = match unit.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        if unit == "F" {
            let degrees = get_degrees();
            println!("{} degrees Fahrenheit is equal to {:.2} degrees Celsius.", degrees, to_celcius(degrees));
            break;
        } else if unit == "C" {
            let degrees = get_degrees();
            println!("{} degrees Celsius is equal to {:.2} degrees Fahrenheit.", degrees, to_fahrenheit(degrees));
            break;
        } else if unit == "Quit" {
            break;
        } else {
            println!("Please type either 'C' or 'F'.")
        }
    }
}

fn get_degrees() -> f64 {
    println!("Now, enter the number of degrees to be converted.");
    loop {
        let mut degrees: String = String::new();
        std::io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");

        match degrees.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number!"),
        }
    }
}

fn to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}
