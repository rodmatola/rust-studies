use _1001_extremely_basic::extremely_basic as basic;
use _1001_extremely_basic::read_user_input;

fn main() {
	println!("Enter 2 numbers:");
	let (a, b) = read_user_input();
	println!("{}", basic(a, b));
}
