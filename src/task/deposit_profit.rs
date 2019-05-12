pub fn depositProfit(deposit: i32, rate: i32, threshold: i32) -> i32 {
    let mut year = 0;
    let pure_rate: f32 = rate as f32 / 100.;
    let mut profit: f32 = deposit as f32;
    while profit < (threshold as f32) {
        year += 1;
        profit += profit * pure_rate;
    }
    year
}