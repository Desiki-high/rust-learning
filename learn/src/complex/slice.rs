pub fn learn_slice() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[4..len];
    let slice = &s[4..];
    let slice = &s[0..len];
    let slice = &s[..];

    let s = String::from("hello world");
    let word = first_word(&s);
    //s.clear(); // error!
    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    //字符串常量就是字符串切片
    let s: &str = "Hello, world!";
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
