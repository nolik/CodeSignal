pub fn deleteDigit(digit: i32) -> i32 {
    let mut digits = Digits::new(digit as i32).collect::<Vec<i32>>();
    let mut result = 0;
    let mut mult = 1;
    println!("{:?}", digits);
    let min_item = digits.into_iter().min().unwrap();

    let index = digits.into_iter().position(|x| x == min_item).unwrap();
    digits.remove(index);
    digits.reverse();

    for x in digits {
        result += mult * x;
        mult = mult * 10;
    }

    result
}

struct Digits {
    n: i32,
    divisor: i32,
}

impl Digits {
    fn new(n: i32) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n,
            divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}