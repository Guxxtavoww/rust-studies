use chrono::prelude::*;

pub fn get_current_date() -> String {
    let local: DateTime<Local> = Local::now();

    let current_date = local.date();

    return current_date.to_string();
}
