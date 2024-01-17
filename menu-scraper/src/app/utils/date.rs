use chrono::{Datelike, Local, Weekday};

pub fn generate_date_with_day_of_week() -> String {
    let current_date = Local::now();
    let day_of_week = match current_date.weekday() {
        Weekday::Sun => "neděle",
        Weekday::Mon => "pondělí",
        Weekday::Tue => "úterý",
        Weekday::Wed => "středa",
        Weekday::Thu => "čtvrtek",
        Weekday::Fri => "pátek",
        Weekday::Sat => "sobota",
    };
    let day_of_month = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    std::format!(
        "Dnes je {} {}. {}. {}",
        day_of_week,
        day_of_month,
        month,
        year
    )
}
