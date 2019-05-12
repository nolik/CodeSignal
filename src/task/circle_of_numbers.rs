/*
Consider integer numbers from 0 to n - 1 written down along the circle in such a way that the
distance between any two neighboring numbers is equal (note that 0 and n - 1 are neighboring, too).

Given n and firstNumber, find the number which is written in the radially opposite position
to firstNumber.
*/
/*
Consider integer numbers from 0 to n - 1 written down along the circle in such a way that the
distance between any two neighboring numbers is equal (note that 0 and n - 1 are neighboring, too).

Given n and firstNumber, find the number which is written in the radially opposite position
to firstNumber.
*/
pub fn circleOfNumbers(n: i32, firstNumber: i32) -> i32 {
    return (firstNumber + n / 2) % n;
}
