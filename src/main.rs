//variable and project name stylising has been changed from camelCase to snake_case
//to comply with rust's cargo crate syntax.
fn main() {
    //begin var dump
    let driveway_dist = 7.4;
    let week_count = 2.0;
    let week_per_year = 52.1429;
    //end var dump
    //why can't there be rust header files :(

    println!("begin solving...");
    let _two_weeks = driveway_dist * week_count;
    let one_year = driveway_dist * week_per_year;
    let accounted_year = one_year * 2.0;
    println!("almost finished solving everything!");
    let eight_years = accounted_year * 8.0;
    println!("solved final answer! answer is: {}", eight_years);
}
