use crate::cli::{
    AccountCommands, BalanceCommands, Cli, Commands, ExpenseCommands, PersonCommands,
};
use crate::models::Account;
use clap::Parser;
use std::process;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod cli;
mod handlers;
mod models;
mod validators;

fn app() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "accounting_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load the account model from an account.json file
    let account = Account::load_from_file()?;

    match &cli.command {
        Commands::Account { subcommand } => match subcommand {
            AccountCommands::Add { name } => {
                tracing::info!("Creating a new account with name: {}", name);

                match handlers::account::handle_account_add(account, name) {
                    Ok(_) => {
                        tracing::info!("Account created successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error saving account: {}", err);
                        return Err(err);
                    }
                }
            }
            AccountCommands::Show => handlers::account::handle_account_show(account),
        },

        Commands::Person { subcommand } => match subcommand {
            PersonCommands::Add { name, income } => {
                tracing::info!("Adding a new person");

                match handlers::person::handle_person_add(account, name, income) {
                    Ok(_) => {
                        tracing::info!("Person added successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error adding person: {}", err);
                        return Err(err);
                    }
                }
            }
            PersonCommands::List => handlers::person::handle_person_list(account),
        },

        Commands::Expense { subcommand } => match subcommand {
            ExpenseCommands::Add {
                description,
                amount,
                date,
                monthly,
                person,
            } => {
                tracing::info!("Adding a new expense");

                match handlers::expense::handle_expense_add(
                    account,
                    description,
                    amount,
                    date,
                    monthly,
                    person,
                ) {
                    Ok(_) => {
                        tracing::info!("Expense added successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error adding expense: {}", err);
                        return Err(err);
                    }
                }
            }
            ExpenseCommands::List => handlers::expense::handle_expense_list(account),
        },

        Commands::Balance { subcommand } => match subcommand {
            BalanceCommands::Show { date } => handlers::balance::handle_balance_show(account, date),
        },
    }

    Ok(())
}

fn main() {
    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            tracing::error!("{}", err.to_string());
            1
        }
    });
}
