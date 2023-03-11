#![allow(dead_code)]
#![allow(unused_variables)]

pub fn learn_control() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    let mut collection: [String; 2] = std::array::from_fn(|_| String::from("rust is good!"));
    // 如果不使用引用的话，所有权会被转移（move）到 for 语句块中
    for item in &mut collection {
        item.push_str("TEST");
    }
    dbg!(collection);

    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    let mut n = 0;
    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }
    // loop {
    //     if n > 5 {
    //         break;
    //     }
    //     println!("{}", n);
    //     n += 1;
    // }
    println!("我出来了！");

    let mut counter = 0;
    // break 可以单独使用，也可以带一个返回值，有些类似 return
    // loop 是一个表达式，因此可以返回一个值
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
