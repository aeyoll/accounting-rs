use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;
// Those models will be serialized to JSON into a file

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub income: f64,
}

impl Person {
    // Load the person's income from the account
    pub fn load_income_from_account(&mut self, account: &Account) {
        self.income = account.persons.iter().find(|p| p.name == self.name).unwrap().income;
    }
}

impl FromStr for Person {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let person: Person = Person {
            name: s.to_string(),
            income: f64::default(),
        };

        Ok(person)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub person: Person,
    pub monthly: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub persons: Vec<Person>,
    pub expenses: Vec<Expense>,
}

pub static ACCOUNT_FILE: &str = "account.json";

impl Account {
    pub fn load_from_file() -> Result<Account, anyhow::Error> {
        let file = std::fs::File::open(ACCOUNT_FILE)
            .unwrap_or_else(|_err| std::fs::File::create(ACCOUNT_FILE).unwrap());

        let account: Account = match serde_json::from_reader(file) {
            Ok(account) => {
                account
            },
            Err(err) => {
                tracing::error!("Error loading account from file {:?}", err);
                Account::default()
            },
        };

        Ok(account)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let mut file = std::fs::File::create(ACCOUNT_FILE)?;
        let account = serde_json::to_string_pretty(self)?;
        file.write_all(account.as_bytes())?;

        Ok(())
    }
}
