//fn main() {
//    let s = [1,5,3,4,8];
//    println!("{:?}", almost_increasing_sequence(s.to_vec()));
//}

fn almost_increasing_sequence(array: Vec<i32>) -> bool {
    let mut prev = std::i32::MIN;
    let before_prev = std::i32::MIN;
    let allow_exceptions = true;

    for curr in &array {
        // Is order not maintained?
        if curr <= prev {
            // Give up when this is not the first exception
            if !allowExceptions {
                return false;
            }

            allowExceptions = false;

            // Decide whether to skip the current or previous value
            if curr > beforePrev {
                prev = curr;
            }
        } else {
            beforePrev = prev;
            prev = curr;
        }
    }

    true
}


/*
function almostIncreasingSequence(sequence) {
var prev = -Infinity,
beforePrev = -Infinity,
allowExceptions = true;

for (var curr of sequence) {
// Is order not maintained?
if (curr <= prev) {
// Give up when this is not the first exception
if (!allowExceptions) return false;
allowExceptions = false;
// Decide whether to skip the current or previous value
if (curr > beforePrev) prev = curr;
} else { // Normal case: keep track of two preceding values
beforePrev = prev;
prev = curr;
}
}
return true;
}*/
