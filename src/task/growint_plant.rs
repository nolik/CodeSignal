pub fn growingPlant(upSpeed: i32, downSpeed: i32, desiredHeight: i32) -> i32 {
    let mut height = 0;
    let mut iter = 0;
    while height < desiredHeight {
        iter += 1;
        height += upSpeed;
        if height >= desiredHeight {
            break;
        } else {
            height -= downSpeed;
        }
    }

    iter
}
