use _1008_salary::calculate_salary;
use _1008_salary::print_salary;

#[test]
fn input1() {
	let number = 25.;
	let hours = 100.;
	let price: f64 = 5.50;

	let salary = calculate_salary(hours, price);

	let output = "NUMBER = 25\nSALARY = U$ 550.00";

	assert_eq!(print_salary(number, salary), output);
}

#[test]
fn input2() {
	let number = 1.;
	let hours = 200.;
	let price: f64 = 20.50;

	let salary = calculate_salary(hours, price);

	let output = "NUMBER = 1\nSALARY = U$ 4100.00";

	assert_eq!(print_salary(number, salary), output);
}

#[test]
fn input3() {
	let number = 6.;
	let hours = 145.;
	let price: f64 = 15.55;

	let salary = calculate_salary(hours, price);

	let output = "NUMBER = 6\nSALARY = U$ 2254.75";

	assert_eq!(print_salary(number, salary), output);
}
