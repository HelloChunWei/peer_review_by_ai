use chrono::Datelike;
use chrono::Local;
use inquire::Select;
use std::fmt;

#[derive(Debug)]
struct Quarter {
    year: i32,
    quarter: u8,
    display: String,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

fn main() {
    // get current year
    let current_year = Local::now().year();
    // quarters list
    let quarters = vec![
        Quarter {
            year: current_year,
            quarter: 1,
            display: format!("{} Q1", current_year),
        },
        Quarter {
            year: current_year,
            quarter: 2,
            display: format!("{} Q2", current_year),
        },
        Quarter {
            year: current_year,
            quarter: 3,
            display: format!("{} Q3", current_year),
        },
        Quarter {
            year: current_year,
            quarter: 4,
            display: format!("{} Q4", current_year),
        },
    ];
    // create selection list
    let ans = Select::new("Which quarter would you like to review?", quarters)
        .with_formatter(&|q| q.to_string())
        .prompt();

    match ans {
        Ok(selected) => {
            println!("Selected: {} Q{}", selected.year, selected.quarter);
            // TODO
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
