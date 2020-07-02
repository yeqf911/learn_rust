use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::LinkedList;

pub fn run() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("A"), 1);
    scores.insert(String::from("B"), 18);
    scores.insert(String::from("A"), 1);
    println!("{:?}", scores);

    let keys = vec!["one", "two", "three"];
    let values = vec![1, 2, 3];
    let map: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    let btmap: BTreeMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", map);
    println!("{:?}", btmap);

    let mut keys: LinkedList<&str> = LinkedList::new();
    keys.push_back("one");
    keys.push_back("two");
    keys.push_back("three");
    let mut values: LinkedList<i32> = LinkedList::new();
    values.push_back(1);
    values.push_back(2);
    values.push_back(3);
    let btmap: BTreeMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", btmap);

    let mut map = HashMap::new();
    {
        let s1 = String::from("one");
        let s2 = String::from("two");
        map.insert(&s1, 1);
        map.insert(&s2, 2);
        println!("{:?}", map);
        // 出了这个作用域就不能再访问这个map了，因为&s1和&s2指向的内存已经释放了
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let blue = scores.get(&String::from("Blue"));
    println!("{:?}", blue);
    for (index, elm) in scores.iter().enumerate() {
        println!("index = {}, elm = {:?}", index, elm);
    }
    for (k, v) in &scores {
        println!("k = {}, v = {}", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    for (k, v) in scores {
        println!("k = {}, v = {}", k, v);
    }
    // scores就不能再用了，上面已经借走了
    // println!("{:?}", scores);

    //
    let mut scores = HashMap::new();
    scores.insert("one", 1);
    scores.insert("two", 2);
    scores.entry("three").or_insert(3);
    let e = scores.entry("four").or_insert_with(|| -> i32 { 911 });
    println!("{}", e);
    // e作为值得引用，可以修改值
    *e = 2;
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    let e = map.entry("world");
    println!("{:?}", e);
    let key = e.key();
    println!("{:?}", e);
    let vref = e.or_insert(0);
    // println!("{:?}", e);
}
