mod enums_with_assosiated_values;

enum UserRole {
    Admin, // 0
    User,  // 1
    Owner, // 2
}

fn enum_with_match(value: UserRole) -> &'static str {
    match value {
        UserRole::Admin => {
            return "adm";
        }
        UserRole::User => {
            return "user";
        }
        UserRole::Owner => {
            return "owner";
        }
    }
}

fn main() {
    let result_adm = enum_with_match(UserRole::Admin);
    let result_user = enum_with_match(UserRole::User);
    let result_owner = enum_with_match(UserRole::Owner);

    println!(
        "result_adm: {}, result_user: {}, result_owner: {}",
        result_adm, result_user, result_owner
    );
    enums_with_assosiated_values::enum_with_values();
}
