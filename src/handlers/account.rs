use crate::models::Account;

pub fn handle_account_add(mut account: Account, name: &String) -> Result<(), anyhow::Error> {
    tracing::info!("Creating a new account with name: {}", name);
    account.name = name.to_string();

    account.save()
}

pub fn handle_account_list() {
    // TODO: Implement the account list handler
}
