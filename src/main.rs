mod analyze_stuff;
mod data_stuff;
mod draw_stuff;
mod time_stuff;

use crate::analyze_stuff::Analyzer;

fn main() {
    let a = Analyzer::new();
    match a.yearly_habit_performance() {
        Ok(res) => {
            println!("{res}");
        }
        Err(_e) => {
            panic!("yearly failed")
        }
    }
    match a.monthly_habit_performance() {
        Ok(res) => {
            println!("{res}");
        }
        Err(_e) => {
            panic!("monthly failed")
        }
    }
    match a.yearly_objective_performance() {
        Ok(res) => {
            println!("{res}");
        }
        Err(_e) => {
            panic!("yearly failed")
        }
    };

    match a.monthly_objective_performance() {
        Ok(res) => {
            println!("{res}");
        }
        Err(_e) => {
            panic!("monthly failed")
        }
    };
}
