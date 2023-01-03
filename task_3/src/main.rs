use std::io::{self, prelude::*};

/*
While Statement – Task Sheet 2
1. Write a program that asks the user for a password and keeps asking for it until
the word ‘secret’ is entered.
When the correct password has been entered the message ‘You are in!’
should be displayed.
*/
fn password_loop() {
	let stdin = io::stdin();
	loop {
		print!("Enter the password: ");
		io::stdout().flush().unwrap();
		if stdin.lock().lines().next().unwrap().unwrap().trim() == "secret" {
			println!("\nYou are in!");
			break;
		} else {
			println!("Password incorrect. Please try again.");
		}
	}
}

/*
2. Write a program that inputs a succession of numbers until the rogue value ‘-1’ is
entered. The program should then output the highest and lowest of the
numbers. Think carefully about what kind of loop to use. One loop should be
sufficient to enter the values and identify the highest and lowest.
*/
fn min_max() {
	let stdin = io::stdin();
	let mut numbers = vec![];
	let mut number = 0.0;
	while number != -1.0 {
		print!("Input a number: ");
		io::stdout().flush().unwrap();
		if let Ok(number_parsed) = stdin.lock().lines().next().unwrap().unwrap().trim().parse() {
			number = number_parsed;
			numbers.push(number_parsed);
		} else {
			println!("Invalid input. Please input a number.");
		}
	}
	numbers.sort_by(f64::total_cmp);
	println!("Highest number was: {}", numbers.last().unwrap());
	println!("Lowest number was: {}", numbers.first().unwrap());
}

/*
3. A simple program is required to produce a conversion table for Fahrenheit to
centigrade. The table is to cover the range of miles from 10 degrees F to 230
degrees F in steps of 20 degrees.
● Require two variables. Determine the name and data type for each
one.
● Use a while statement.
● The output should appear as follows:
Fahrenheit Centigrade
10 -12
30 -1
*/
fn fahrenheit_to_centigrade_table() {
	println!("Fahrenheit Centigrade");
	(10..231).step_by(20).for_each(|fahrenheit| {
		println!("{} {}", fahrenheit, (fahrenheit - 32) * 5 / 9);
	});
}

fn main() {
	password_loop();
	min_max();
	fahrenheit_to_centigrade_table();
}
