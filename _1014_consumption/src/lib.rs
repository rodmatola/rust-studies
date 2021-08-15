use std::io;

pub fn read_input() -> f64 {
	let mut a = String::new();

	io::stdin()
		.read_line(&mut a)
		.expect("please give me correct string number!");

	convert_to_number(a)
}

fn convert_to_number(a: String) -> f64 {
	let a: f64 = a
		.trim()
		.parse()
		.expect("please give me correct string number!");
	a
}

fn average_consumption(dist: f64, gas: f64) -> f64 {
	dist / gas
}

pub fn print_average_consumption(dist: f64, gas: f64) -> String {
	let average = average_consumption(dist, gas);
	let output = format!("{:.3} km/l", average);
	println!("{}", output);
	output
}
