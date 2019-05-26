pub fn deleteDigit(digit: i32) -> i32 {
    let digits = Digits::new(digit).collect::<Vec<i32>>();
    let mut result_vec;

    println!("{:?}", digits);
    let min_item = digits.iter().min().unwrap();
    let index = digits.iter().position(|x| x == min_item).unwrap();
    result_vec = digits.clone();
    result_vec.remove(index);
    result_vec.reverse();

    to_sum_digit(&result_vec)
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

fn to_sum_digit(digits: &Vec<i32>) -> i32 {
    let mut sum_digit = 0;
    let mut mult = 1;

    for x in digits {
        sum_digit += mult * x;
        mult = mult * 10;
    }

    return sum_digit;
}