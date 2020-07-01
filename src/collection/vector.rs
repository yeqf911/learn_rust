pub fn run() {
    let mut v: Vec<i32> = Vec::new();
    v.insert(0, 8);
    v.insert(1, 8);
    v.insert(2, 8);
    v.insert(1, 18);
    println!("{:?}", v);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    let s1 = String::from("I");
    let s2 = String::from("Love ");
    let s3 = String::from("You");

    {
        let vs = vec![s1, s2, s3];
        // 结束就释放了，包括数组中的元素
        // s1,s2,s3都不能再使用了
    }
    // println!("{:?}", s1);

    let vi = vec![2, 2, 3, 4, 5];
    let i2 = vi[2];
    println!("{}", i2);
    match vi.get(9999) {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    let ss1 = String::from("hello");
    let ss2 = String::from("world");
    let vss: Vec<String> = vec![ss1, ss2];
    let sss1: &String = &vss[0];
    println!("{}", sss1);

    match vss.get(0) {
        Some(s) => println!("{}", s),
        // Some(&s) => println!("{}", s), // 此处如果是 &s，则 s 会被当成String类型，编译出错，因为无法move
        None => println!("None string"),
    }
    println!("{:?}", vss);

    // 遍历数组
    let v = vec![1, 2, 3, 45];
    for i in &v {
        println!("{}", i);
    }

    // 遍历并修改数据元素
    let mut v = vec![1, 100, 199];
    for i in &mut v {
        *i += 1;
    }
    println!("{:?}", v);

    // 使用枚举类型让vector存储多种类型
    let v: Vec<DifferentType<String>> = vec![
        DifferentType::Int(1),
        DifferentType::Float(0.24),
        DifferentType::Text(String::from("world")),
        DifferentType::Other(String::from("Fuck")),
    ];
    println!("{:?}", v);

    // 数组存在元素的引用的时候，不可以push, push会增加长度，扩展空间，移动内存，之前的元素就会指向释放的内存
    let mut v = vec![1, 2, 3, 4];
    let first = &v[0];
    v.push(5);
    // 如果此处不适用first，是可以编译通过的。
    // println!("{}", first);
}

#[derive(Debug)]
enum DifferentType<T> {
    Int(i32),
    Float(f64),
    Text(String),
    Other(T),
}
