/*
For n = 5, the output should be
digitDegree(n) = 0;
For n = 100, the output should be
digitDegree(n) = 1.
1 + 0 + 0 = 1.
For n = 91, the output should be
digitDegree(n) = 2.
9 + 1 = 10 -> 1 + 0 = 1.
*/
pub fn digitDegree(n: i32) -> i32 {
    let mut counter = 0;

    flatter_digit(n, counter)
}

fn flatter_digit(digit: i32, mut counter: i32) -> i32 {
    println!("counter= {:?}", counter);
    println!("digit= {:?}", digit);
    if DigitIter(digit).count() > 1 {
        let temp = DigitIter(digit).sum::<i32>();
        counter += 1;
        counter = flatter_digit(temp, counter)
    }
    counter
}

struct DigitIter(i32);

impl Iterator for DigitIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % 10;
            self.0 /= 10;
            Some(ret)
        }
    }
}
