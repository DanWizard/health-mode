use chrono::{Datelike, Local, NaiveDate};
use std::str::FromStr;

use crate::data_stuff::HabitDayPerformance;

#[derive(Debug)]
pub struct CurrentDate {
    pub year: String,  // YYYY
    pub month: String, // Full month name, first letter capitalized
    pub day: String,   // DD
}

impl CurrentDate {
    /// Creates a new FilePath with the current date
    pub fn new() -> Self {
        let now = Local::now();
        let year = now.year().to_string();

        // Get full month name with first letter capitalized
        let month = match now.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        }
        .to_string();

        // Format day with leading zero if needed
        let day = format!("{:02}", now.day());

        CurrentDate { year, month, day }
    }
}

fn month_name_to_num(month_name: &str) -> u32 {
    let month_number = match month_name {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => panic!("Invalid month name: {}", month_name),
    };
    month_number
}

/// Returns an array of days for the given month, accounting for leap years
pub fn days_in_month(month_name: &str) -> Vec<u32> {
    let now = Local::now();
    let year = now.year();

    // Parse month name to month number
    let month_number = month_name_to_num(month_name);

    // Get the last day of the month
    let days_in_month = if month_number == 12 {
        // For December, get days by creating Jan 1 of next year and going back 1 day
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    } else {
        // For other months, get days by creating 1st of next month and going back 1 day
        NaiveDate::from_ymd_opt(year, month_number + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    };

    // Create and return the array of days
    (1..=days_in_month).collect()
}

pub fn day_of_year(performance: &HabitDayPerformance) -> u32 {
    // Parse the year, month, and day from strings
    let year: u32 = performance.year.parse().unwrap();
    let day: u32 = performance.day.parse().unwrap();

    // Convert month name to month number (1-12)

    let month_number = month_name_to_num(&performance.month);

    // println!("day {}, month {}, year {}", day, month_number, year);

    // Days in each month (non-leap year)
    let days_in_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // Check if it's a leap year
    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    // Calculate day of year
    let mut day_of_year = day;
    for m in 1..month_number {
        day_of_year += days_in_month[m as usize];
    }

    // Add leap day if it's a leap year and we're past February
    if is_leap_year && month_number > 2 {
        day_of_year += 1;
    }

    day_of_year
}
