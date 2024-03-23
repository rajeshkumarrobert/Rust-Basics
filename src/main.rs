use time::PrimitiveDateTime as DateTime;

mod clock;
mod guessing_game;
mod learn_tuple;
mod reverse_string;
mod gigasecond;

fn main() {
    println!("MY first Game");
    sample();
    clock::Clock::new(25, 0);
    reverse_string::reverse("Hello");
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    gigasecond::after(start_date);
}

#[allow(dead_code)]
fn sample() {
    guessing_game::guessed();
    learn_tuple::tuple_check();
}

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
