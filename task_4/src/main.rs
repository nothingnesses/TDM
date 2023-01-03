use rand::prelude::*;
use std::io::{self, prelude::*};

/*
Task 1
A program is required to display the times table selected by the user. The user is
asked which table is to be displayed. Then the times table up to 10 is to be
displayed.
1. Obtain the table value from the user.
2. Requires three integer variables: i, table and sum.
3. Use a for statement.
4. Use sum = i * table; as a statement in the for loop.
*/
fn times_table() {
	let stdin = io::stdin();
	let number: u64 = loop {
		print!("Please input a positive integer up to 10: ");
		io::stdout().flush().unwrap();
		if let Ok(number_parsed) =
			stdin.lock().lines().next().unwrap().unwrap().trim().parse::<u64>()
		{
			if number_parsed <= 10 {
				break number_parsed;
			}
		}
		println!("Invalid input.");
	};
	(1..11).for_each(|multiplicand| {
		println!("{}", number * multiplicand);
	});
}

/*
Task 2
A simple program is required to produce a conversion table for miles to Kms.
The table is to cover the range of miles from 10 miles to 100 miles in steps of
10 miles.
1. Suggest the program is saved as “MilestoKms.java”
2. Requires two variables. Determine the name and data type for each one.
3. Use a for statement.
4. Miles can be used as the for loop variable.
The output should appear as follows (Hint use escape characters to format the
output) :
Miles Kms
10 16
20 32
...
100 160
*/
fn miles_to_kilometres_table() {
	println!("Miles Kilometres");
	(10..101).step_by(10).for_each(|mile| {
		println!("{} {}", mile, (mile as f64 * 1.609344) as u32);
	});
}

/*
Task 3
A simple program is required to add up all the heights of ten students. The
application asks the user to input the ten heights. The average height is to be
calculated. Design the solution in pseudo code and then implement it.

### Version 1 that didn't validate inputs
Pseudocode:
```
parse_numbers input =
	for_each parse_as_number (split " " input)

mean values =
	(sum values) / length values

average_heights =
	print "Please input 10 heights in metres, delimited by spaces: "
	print "The average height is {mean (parse_numbers get_input)}"
```

Code:
```rust
fn parse_numbers(delimiter: &str, values: &str) -> Vec<f64> {
	values.split(delimiter).map(|value| value.parse().unwrap()).collect()
}

fn mean(numbers: &[f64]) -> f64 {
	numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn average_heights() {
	print!("Please input 10 heights in metres, delimited by spaces: ");
	io::stdout().flush().unwrap();
	let stdin = io::stdin();
	println!("The average height is {}", mean(&parse_numbers(" ", stdin.lock().lines().next().unwrap().unwrap().trim())));
}
```

### Version 2
Pseudocode:
```
mean values =
	(sum values) / length values

average_heights =
	print "Please input 10 heights in metres, one at a time."
	heights = []
	while (len heights) != 10 {
		if Ok(number) = parse_number get_input {
			push numbers number
		} else {
			print "Please input 10 heights in metres, one at a time."
		}
	}
	print "The average height is {mean (parse_numbers get_input)}"
```
*/
fn mean(numbers: &[f64]) -> f64 {
	numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn average_heights() {
	println!("Please input 10 heights in metres.");
	let stdin = io::stdin();
	let mut heights: Vec<f64> = Vec::with_capacity(10);
	while heights.len() != 10 {
		print!("Please input a number: ");
		io::stdout().flush().unwrap();
		if let Ok(number) = stdin.lock().lines().next().unwrap().unwrap().trim().parse() {
			heights.push(number)
		} else {
			println!("Invalid input.");
		}
	}
	println!("The average height is {}", mean(&heights));
}

/*
Task 4
A program to play a simple guessing game is required. The application is to generate a
random number between 0 and 100. The user then has up to 5 guesses then display a
suitable message. To show whether the user has been successful in guessing the
random number. Design the solution in pseudo code and then implement it.

Pseudocode:
```
guessing_game =
	print "Guess the number in 5 turns or less."
	number = random_in_range 0 101
	guesses = 5
	fail = false
	while guesses > 0 {
		print "Please input a number: "
		if Ok(answer) = parse_as_number get_input {
			if answer == number {
				print "Correct! Your score is: {5 - guesses}"
				fail = false
				break
			} else {
				print "Incorrect."
			}
			--guesses
		} else {
			print "Invalid input."
		}
		print "Guesses remaining: {guesses}"
	}
	if fail {
		print "Game over."
		print "The correct answer was: {number}"
	}
	print "Bye for now!"
```
*/
fn guessing_game() {
	println!("Guess the number in 5 turns or less.");
	let stdin = io::stdin();
	let mut rng = thread_rng();
	let number: u32 = rng.gen_range(0..101);
	let mut guesses = 5u32;
	let mut fail = true;
	println!("{}", number);
	while guesses > 0 {
		print!("Please input a number: ");
		io::stdout().flush().unwrap();
		if let Ok(answer) = stdin.lock().lines().next().unwrap().unwrap().trim().parse::<u32>() {
			if answer == number {
				println!("Correct! Your score is: {}", 5 - guesses);
				fail = false;
				break;
			} else {
				println!("Incorrect.");
			}
			guesses -= 1;
		} else {
			println!("Invalid input.");
		}
		println!("Guesses remaining: {}", guesses);
	}
	if fail {
		println!("Game over.");
		println!("The correct answer was: {}", number);
	}
	println!("Bye for now!");
}

fn main() {
	times_table();
	miles_to_kilometres_table();
	average_heights();
	guessing_game();
}
