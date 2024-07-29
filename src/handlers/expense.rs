use crate::models::{Account, Expense, Person};

pub fn handle_expense_add(
    mut account: Account,
    description: &String,
    amount: &f64,
    date: &String,
    monthly: &bool,
    person: &Person,
) -> Result<(), anyhow::Error> {
    // Load the person's income from the account
    let mut p = person.clone();
    p.load_income_from_account(&account);

    let expense = Expense {
        description: description.to_string(),
        amount: *amount,
        date: date.to_string(),
        monthly: *monthly,
        person: p,
    };

    account.expenses.push(expense);
    account.save()
}
