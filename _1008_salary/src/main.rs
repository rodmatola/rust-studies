use _1008_salary::calculate_salary;
use _1008_salary::print_salary;
use _1008_salary::read_input;

fn main() {
	let number = read_input();
	let hours = read_input();
	let price = read_input();

	let salary = calculate_salary(hours, price);

	print_salary(number, salary);
}
