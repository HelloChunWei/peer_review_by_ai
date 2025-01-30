use chrono::Local;
use chrono::{Datelike, NaiveDate};
use inquire::Select;
use std::fmt;
use std::fs;
use std::path::Path;

// Debug trait - let me can println!("{:?}", Quarter)
#[derive(Debug)]
struct Quarter {
    year: i32,
    quarter: u8,
    display: String,
}

impl Quarter {
    // Check if the given date is within this quarter
    fn contains_date(&self, date: NaiveDate) -> bool {
        if date.year() != self.year {
            return false;
        }
        let quarter = (date.month() - 1) / 3 + 1;
        quarter == self.quarter as u32
    }
}
// it is different from Java, in Java we declare propertys and methods in Class
// but in the rust, we declare property in struct and implement methods in impl

// implement print methods
impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

// // Debug trait - let me can println!("{:?}", Review)
#[derive(Debug)]
struct Review {
    date: NaiveDate,
    coworker: String,
    content: String,
}
// file format - yyyy-mm-dd-cowork-coworker
fn read_review_from_file(path: &Path) -> Option<Review> {
    // Get date and coworker name from filename
    let file_name = path.file_stem()?.to_str()?;
    let parts: Vec<&str> = file_name.split('-').collect();

    // Check if the filename format is correct
    if parts.len() < 5 || parts[3] != "cowork" {
        return None;
    }

    // Parse date
    let date_str = format!("{}-{}-{}", parts[0], parts[1], parts[2]);
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()?;

    // get file content
    let content = fs::read_to_string(path).ok()?;

    Some(Review {
        date,
        coworker: parts[4].to_string(),
        content,
    })
}

// Filter reviews by quarter and skip others
// Returns a vector of reviews that match the given quarter
fn get_reviews_for_quarter(quarter: &Quarter) -> Vec<Review> {
    let mut reviews = Vec::new();

    // Read reviews directory
    let reviews_dir = Path::new("reviews");
    if !reviews_dir.exists() {
        println!("Reviews directory not found!");
        return reviews;
    }

    // Iterate through all files in the directory
    if let Ok(entries) = fs::read_dir(reviews_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check if it's a .md file
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    // Try to read the review
                    if let Some(review) = read_review_from_file(&path) {
                        // Check if the date is within the selected quarter
                        if quarter.contains_date(review.date) {
                            reviews.push(review);
                        }
                    }
                }
            }
        }
    }

    // Sort by date
    reviews.sort_by_key(|r| r.date);
    reviews
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

            // 讀取該季度的評論
            let reviews = get_reviews_for_quarter(&selected);

            if reviews.is_empty() {
                println!("No reviews found for this quarter.");
            } else {
                println!("\nFound {} reviews:", reviews.len());
                for review in reviews {
                    println!("\n--- {} - {} ---", review.date, review.coworker);
                    println!("{}", review.content);
                }
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
