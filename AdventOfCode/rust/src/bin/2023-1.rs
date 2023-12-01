use std::io;

fn main() {
	let lines = io::stdin().lines().map(|line| line.unwrap());
	let ints = lines.map(|line| {
		let digits: Vec<_> = line.chars().filter(|char| char.is_digit(10)).collect();
		let result = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
		result.parse::<i32>().unwrap()
	});
	let result: i32 = ints.sum();

	println!("{:#?}", result);
}
