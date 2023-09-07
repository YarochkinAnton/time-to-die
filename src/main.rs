use std::process::ExitCode;

use chrono::NaiveDate;

/// And that's optimistic)))
const RUSSIAN_LIFE_EXPECTANCY_YEARS: i64 = 74;
const WEEK_IN_A_YEAR: f32 = 52.14;
const DAYS_IN_A_WEEK: f32 = 7.0;
const HORIZONTAL_BORDER: char = '━';
const VERTICAL_BORDER: char = '┃';
const TOP_LEFT_CORNER: char = '┏';
const TOP_RIGHT_CORNER: char = '┓';
const BOTTOM_LEFT_CORNER: char = '┗';
const BOTTOM_RIGHT_CORNER: char = '┛';
const PROGRESS_SYMBOL: char = '█';
const EMPTY_SYMBOL: char = ' ';

lazy_static::lazy_static! {
    static ref COLUMNS: usize = terminal_size::terminal_size().unwrap().0.0 as usize - 2;
}

fn main() -> ExitCode {
    let mut args = std::env::args();

    let Some(date_of_birth_string) = args.nth(1) else {
        println!("provide date of birth as the first argument in following format \"yyyy-mm-dd\"");
        return ExitCode::FAILURE;
    };

    let date_of_birth: NaiveDate = date_of_birth_string.parse().unwrap();

    let now = chrono::Local::now().date_naive();

    let lived = now.signed_duration_since(date_of_birth);

    let lived_weeks = lived.num_days() as f32 / DAYS_IN_A_WEEK;
    let expected_weeks = RUSSIAN_LIFE_EXPECTANCY_YEARS as f32 * WEEK_IN_A_YEAR;
    let left_weeks = expected_weeks - lived_weeks;
    let death_percentage = lived_weeks / expected_weeks * 100.0;
    let left_weeks: usize = left_weeks.trunc() as usize;
    let total_weeks = lived_weeks.trunc() as usize + left_weeks;
    let total_weeks_rem = total_weeks % *COLUMNS;
    let total_weeks_pad = *COLUMNS - total_weeks_rem;

    println!("I lived:");
    println!("\t{} days", lived.num_days());
    println!("\t{} weeks", lived_weeks);
    println!("\t{}% of Russian life expectancy", death_percentage);

    let progress: Vec<char> = std::iter::repeat_with(|| PROGRESS_SYMBOL)
        .take(lived_weeks.trunc() as usize)
        .chain(std::iter::repeat_with(|| EMPTY_SYMBOL).take(left_weeks + total_weeks_pad))
        .collect();

    print_horizontal_border(Position::Top);
    progress
        .chunks(*COLUMNS)
        .map(|chunk| chunk.iter().collect::<String>())
        .for_each(|line| print_horizontal(&line));
    print_horizontal_border(Position::Bottom);

    ExitCode::SUCCESS
}

enum Position {
    Top,
    Bottom,
}

fn print_horizontal_border(position: Position) {
    let horizontal_line: String = std::iter::repeat_with(|| HORIZONTAL_BORDER)
        .take(*COLUMNS)
        .collect();
    match position {
        Position::Top => {
            print!("{}", TOP_LEFT_CORNER);
            print!("{}", horizontal_line);
            println!("{}", TOP_RIGHT_CORNER);
        }
        Position::Bottom => {
            print!("{}", BOTTOM_LEFT_CORNER);
            print!("{}", horizontal_line);
            println!("{}", BOTTOM_RIGHT_CORNER);
        }
    }
}

fn print_horizontal(s: &str) {
    println!("{}{}{}", VERTICAL_BORDER, s, VERTICAL_BORDER);
}
