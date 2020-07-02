pub fn run() {
    let data = "initial string";
    let s = data.to_string();
    println!("{}", s);

    let s = "hello world".to_string();
    println!("{}", s);

    let mut s = String::from("hello world");
    s.push('!');
    s.push_str(" Thanks");
    println!("{}", s);

    // + 运算符 fn add(self, s: &str) -> String;
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);

    let s1 = String::from("I ");
    let s2 = String::from("Love ");
    let s3 = String::from("You!");
    // let s = s1 + &s2 + &s3;
    let s = format!("{}{}{}", s1, s2, s3);
    println!("{}", s);
    println!("s1 = {}, s2 = {} s3 = {}", s1, s2, s3);

    let hello = "Здравствуйте";
    // let s = &hello[0..=6]; // 不能设置为奇数末尾，否则无法识别为有效字符，编译错误
    let s = &hello[1..6];
    println!("{}", s);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
