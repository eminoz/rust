fn main() {
    let mut user1 = User {
        name: String::from("emin"),
        suspended: false,
        email: String::from("emin@emin.com"),
    };
    user1.suspended = true;
    user1.email = String::from("emin2@e.com");
    user1.name = String::from("emin2");
}

struct User {
    name: String,
    suspended: bool,
    email: String,
}
