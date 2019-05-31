pub fn validTime(time: String) -> bool {
    let timings: Vec<&str> = time.split(':').collect();
    let hours = timings.get(0).unwrap().parse::<i32>().unwrap();
    let minutes = timings.get(1).unwrap().parse::<i32>().unwrap();

    let is_valid_hours = (0 <= hours) && (hours < 24);
    let is_valid_minutes = (0 <= minutes) && (minutes < 60);

    is_valid_hours && is_valid_minutes
}


