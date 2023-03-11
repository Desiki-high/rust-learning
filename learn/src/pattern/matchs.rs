enum Direction {
    East,
    West,
    North,
    South,
}
#[derive(Debug)]
enum IpAddr {
    Ipv4,
    Ipv6,
}
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

pub fn learn_match() {
    // 作分支选择处理不同代码
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    // 赋值，match本身也是一个表达式
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    // 通过绑定取值
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            // s x y r g 为取出的值
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    learn_if_let();
    learn_matches();

    let age = Some(30);
    println!("在匹配前,age是{:?}", age);
    if let Some(age) = age {
        // 内部覆盖了age
        println!("匹配出来的age是{}", age);
    }
    println!("在匹配后,age是{:?}", age);
}

// 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
fn learn_if_let() {
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = v {
        println!("three");
    }
}

// Rust 标准库宏：matches!，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。
fn learn_matches() {
    let v = vec![IpAddr::Ipv4, IpAddr::Ipv4, IpAddr::Ipv6];
    // use matches!
    v.iter()
        .filter(|x| matches!(x, IpAddr::Ipv4))
        .for_each(|x| println!("{:?}", x));
    // use if let
    v.iter()
        .filter(|x| if let IpAddr::Ipv6 = x { false } else { true })
        .for_each(|x| println!("{:?}", x));

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}
