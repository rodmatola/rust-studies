use _1002_area_circle::area_circle;

#[test]
fn input1() {
	assert_eq!(area_circle(2.00), "A=12.5664");
}

#[test]
fn input2() {
	assert_eq!(area_circle(100.64), "A=31819.3103");
}

#[test]
fn input3() {
	assert_eq!(area_circle(150.00), "A=70685.7750");
}
