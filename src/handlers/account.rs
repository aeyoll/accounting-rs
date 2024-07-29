use crate::models::Account;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

pub fn handle_account_add(mut account: Account, name: &String) -> Result<(), anyhow::Error> {
    tracing::info!("Creating a new account with name: {}", name);
    account.name = name.to_string();

    account.save()
}

pub fn handle_account_show(account: Account) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec!["Name"])
        .add_row(vec![account.name.as_str()]);

    println!("{table}");
}
