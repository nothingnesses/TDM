use std::io::{self, prelude::*};

/*
Task 1
Design the algorithm for the above process using pseudo code.

Hint: Initially determine the input data and the output data. Then consider the process (instructions/calculations) required to obtain the output data from the input data.

Algorithm:
1. Ask user if peak or off-peak membership is desired.
2. Parse and store input.
3. Ask user if swimming pool access is desired.
4. Parse and store input.
5. Ask user if tennis court access is desired.
6. Parse and store input.
7. Calculate and display total cost.

Pseudocode:
```
parse_yes_no input =
	match (lower input) {
		"y" = true
		"n" = false
		_ = panic
	}

calculate_total is_peak pool_access court_access =
	(is_peak ? 30 : 20) + (pool_access ? 10 : 0) + (court_access ? 10 : 0)

gym_membership_calculator =
	print "Costs per month:
Peak membership: £30
Off-peak membership: £20
Swimming pool access: +£10
Tennis court access: +£10
"
	print "Do you want a peak membership (as opposed to an off-peak membership)? Input y/n: "
	is_peak = parse_yes_no get_input
	print "Do you want access to the swimming pool? Input y/n: "
	pool_access = parse_yes_no get_input
	print "Do you want access to the tennis court? Input y/n: "
	court_access = parse_yes_no get_input
	print "Total cost per month is: {calculate_total is_peak pool_access court_access}"
```
*/

/*
Task 2
Implement your algorithm i.e. build the program in Java.
*/

fn parse_yes_no(input: &str) -> bool {
	match input.to_lowercase().as_str() {
		"y" => true,
		"n" => false,
		_ => panic!("Invalid input: `{}`.\n Valid inputs are: y, n", input),
	}
}

fn calculate_total(is_peak: bool, pool_access: bool, court_access: bool) -> u64 {
	(if is_peak { 30 } else { 20 })
		+ (if pool_access { 10 } else { 0 })
		+ (if court_access { 10 } else { 0 })
}

fn gym_membership_calculator() {
	println!(
		"{}",
		"Costs per month:
Peak membership: £30
Off-peak membership: £20
Swimming pool access: +£10
Tennis court access: +£10
"
	);
	let stdin = io::stdin();
	print!("Do you want a peak membership (as opposed to an off-peak membership)? Input y/n: ");
	io::stdout().flush().unwrap();
	let is_peak: bool = parse_yes_no(stdin.lock().lines().next().unwrap().unwrap().trim());
	print!("Do you want access to the swimming pool? Input y/n: ");
	io::stdout().flush().unwrap();
	let pool_access: bool = parse_yes_no(stdin.lock().lines().next().unwrap().unwrap().trim());
	print!("Do you want access to the tennis court? Input y/n: ");
	io::stdout().flush().unwrap();
	let court_access: bool = parse_yes_no(stdin.lock().lines().next().unwrap().unwrap().trim());
	println!("Total cost per month is: {}", calculate_total(is_peak, pool_access, court_access));
}

fn main() {
	gym_membership_calculator();
}
