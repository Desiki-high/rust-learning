pub fn learn_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five);
    dbg!(six);
    dbg!(none);
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
