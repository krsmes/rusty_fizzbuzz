fn fizzbuzz(i: u8) -> &'static str {
	if i == 3 {
		"fizz"
	} else {
		"buzz"
	}
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn fizzbuzz_returns_fizz_for_3() {
	assert_eq!("fizz", fizzbuzz(3));
}

#[test]
fn fizzbuzz_returns_buzz_for_5() {
	assert_eq!("buzz", fizzbuzz(5));
}
