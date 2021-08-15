use _1014_consumption::print_average_consumption;
use _1014_consumption::read_input;

fn main() {
    println!("What is the total distance?");
    let dist = read_input();

    println!("What is the total gas consumption?");
    let gas = read_input();

    print_average_consumption(dist, gas);
}
