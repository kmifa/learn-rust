fn main() {
    for n in 0..100 {
        println!("{}", fizzbuzz(n));
    }
}

fn fizzbuzz(num: i32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        return "FizzBuzz".to_string();
    } else if num % 3 == 0 {
        return "Fizz".to_string();
    } else if num % 5 == 0 {
        return "Buzz".to_string();
    } else {
        return num.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _1を渡すと文字列1を返す() {
        assert_eq!("1", fizzbuzz(1));
    }

    #[test]
    fn _3を渡すと文字列Fizzを返す() {
        assert_eq!("Fizz", fizzbuzz(3));
    }

    #[test]
    fn _5を渡すと文字列Buzzを返す() {
        assert_eq!("Buzz", fizzbuzz(5));
    }

    #[test]
    fn _15を渡すと文字列FizzBuzzを返す() {
        assert_eq!("FizzBuzz", fizzbuzz(15));
    }
}
