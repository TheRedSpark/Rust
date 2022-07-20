struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub ( crate ) fn chapter_5(){
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
let email = user1.email;
    println!("{email}")
}
