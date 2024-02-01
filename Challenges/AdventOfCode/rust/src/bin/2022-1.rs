use std::io;
use std::num::IntErrorKind;

use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
	let stdin_lines = io::stdin().lines();

	let mut stdin_stream = stream::iter(stdin_lines).filter_map(|line| match line {
		Ok(line) => Some(line),
		Err(_) => None
	});

	let mut max_so_far = 0;
	let mut current = 0;

	while let Some(value) = stdin_stream.next().await {
		match value.parse::<i32>() {
			Err(err) if err.kind() == &IntErrorKind::Empty => {
				max_so_far = max_so_far.max(current);
				current = 0;
			},
			Err(err) => {
				println!("{:#?}", err);
			}
			Ok(calories) => {
				current += calories
			}
		}
	}

	println!("Maximum calories: {}", max_so_far);
}
