use crate::models;
use crate::models::Account;

pub fn handle_balance_show(account: Account, date: &String) {
    let filtered_expenses: Vec<&models::Expense> = account
        .expenses
        .iter()
        .filter(|e| e.date.starts_with(date))
        .collect();

    let total_expenses: f64 = filtered_expenses.iter().map(|e| e.amount).sum();
    let total_income: f64 = account.persons.iter().map(|p| p.income).sum();

    let mut paid_amounts: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for expense in filtered_expenses {
        *paid_amounts
            .entry(expense.person.name.clone())
            .or_insert(0.0) += expense.amount;
    }

    println!("Total income: {:.2}€", total_income);
    println!("Total expenses: {:.2}€", total_expenses);

    let mut balances: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for person in &account.persons {
        let fair_share = (person.income / total_income) * total_expenses;
        let paid = paid_amounts.get(&person.name).unwrap_or(&0.0);
        balances.insert(person.name.clone(), fair_share - paid);
    }

    let mut debtors: Vec<(&String, &f64)> = balances
        .iter()
        .filter(|(_, &balance)| balance > 0.0)
        .collect();
    let mut creditors: Vec<(&String, &f64)> = balances
        .iter()
        .filter(|(_, &balance)| balance < 0.0)
        .collect();

    debtors.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    creditors.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    println!("Balance breakdown for {}:", date);
    for (debtor, amount) in debtors {
        for (creditor, credit) in &creditors {
            let payment = amount.min(credit.abs());
            if payment > 0.0 {
                println!("{} owes {} {:.2}€", debtor, creditor, payment);
            }
        }
    }
}
