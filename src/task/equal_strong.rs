pub fn areEquallyStrong(
    yourLeft: i32,
    yourRight: i32,
    friendsLeft: i32,
    friendsRight: i32,
) -> bool {
    let left_delta = yourLeft - friendsLeft;
    let right_delta = yourRight - friendsRight;

    pub fn areEquallyStrong(
        yourLeft: i32,
        yourRight: i32,
        friendsLeft: i32,
        friendsRight: i32,
    ) -> bool {
        let left_delta = yourLeft - friendsLeft;
        let right_delta = yourRight - friendsRight;

        if (left_delta + right_delta) == 0 {
            if left_delta == right_delta {
                return true;
            }

            if left_delta > 0 {
                return friendsRight == yourLeft;
            } else {
                return friendsLeft == yourRight;
            }
        } else {
            false
        }
    }
}
