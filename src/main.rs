// Very naive year calculation, ignores leap years etc.
fn get_current_year(timestamp: i32) -> i32 {
    const START_YEAR: i32 = 1970;
    let timestamp_in_years = timestamp / 60 / 60 / 24 / 365;

    START_YEAR + timestamp_in_years
}

fn main() {
    let max_timestamp: i32 = i32::MAX; // 19.11.2038 in unix
    let past_max_timestamp = max_timestamp.wrapping_add(1); // Second past max

    let max_date = get_current_year(max_timestamp);
    let past_max_date = get_current_year(past_max_timestamp);

    println!("----------------------------------------------------------------");
    println!("Should print year from max timestamp. Real value should be 2038");
    println!("Max timestamp: {}", max_date); 
    println!("----------------------------------------------------------------");
    println!("Should print year from max timestamp + 1 second. Logically it should still be 2038");
    println!("Past max timestamp: {}", past_max_date);
    println!("Due to integer overflow, we're in 1900s. With proper date calc it would be 1901, but less-than-precise check brings us to 1902.");
    println!("Still far cry from 2038, which would be the 'logical' value");
    println!("----------------------------------------------------------------");
}
