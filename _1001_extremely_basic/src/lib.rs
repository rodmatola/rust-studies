use std::io;

pub fn extremely_basic(a: i32, b: i32) -> String {
	let x = format!("X = {}", a + b);
	x
}

pub fn read_user_input() -> (i32, i32) {
	let mut a = String::new();
	let mut b = String::new();

	io::stdin()
		.read_line(&mut a)
		.expect("please give me correct string number!");
	io::stdin()
		.read_line(&mut b)
		.expect("please give me correct string number!");

	let (a, b) = convert_to_number(a, b);
	(a, b)
}

pub fn convert_to_number(a: String, b: String) -> (i32, i32) {
	let a: i32 = a
		.trim()
		.parse()
		.expect("please give me correct string number!");
	let b: i32 = b
		.trim()
		.parse()
		.expect("please give me correct string number!");
	(a, b)
}
