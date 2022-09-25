pub trait Money<T> {
    fn new(amount: i32) -> T;
    fn times(&mut self, multiplier: i32) -> T;
    fn equals(&self, money: T) -> bool;
}

#[derive(Debug, PartialEq)]
struct Doller {
    amount: i32,
}

impl Money<Doller> for Doller {
    fn new(amount: i32) -> Doller {
        Doller { amount }
    }
    fn times(&mut self, multiplier: i32) -> Doller {
        return Doller::new(self.amount * multiplier);
    }
    fn equals(&self, doller: Doller) -> bool {
        return self.amount == doller.amount;
    }
}

#[derive(Debug, PartialEq)]
struct Franc {
    amount: i32,
}

impl Money<Franc> for Franc {
    fn new(amount: i32) -> Franc {
        Franc { amount }
    }
    fn times(&mut self, multiplier: i32) -> Franc {
        return Franc::new(self.amount * multiplier);
    }
    fn equals(&self, franc: Franc) -> bool {
        return self.amount == franc.amount;
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
        assert_eq!(Doller::new(10), five.times(2));
        assert_eq!(Doller::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Doller::new(5).equals(Doller::new(5)));
        // falseを評価したい場合はassert!マクロに渡す前に反転させる必要がある
        assert!(!Doller::new(5).equals(Doller::new(6)));
        assert!(Franc::new(5).equals(Franc::new(5)));
        assert!(!Franc::new(5).equals(Franc::new(6)));
    }

    #[test]
    fn test_franc_multiplication() {
        let mut five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
