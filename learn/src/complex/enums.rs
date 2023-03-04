enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
pub fn learn_enum() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);
    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    learn_option();
}

fn learn_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or_default();
    dbg!(sum);

    let five = Some(5);
    dbg!(plus_one(five));
    dbg!(plus_one(None));
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
