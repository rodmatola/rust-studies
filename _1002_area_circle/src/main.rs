use _1002_area_circle::area_circle;
use _1002_area_circle::read_user_input;

fn main() {
	let r = read_user_input();
	let area = area_circle(r);
	println!("{}", area);
}
