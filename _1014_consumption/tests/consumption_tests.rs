use _1014_consumption::print_average_consumption;

#[test]
fn input1() {
	let dist = 500.;
	let gas = 35.0;

	let output = "14.286 km/l";
	assert_eq!(print_average_consumption(dist, gas), output);
}

#[test]
fn input2() {
	let dist = 2254.;
	let gas = 124.4;

	let output = "18.119 km/l";
	assert_eq!(print_average_consumption(dist, gas), output);
}

#[test]
fn input3() {
	let dist = 4554.;
	let gas = 464.6;

	let output = "9.802 km/l";
	assert_eq!(print_average_consumption(dist, gas), output);
}
