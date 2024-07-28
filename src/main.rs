use crate::models::{Account, Person};
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use std::process;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod models;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
enum AccountCommands {
    #[command()]
    Add {
        #[arg(short, long)]
        name: String,
    },
    #[command()]
    List,
}

#[derive(Subcommand)]
enum PersonCommands {
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
enum ExpenseCommands {
    #[command()]
    Add {
        #[arg(long)]
        description: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
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
enum BalanceCommands {
    #[command()]
    Show,
}

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
    let mut account = Account::load_from_file()?;

    match &cli.command {
        Commands::Account { subcommand } => match subcommand {
            AccountCommands::Add { name } => {
                tracing::info!("Creating a new account with name: {}", name);
                account.name = name.to_string();

                match account.save() {
                    Ok(_) => {
                        tracing::info!("Account created successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error saving account: {}", err);
                        return Err(err);
                    }
                }
            }
            AccountCommands::List => {
                todo!("Implement the list command");
            }
        },

        Commands::Person { subcommand } => match subcommand {
            PersonCommands::Add { name, income } => {
                tracing::info!("Adding a new person");
                let person = Person {
                    name: name.to_string(),
                    income: *income,
                };

                account.persons.push(person);
                match account.save() {
                    Ok(_) => {
                        tracing::info!("Person added successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error adding person: {}", err);
                        return Err(err);
                    }
                }
            }
            PersonCommands::List => {
                todo!("Implement the list command");
            }
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
                let expense = models::Expense {
                    description: description.to_string(),
                    amount: *amount,
                    date: date.to_string(),
                    monthly: *monthly,
                    person: person.clone(),
                };

                account.expenses.push(expense);
                match account.save() {
                    Ok(_) => {
                        tracing::info!("Expense added successfully");
                    }
                    Err(err) => {
                        tracing::error!("Error adding expense: {}", err);
                        return Err(err);
                    }
                }
            }
            ExpenseCommands::List => {
                todo!("Implement the list command");
            }
        },

        Commands::Balance { subcommand } => match subcommand {
            BalanceCommands::Show => {
                // Compute who owes money to whom based on income-weighted fair share
                let total_expenses: f64 = account.expenses.iter().map(|e| e.amount).sum();
                let total_income: f64 = account.persons.iter().map(|p| p.income).sum();

                // Calculate how much each person has paid
                let mut paid_amounts: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
                for expense in &account.expenses {
                    *paid_amounts.entry(expense.person.name.clone()).or_insert(0.0) += expense.amount;
                }

                println!("Total income: {:.2}€", total_income);
                println!("Total expenses: {:.2}€", total_expenses);

                // Calculate the difference between income-weighted fair share and paid amount
                let mut balances: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
                for person in &account.persons {
                    let fair_share = (person.income / total_income) * total_expenses;
                    let paid = paid_amounts.get(&person.name).unwrap_or(&0.0);
                    balances.insert(person.name.clone(), fair_share - paid);
                }

                // Determine who owes money to whom
                let mut debtors: Vec<(&String, &f64)> = balances.iter().filter(|(_, &balance)| balance > 0.0).collect();
                let mut creditors: Vec<(&String, &f64)> = balances.iter().filter(|(_, &balance)| balance < 0.0).collect();

                debtors.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
                creditors.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

                // Print out who owes money to whom
                println!("Balance breakdown:");
                for (debtor, amount) in debtors {
                    for (creditor, credit) in &creditors {
                        let payment = amount.min(credit.abs());
                        if payment > 0.0 {
                            println!("{} owes {} {:.2}€", debtor, creditor, payment);
                        }
                    }
                }
            }
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
