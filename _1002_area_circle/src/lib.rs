use std::io;

fn area(r: f64) -> f64 {
	const PI: f64 = 3.14159;
	PI * r.powi(2)
}

pub fn area_circle(r: f64) -> String {
	let area = area(r);
	let answer = format!("A={:.4}", area);
	answer
}

pub fn read_user_input() -> f64 {
	let mut a = String::new();
	io::stdin()
		.read_line(&mut a)
		.expect("please give me correct string number!");
	let a = convert_to_number(a);
	a
}

pub fn convert_to_number(a: String) -> f64 {
	let a: f64 = a
		.trim()
		.parse()
		.expect("please give me correct string number!");
	a
}
