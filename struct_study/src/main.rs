// this is like a class um poo
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//function that expect a result as User type
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("mateus_mota123"),
        email: String::from("mateus@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");

    println!("{:?}", user1);

    let building_user = build_user(
        String::from("salmple@idont.com"),
        String::from("Mateus Mota Nobrega")
    );

    println!(" Building User: {:?} ", building_user);
}
