use std::io;
use std::collections::HashMap;

fn main() {
    'selection: loop {
        println!("Choose a number to calculate the nth Fibonacci number in the sequence: (max: 186)");
        println!("(Enter 'q' to quit the program)");
        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read selection");

        let selection: u128 = match selection.trim()
            .parse() {
                Ok(num) => match num {
                    0..=186 => num,
                    _ => {
                        println!("The number must be between 0 and 186");
                        continue 'selection;
                    }
                },
                Err(err) => match selection.trim() {
                    "q" => break 'selection,
                    _ => {
                        println!("Error: {err}");
                        continue 'selection;
                    }
                },
            };

        let num = fibonacci(selection, &mut HashMap::new());
        println!("The Fibonacci number for {selection} is: {num}");
    };
}

fn fibonacci(n: u128, map: &mut HashMap<u128, u128>) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            match map.get(&n) {
                // We need to use the dereference operator (*) to get the value from the Option<&V> type
                Some(num) => *num,
                None => {
                    let num = fibonacci(n - 1, map) + fibonacci(n - 2, map);
                    map.insert(n, num);
                    num
                }
            }
        },
    }
}
