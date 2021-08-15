fn area(r: f64) -> f64 {
	const PI: f64 = 3.14159;
	PI * r.powi(2)
}

pub fn area_circle(r: f64) -> String {
	let area = area(r);
	let answer = format!("A={:.4}", area);
	answer
}
