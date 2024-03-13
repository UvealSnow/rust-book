use std::io;

fn main() {
    'option: loop {
        println!("Select an option:");
        println!("a) Convert Celsius to Farenheit:");
        println!("b) Convert Farenheit to Celsius:");
        println!("(Enter 'q' to quit the program)");
    
        let mut selection = String::new();
    
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read selection");

        let selection = match selection.trim() {
            "a" => "Farenheit",
            "b" => "Celsius",
            "q" => break 'option,
            _ => continue 'option,
        };

        'conversion: loop {
            println!("Please input the temperature in {}", get_inverse(selection));
            println!("(Enter 'q' to return to the previous menu)");

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read temperature");

            let response: f64 = match temperature.trim()
                .parse::<f64>() {
                    Ok(num) => match selection {
                        "Farenheit" => num * 1.8 + 32.0,
                        "Celsius" => num / 1.8 - 32.0,
                        "q" => break 'conversion,
                        _ => continue 'conversion,
                    },
                    Err(_) => continue 'conversion,
                };
            
            println!("The temperature in {selection} is: {response}")
        };
    }
}

fn get_inverse(selection: &str) -> &str {
    match selection {
        "Farenheit" => "Celsius",
        "Celsius" => "Farenheit",
        _ => "Invalid",
    }
}
