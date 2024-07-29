use crate::models::Person;
use chrono::{Datelike, NaiveDate};
use clap::{Parser, Subcommand};

// Validate the YYYY-MM date format using chrono
fn validate_month_date_format(s: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
        Ok(date) if date.day() == 1 => Ok(s.to_string()),
        _ => Err(String::from("Date must be in the format YYYY-MM")),
    }
}

// Validate the YYYY-MM-DD date format using chrono
fn validate_date_format(s: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(_) => Ok(s.to_string()),
        _ => Err(String::from("Date must be in the format YYYY-MM-DD")),
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Account {
        #[command(subcommand)]
        subcommand: AccountCommands,
    },
    Person {
        #[command(subcommand)]
        subcommand: PersonCommands,
    },
    Expense {
        #[command(subcommand)]
        subcommand: ExpenseCommands,
    },
    Balance {
        #[command(subcommand)]
        subcommand: BalanceCommands,
    },
}

#[derive(Subcommand)]
pub enum AccountCommands {
    #[command()]
    Add {
        #[arg(short, long)]
        name: String,
    },
    #[command()]
    Show,
}

#[derive(Subcommand)]
pub enum PersonCommands {
    #[command()]
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        income: f64,
    },
    #[command()]
    List,
}

#[derive(Subcommand)]
pub enum ExpenseCommands {
    #[command()]
    Add {
        #[arg(long)]
        description: String,
        #[arg(long)]
        amount: f64,
        #[arg(long, value_parser = validate_date_format)]
        date: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        monthly: bool,
        #[arg(long)]
        person: Person,
    },
    #[command()]
    List,
}

#[derive(Subcommand)]
pub enum BalanceCommands {
    #[command()]
    Show {
        #[arg(long, value_parser = validate_month_date_format)]
        date: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_month_date_format() {
        assert_eq!(
            validate_month_date_format("2023-01"),
            Ok("2023-01".to_string())
        );
        assert_eq!(
            validate_month_date_format("2023"),
            Err("Date must be in the format YYYY-MM".to_string())
        );
    }

    #[test]
    fn test_validate_date_format() {
        assert_eq!(
            validate_date_format("2023-01-05"),
            Ok("2023-01-05".to_string())
        );
        assert_eq!(
            validate_date_format("2023-01"),
            Err("Date must be in the format YYYY-MM-DD".to_string())
        );
    }
}
