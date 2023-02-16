use std::{io, io::Write, str::FromStr};

fn main() {
    let mut inputs: (Option<f64>, Option<f64>, Option<char>) = (None, None, None);

    while inputs.0 == None {
        print!("Please provide your first number: ");
        io::stdout().flush().expect("flush failed.");
        let given_user_string: Option<String> = get_user_input();

        inputs.0 = match given_user_string {
            Some(string) => match f64::from_str(&string.trim()) {
                Ok(number) => Some(number),
                Err(_) => None,
            },
            None => None,
        };
    }

    while inputs.1 == None {
        print!("Please provide your second number: ");
        io::stdout().flush().expect("flush failed.");
        let given_user_string: Option<String> = get_user_input();

        inputs.1 = match given_user_string {
            Some(string) => match f64::from_str(&string.trim()) {
                Ok(number) => Some(number),
                Err(_) => None,
            },
            None => None,
        };
    }

    while inputs.2 == None {
        print!("Please provide your operation [+, -, x, /]: ");
        io::stdout().flush().expect("flush failed.");
        let given_user_string: Option<String> = get_user_input();

        inputs.2 = match given_user_string {
            Some(string) => match char::from_str(&string.trim()) {
                Ok(operation) => match operation {
                    '+' | '-' | 'x' | '/' | '*' => Some(operation),
                    _ => None,
                },
                Err(_) => None,
            },
            None => None,
        };
    }

    let calculator_result = match calculate(
        unwrap_option(inputs.0.clone()),
        unwrap_option(inputs.1.clone()),
        unwrap_option(inputs.2.clone()),
    ) {
        Some(data) => data,
        None => panic!("Invalid data receieved by calculator!"),
    };

    println!(
        "{} {} {} = {}",
        unwrap_option(inputs.0.clone()),
        unwrap_option(inputs.2.clone()),
        unwrap_option(inputs.1.clone()),
        calculator_result
    );
}

fn get_user_input() -> Option<String> {
    let mut user_input_string = String::new();

    match io::stdin().read_line(&mut user_input_string) {
        Ok(_) => {
            //println!("user_input_string: {}", user_input_string); TODO: asdf
            return Some(user_input_string);
        }
        Err(_) => {
            //println!("user_input_string: {}", user_input_string); TODO: asdf
            return None;
        }
    }
}

fn unwrap_option<Data: std::fmt::Debug>(input: Option<Data>) -> Data {
    match input {
        Some(data) => return data,
        None => panic!("Data was invalid! input: {:?}", input),
    }
}

fn calculate(number_1: f64, number_2: f64, operation: char) -> Option<f64> {
    let result: Option<f64>;

    match operation {
        'x' | '*' => result = Some(number_1 * number_2),
        '/' => result = Some(number_1 / number_2),
        '+' => result = Some(number_1 + number_2),
        '-' => result = Some(number_1 - number_2),
        _ => result = None,
    }

    if result != None {
        return Some(result?);
    }
    result
}
