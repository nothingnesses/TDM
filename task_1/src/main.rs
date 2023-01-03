use std::io::{self, prelude::*};

/*
A. Company Profit Calculator
- Create a variable and read in from the keyboard and store the revenue
- Create a variable and read in from the keyboard and store the costs
- Create a profit variable which stores the result of revenue-costs
- Display the value of profit in the console.
*/
fn company_profit_calculator() {
	let stdin = io::stdin();
	print!("Please input revenue: ");
	io::stdout().flush().unwrap();
	let revenue: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	print!("Please input costs: ");
	io::stdout().flush().unwrap();
	let costs: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	let profit = revenue - costs;
	println!("Profit was: {}", profit);
}

/*
B. Stock Purchasing System
- Create a variable to store cost per share read in from the keyboard
- Create a variable to store the number of shares read in from the keyboard
- Create a total cost variable which stores the result of cost per share * number of shares
- Display the total cost in the console.
*/
fn stock_purchasing_system() {
	let stdin = io::stdin();
	print!("Please input cost per share: ");
	io::stdout().flush().unwrap();
	let cost_per_share: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	print!("Please input number of shares: ");
	io::stdout().flush().unwrap();
	let shares_amount: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	let total_cost = cost_per_share * shares_amount;
	println!("Total cost was: {}", total_cost);
}

/*
C. Discount System
- Create a variable to store the variable price read in from the keyboard
- Create a variable to store the discount read in from the keyboard
- Create a variable to store the mark down price (discount / 100) * price.
- Decrease the price, by the mark down price.
- Display the original price and the new price in the console.
*/
fn discount_system() {
	let stdin = io::stdin();
	print!("Please input price: ");
	io::stdout().flush().unwrap();
	let price: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	print!("Please input discount: ");
	io::stdout().flush().unwrap();
	let discount: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	let mark_down_price = discount * price / 100.0;
	println!("Original price was: {}", price);
	println!("New price is: {}", mark_down_price);
}

/*
D. Farm Land Calculator
- One acre of land on a farm can produce 18 tons of corn. Create a variable to store the area of the
land. Calculate the amount of corn that may be produced on the land. Output your result in the
console.
*/
fn farm_land_calculator() {
	const TONS_OF_CORN_PER_ACRE: f64 = 18.0;
	let stdin = io::stdin();
	print!("Please input farm land area in acres: ");
	io::stdout().flush().unwrap();
	let area: f64 = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
	let tons_of_corn = TONS_OF_CORN_PER_ACRE * area;
	println!("Mass of corn produced in tons: {}", tons_of_corn);
}

fn main() {
	company_profit_calculator();
	stock_purchasing_system();
	discount_system();
	farm_land_calculator();
}
