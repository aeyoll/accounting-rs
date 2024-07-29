use crate::models::Person;
use crate::validators;
use clap::{Parser, Subcommand};

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
        #[arg(long, value_parser = validators::validate_date_format)]
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
        #[arg(long, value_parser = validators::validate_month_date_format)]
        date: String,
    },
}
