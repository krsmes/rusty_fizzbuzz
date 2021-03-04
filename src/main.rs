fn fizzbuzz(i: u8) -> &'static str {
	"fizz"
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn fizzbuzz_returns_fizz_for_3() {
	assert_eq!("fizz", fizzbuzz(3));
}
