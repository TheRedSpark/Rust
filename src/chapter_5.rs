struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub(crate) fn chapter_5() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@passt.com"),
        ..user1
    };
    user1.email = String::from("anotheremail@example.com");
    let email2 = user2.email;
    let email = user1.email;
    println!("{email}");
    println!("{email2}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub(crate) fn area() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,

    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1));



    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}


