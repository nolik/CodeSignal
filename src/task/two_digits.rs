pub fn addTwoDigits(digit: i32) -> i32 {
    DigitIter(digit).sum::<i32>()
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