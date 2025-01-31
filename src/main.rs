use chrono::Datelike;
use chrono::Local;
use inquire::Select;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get current year
    let current_year = Local::now().year();
    // quarters list
    let quarters = vec![
        peer_review_by_ai::Quarter {
            year: current_year,
            quarter: 1,
            display: format!("{} Q1", current_year),
        },
        peer_review_by_ai::Quarter {
            year: current_year,
            quarter: 2,
            display: format!("{} Q2", current_year),
        },
        peer_review_by_ai::Quarter {
            year: current_year,
            quarter: 3,
            display: format!("{} Q3", current_year),
        },
        peer_review_by_ai::Quarter {
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

            // Read reviews for the selected quarter
            let reviews_by_coworker = peer_review_by_ai::get_reviews_for_quarter(&selected);

            if reviews_by_coworker.is_empty() {
                println!("No reviews found for this quarter.");
            } else {
                println!(
                    "\nFound reviews for {} coworkers",
                    reviews_by_coworker.len()
                );

                // Create review_results directory if it doesn't exist
                fs::create_dir_all("review_results")?;

                for (coworker, reviews) in reviews_by_coworker {
                    println!("\n=== Processing review for {} ===\n", coworker);

                    let prompt = peer_review_by_ai::generate_review_prompt(&coworker, &reviews);
                    match peer_review_by_ai::get_claude_review(&prompt).await {
                        Ok(review) => {
                            // Save to MD file
                            let filename = format!(
                                "review_results/{}_{}_Q{}_review.md",
                                selected.year, coworker, selected.quarter
                            );

                            fs::write(&filename, &review)?;
                            println!("Review saved to: {}", filename);

                            // Print the review content
                            println!("\nReview content for {}:", coworker);
                            println!("{}", review);
                            println!("\n=== End of review for {} ===\n", coworker);
                        }
                        Err(e) => {
                            println!("Error generating review for {}: {}", coworker, e);
                        }
                    }
                }

                println!(
                    "\nAll reviews have been processed and saved to the review_results directory."
                );
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
    Ok(())
}
