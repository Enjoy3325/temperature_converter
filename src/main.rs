use std::io;
// use std::num::{ParseIntError, ParseFloatError};

const C: f32 = 32.0;

fn c_to_f(chelsius_temp: f32) -> f32 {
    (chelsius_temp * (9.0 / 5.0)) + C
}

fn f_to_c(faringate_temp: f32) -> f32 {
    (faringate_temp - C) * (5.0 / 9.0)
}

fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

fn main() {
    println!("Temperature converter. \n (1) C to F \n (2) F to C");

    // Ввод выбора пользователя
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap(); // Чтение выбора
    let number_choice: u8 = match user_choice.trim().parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            println!("Please type a valid number for the choice.");
            return;
        }
    };


    if number_choice != 1 && number_choice != 2 {
        println!("Unknown conversion requested!");
        return; // Если не 1 и не 2, выходим из программы
    }

    // Ввод температуры
    println!("Enter temperature:");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).unwrap(); // Чтение температуры
    let temperature: f32 = match temperature.trim().parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            println!("Please type a valid number for temperature.");
            return;
        }
    };

    // Вызов функции для конвертации
    match convert(temperature, number_choice) {
        Some(result) => println!("The result of conversion is: {}", result),
        None => println!("Unknown conversion requested!"),
    };
}
