use std::io;

fn recover(input: &String) -> i32 {
		let digits: Vec<_> = input.chars().filter(|char| char.is_digit(10)).collect();
		let result = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
		result.parse::<i32>().unwrap()
}

fn replacement(input: &String) -> String {
	input
		.replace("one", "1")
		.replace("two", "2")
		.replace("three", "3")
		.replace("four", "4")
		.replace("five", "5")
		.replace("six", "6")
		.replace("seven", "7")
		.replace("eight", "8")
		.replace("nine", "9")
}


fn main() {
	let lines: Vec<_> = io::stdin().lines().map(|line| line.unwrap()).collect();

	let part1: i32 = lines.iter().map(recover).sum();
	println!("Part 1: {}", part1);

	let part2: i32 = lines.iter().map(|line| {
		// TODO: oneight, eightwo, etc seems to throw a wrench at this approach
		replacement(line)
	}).map(|line| recover(&line)).sum();
	println!("Part 2: {}", part2);
}
