use chrono::Local;
use chrono::{Datelike, NaiveDate};
use inquire::Select;
use std::collections::HashMap;
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
fn get_reviews_for_quarter(quarter: &Quarter) -> HashMap<String, Vec<Review>> {
    let mut reviews_by_coworker: HashMap<String, Vec<Review>> = HashMap::new();

    let reviews_dir = Path::new("reviews");
    if !reviews_dir.exists() {
        println!("Reviews directory not found!");
        return reviews_by_coworker;
    }

    if let Ok(entries) = fs::read_dir(reviews_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(review) = read_review_from_file(&path) {
                        //  unlike other language
                        /*
                        // for example java:
                            if (map.containsKey("Alex")) {
                                map.get("Alex").add(review);
                            } else {
                                List<Review> list = new ArrayList<>();
                                list.add(review);
                                map.put("Alex", list);
                            }
                         */
                        // but we don't do that in rust
                        if quarter.contains_date(review.date) {
                            reviews_by_coworker
                                .entry(review.coworker.clone()) // get key name
                                .or_insert_with(Vec::new) // if key is not exist, create Vec
                                .push(review);
                        }
                    }
                }
            }
        }
    }

    // Sort reviews by date for each coworker
    for reviews in reviews_by_coworker.values_mut() {
        reviews.sort_by_key(|r| r.date);
    }

    reviews_by_coworker
}

fn generate_review_prompt(coworker: &str, reviews: &Vec<Review>) -> String {
    let reviews_text = reviews
        .iter()
        .map(|r| format!("Date: {}\nContent:\n{}\n---\n", r.date, r.content))
        .collect::<String>();
    // r# mean raw string
    format!(
        r#"Based on the following work logs for {}, please provide a comprehensive quarterly review. For each category, provide a rating and detailed explanation with specific examples from the work logs:

1. Working Frequency
Rate how closely you worked with {} this quarter:
- Never
- Almost never
- Sometimes
- Frequently
- Daily

2. Core Competencies (Rate each as: Unsatisfactory/Improvement needed/Meets expectations/Exceeds expectations/Truly exceptional/Not Applicable)

Work Quality:
- Accuracy and thoroughness
- Productivity and competence
- Detailed explanation required

Problem Solving:
- Reasoning and analysis capabilities
- Solution identification
- Willingness to tackle problems
- Acceptance of new responsibilities

Work Independence and Autonomy:
- Quality of work with minimal guidance
- Self-sufficiency in delivery
- Independence from management oversight

Attitude:
- Respect for others
- Initiative taking
- Handling mistakes and criticism
- Active listening

Leadership:
- Emerging leadership skills
- Project leadership
- Proactiveness
- Personal ownership of results

Teamwork:
- Collaboration within and across teams
- Relationship maintenance with colleagues

Communication:
- Clarity and timeliness
- Transparency in work progress
- Information sharing with team members

Engagement:
- Participation in discussions
- Solution proposal
- Constructive disagreement when needed

Company Goals:
- Contribution to annual company objectives
- Alignment with organizational aims

Security:
- Adherence to Information Security Policy
- Security practice implementation
- Computer locking when away from desk

Professional Etiquette:
- Meeting punctuality
- Compliance with company policies
- Professional conduct

3. Managerial Assessment (if applicable)
For each item, rate as: Unsatisfactory/Improvement needed/Meets expectations/Exceeds expectations/Truly exceptional/Not Applicable

Mentoring:
- Support and guidance for junior team members

Management Skills:
- Career growth promotion
- Challenge assignment
- Mentoring of other leads
- Effective delegation

Team Culture:
- Fostering positive environment
- Supporting collaboration

Industry Visibility:
- Conference participation
- Public speaking
- Blog post writing
- Meetup hosting

Vision Communication:
- Company vision understanding and explanation
- Strategic direction setting
- Goal alignment
- Operating Principles promotion

4. Strengths and Continuity
Provide specific examples of:
- Notable achievements this quarter
- Successful projects
- Positive behaviors to continue
- Areas of excellence

5. Areas for Improvement
Specify which areas need development (select and explain):
A. Communication
B. Work quality
C. Teamwork
D. Proactiveness
E. Accountability
F. Attention to detail
G. Other

6. Additional Comments
Provide any other relevant feedback or suggestions for {}'s development.

Work logs for review:

{}"#,
        coworker, coworker, coworker, reviews_text
    )
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

            // Read reviews for the selected quarter
            let reviews_by_coworker = get_reviews_for_quarter(&selected);

            if reviews_by_coworker.is_empty() {
                println!("No reviews found for this quarter.");
            } else {
                println!(
                    "\nFound reviews for {} coworkers:",
                    reviews_by_coworker.len()
                );
                for (coworker, reviews) in reviews_by_coworker {
                    println!("\n=== Generated prompt for {} ===\n", coworker);
                    let prompt = generate_review_prompt(&coworker, &reviews);
                    println!("{}", prompt);
                    println!("\n=== End of prompt for {} ===\n", coworker);
                }
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
