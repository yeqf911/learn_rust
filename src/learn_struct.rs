pub fn run() {
    println!("hello");
    let user = User::new(String::from("yeqianfeng"), 18);
    println!("{:?}", user);
    let u2 = User {
        name: String::from("xiaoming"),
        age: 20,
        ..user
    };

    println!("{:#?}", u2);

    let u3 = User { ..u2 };

    println!("{:#?}", u3);

    let c = Color(1, 2, 3);
    let p = Point(3, 4, 5);

    println!("{:#?}", c);
    println!("{:#?}", p);
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
    name: String,
    age: usize,
    email: String,
    sign_in_count: usize,
}

// struct Acount {
//     username: &str,
//     email: &str,
//     active: bool,
// }

impl User {
    fn new(name: String, age: usize) -> User {
        User {
            name,
            age,
            email: String::from("email"),
            sign_in_count: 1,
        }
    }
}
