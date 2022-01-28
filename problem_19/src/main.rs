use chrono::{Datelike, NaiveDate};

fn main() {
    // the first sunday
    let start = NaiveDate::from_ymd(1901, 1, 6);
    let end = NaiveDate::from_ymd(2000, 12, 31);

    let res = start
        .iter_weeks()
        .take_while(|date| date <= &end)
        .filter(|sunday| sunday.day() == 1)
        .count();

    println!(
        "There is {} sunday that fall on the first of the month between {} and {}",
        res, start, end
    );
}
