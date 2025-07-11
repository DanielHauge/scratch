// tokio

use tokio;

#[tokio::main]
async fn main() {
    use leap_seconds::LeapSecondsList;
    use std::io::BufReader;

    // ======= fetching & parsing the file ======= //

    // get the file from the IERS
    let file = reqwest::get("https://hpiers.obspm.fr/iers/bul/bulc/ntp/leap-seconds.list")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // parse the file
    // create a `BufReader` from the string
    let file = file.as_bytes();
    let leap_seconds_list = LeapSecondsList::new(BufReader::new(file)).unwrap();

    // make sure the file is up to date
    // you should always do this unless you don't mind working with outdated data
    assert!(!leap_seconds_list.is_expired());

    // ======= some things that are possible ======= //

    // get the next leap second that will be introduced
    let next_leap_second = leap_seconds_list.next_leap_second();
    println!(
        "Next leap second: {:?}",
        next_leap_second.map(|ls| ls.timestamp())
    );

    // get an ordered slice of all future leap seconds currently announced
    let future_leap_seconds = leap_seconds_list.planned_leap_seconds();
    println!(
        "Next leap second: {:?}",
        next_leap_second.map(|ls| ls.timestamp())
    );

    // get an ordered slice of all leap seconds that have been introduced since 1970
    let all_leap_seconds = leap_seconds_list.leap_seconds();
    println!("All leap seconds since 1970: {:?}", all_leap_seconds);

    // get the last time the `leap-seconds.list` file was updated
    let last_update = leap_seconds_list.last_update();
    println!("Last update: {}", last_update);
}
