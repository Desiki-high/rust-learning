use num::complex::Complex;

pub fn learn_types() {
    learn_int();
    learn_float();
    learn_math();
    learn_bit_operation();
    learn_range();
    learn_complex();
    learn_char();
    learn_bool();
}

fn learn_int() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
}

fn learn_float() {
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);
    // assert!(0.1 + 0.2 == 0.3);    //panic

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);    //panic

    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);    //panic
    if x.is_nan() {
        println!("未定义的数学行为")
    }
    // 取整
    println!("{}", 13.14_f32.round());
}

fn learn_math() {
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

fn learn_bit_operation() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;
    println!("a: {}, b: {}", a, b);
    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {} ", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn learn_range() {
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

fn learn_complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

// char only use ''
fn learn_char() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", g);
    println!("{}", heart_eyed_cat);
    println!(
        "字符'{}'占用了{}字节的内存大小",
        heart_eyed_cat,
        std::mem::size_of_val(&heart_eyed_cat)
    );
    let mut i = 0;
    for _ in '你'..='我' {
        i += 1;
    }
    println!("你我相差: {}", i);
}

fn learn_bool() {
    let _t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }
}
