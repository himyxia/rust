use std::io::Write;
use std::io::FromStr;

fn main() {
	let mut numbers = Vec:new();

	for arg in std::env::args().skip(1) {
		numbers.push(u64::from_str(&arg)
					.expect("error parsing argument"));
	}
	if numbers.len() == 0 {
		writenln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
		std::process::exit(1);
	}
}
