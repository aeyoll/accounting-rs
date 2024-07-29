use crate::models::Account;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

pub fn handle_person_list(account: Account) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec!["Name", "Income"]);

    for person in &account.persons {
        table.add_row(vec![
            person.name.as_str(),
            person.income.to_string().as_str(),
        ]);
    }

    println!("{table}");
}
