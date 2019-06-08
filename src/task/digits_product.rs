/*
Given an integer product, find the smallest positive (i.e. greater than 0) integer the product of whose digits is equal to product. If there is no such integer, return -1 instead.

Example

For product = 12, the output should be
digitsProduct(product) = 26;
For product = 19, the output should be
digitsProduct(product) = -1.
*/
pub fn digitsProduct(mut product: i32) -> i32 {
    if product == -1 {
        return -1;
    }
    if product == 0 {
        return 10;
    }
    if (product < 9) && (product > 0) {
        return product;
    }

    let mut deviders: Vec<i32> = Vec::new();

    for factor in (2..10).rev() {
        while product % factor == 0 {
            deviders.push(factor);
            product /= factor;
        }
    }

    if product == 1 {
        return convert_to_digit(deviders);
    }

    -1
}

fn convert_to_digit(vec: Vec<i32>) -> i32 {
    if vec.len() == 0 {
        return -1;
    }

    let mut result = 0;
    let mut multiplexor = 1;

    for x in vec {
        result += multiplexor * x;
        multiplexor *= 10;
    }

    result
}

/*
int digitsProduct(int p) {
    if (p == 0) return 10;
    if (p == 1) return 1;

    int r = 0, e = 1;

    for (int k = 9; k > 1; k--)
        while (!(p % k)) {
            r += k * e;
            p /= k;
            e *= 10;
        }

    return p == 1 ? r : -1;
}
*/