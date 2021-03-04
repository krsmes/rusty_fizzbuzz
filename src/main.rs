
fn fizzbuzz(i: u8) -> String {
	if i % 15 == 0 {
		"fizzbuzz".to_string()
	} else if i % 3 == 0 {
		"fizz".to_string()
	} else if i % 5 == 0  {
		"buzz".to_string()
	} else {
		i.to_string()
	}
}

fn main() {
    println!("Hello, world!");
    for val in 1..=100 {
    	println!("{}", fizzbuzz(val));
    }
}

#[test]
fn fizzbuzz_returns_fizz_for_divisible_by_3() {
	for val in vec![3,6,9,12].iter() {
		assert_eq!("fizz", fizzbuzz(*val));
	}
}

#[test]
fn fizzbuzz_returns_buzz_for_divisible_by_5() {
	for val in vec![5,10].iter() {
		assert_eq!("buzz", fizzbuzz(*val));
	}
}

#[test]
fn fizzbuzz_returns_buzz_for_divisible_by_3_and_5() {
	for val in vec![15,30].iter() {
		assert_eq!("fizzbuzz", fizzbuzz(*val));
	}
}
#[test]
fn fizzbuzz_returns_1_for_1() {
	assert_eq!("1", fizzbuzz(1));
}
#[test]
fn fizzbuzz_returns_2_for_2() {
	assert_eq!("2", fizzbuzz(2));
}
