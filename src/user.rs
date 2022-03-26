struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn try_user() {
    let mut user1 = User {
        email: String::from("someone@email.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@email.com");
    let user2 = build_user(String::from("thenew@email.com"), String::from("the-username"));

    let _user3 = User{
        email: String::from("anew@email.com"),
        ..user2
    };
    //println!("Nope, it's moved {}", user2.username)

    let _origin = Point(0, 0, 0);
    let _black = Color(0, 0, 0);
}