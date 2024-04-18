struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user_1 = user_struct(
        String::from("Vladimir Vlad"),
        String::from("valdthevlad@icloud.com"),
    );
}

fn user_struct(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
    }
}
