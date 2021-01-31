//this is a remake in rust of the formChallenge cpp project
//located at D:\CPP\src\formChallenge.
//variable and project name stylising has been changed from camelCase to snake_case
//to comply with rust's cargo crate syntax.
fn main() {
    //begin var dump
    let mut driveway_dist = 7.4;
    let mut week_count = 2.0;
    let mut two_weeks = 0.0;
    let mut week_per_year = 52.1429;
    let mut distance = 0.0;
    let mut one_year = 0.0;
    let mut eight_years = 0.0;
    let mut accounted_year = 0.0;
    let mut c = 0.0;
    //end var dump
    //why can't there be rust header files :(

    println!("begin solving...");
    two_weeks = driveway_dist * week_count;
    one_year = driveway_dist * week_per_year;
    accounted_year = one_year * 2.0;
    println!("almost finished solving everything!");
    eight_years = accounted_year * 8.0;
    println!("solved final answer! answer is: {}", eight_years);
}
