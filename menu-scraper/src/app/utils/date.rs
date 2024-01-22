use chrono::{Datelike, NaiveDate, Weekday};

pub fn format_date_with_day_of_week(date: NaiveDate) -> String {
    let day_of_week = match date.weekday() {
        Weekday::Sun => "neděle",
        Weekday::Mon => "pondělí",
        Weekday::Tue => "úterý",
        Weekday::Wed => "středa",
        Weekday::Thu => "čtvrtek",
        Weekday::Fri => "pátek",
        Weekday::Sat => "sobota",
    };
    let day_of_month = date.day();
    let month = date.month();
    let year = date.year();

    std::format!("{} {}. {}. {}", day_of_week, day_of_month, month, year)
}
