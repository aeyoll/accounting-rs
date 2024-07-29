use crate::models::{Account, Expense, Person};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

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

pub fn handle_expense_list(account: Account) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Description", "Amount", "Date", "Monthly", "Person"]);

    for expense in &account.expenses {
        table.add_row(vec![
            expense.description.as_str(),
            expense.amount.to_string().as_str(),
            expense.date.as_str(),
            expense.monthly.to_string().as_str(),
            expense.person.name.as_str(),
        ]);
    }

    println!("{table}");
}
