mod days;
mod utils;

use std::error::Error;

use utils::*;

const AOC_YEAR: u16 = 2025;
const AOC_FIRST_DAY: u8 = 1;
const AOC_LAST_DAY: u8 = 12;
const X_PADDING: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    display_banner(&format!("Advent of Code {}", AOC_YEAR), X_PADDING, true);
    if dotenvy::dotenv().is_err() {
        println!(
            "No .env file detected. AOC_SESSION variable may not be set and auto-fetching puzzle inputs is not possible!"
        )
    }

    if let Ok(day) = get_day() {
        display_banner(
            &format!("Advent of Code {} - Day {}", AOC_YEAR, day),
            X_PADDING,
            true,
        );
        return solve_day(day).await;
    }

    for day in AOC_FIRST_DAY..=AOC_LAST_DAY {
        display_banner(
            &format!("Advent of Code {} - Day {}", AOC_YEAR, day),
            X_PADDING,
            day == AOC_FIRST_DAY,
        );
        solve_day(day).await?
    }
    Ok(())
}
