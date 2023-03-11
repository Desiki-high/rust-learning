pub fn learn_guard() {
    // if x < 5 is a guard
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let num = Some(10);
    match num {
        // 如果x是大于等于5的 则第一个匹配守卫为假 则会继续匹配
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    fix_overwrite();
    learn_at();
}

fn fix_overwrite() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 我们可以在内部使用外部变量y 而不是被覆盖的y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Hello { id: i32 },
}
// 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
fn learn_at() {
    let msg = Message::Hello { id: 5 };
    match msg {
        // 通过在 3..=7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时将该值绑定到变量 id_variable 上。
        // can use id value
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        // can't use id value
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    // since Rust 1.53
    match 2 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
