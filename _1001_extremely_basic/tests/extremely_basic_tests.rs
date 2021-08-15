use _1001_extremely_basic::convert_to_number;
use _1001_extremely_basic::extremely_basic as basic;

#[test]
fn input_1() {
	assert_eq!("X = 19", basic(10, 9))
}

#[test]
fn input_2() {
	assert_eq!("X = -6", basic(-10, 4))
}

#[test]
fn input_3() {
	assert_eq!("X = 22", basic(15, 7))
}

#[test]
#[should_panic]
fn not_number() {
	let a = String::from("-");
	let b = String::from("+");
	let (a, b) = convert_to_number(a, b);
	basic(a, b);
}
