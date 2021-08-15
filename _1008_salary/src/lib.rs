use std::io;

pub fn read_input() -> f64 {
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("please give me correct string number!");
    convert_to_number(a)
}

pub fn convert_to_number(a: String) -> f64 {
    let a: f64 = a
        .trim()
        .parse()
        .expect("please give me correct string number!");
    a
}

pub fn calculate_salary(hours: f64, price: f64) -> f64 {
    hours * price
}

pub fn print_salary(number: f64, salary: f64) -> String {
    let output = format!("NUMBER = {}\nSALARY = U$ {:.2}", number, salary);
    println!("{}", output);
    output
}
