struct Doller {
    amount: i32,
}

impl Doller {
    fn new(amount: i32) -> Doller {
        Doller { amount }
    }
    fn times(&mut self, multiplier: i32) -> Doller {
        return Doller::new(self.amount * multiplier);
    }
}

fn main() {
    println!("{}", 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let mut five = Doller::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
