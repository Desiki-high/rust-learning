pub fn learn_copy() {
    let x = 5;
    let _y = x; // stack auto copy

    //stack auto copy is sure deep copy, actually there is no shallow copy or deep copy and it is just a sample copy
    println!("{}", x);

    let s1 = String::from("hello");
    let _s2 = s1; //move copy, s1 will be can't use

    //  println!("{}", s1); //compile error

    let x: &str = "hello, world"; // x is an immutable reference of str(restore in stack)
    let y = x; // y is the copy of x (deep copy in stack)
    println!("{};{}", x, y);
}

pub fn learn_func_value() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里

    // 所以到这里s不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里,但 i32 是 Copy 的，所以在后面可继续使用 x

    println!("x:{}", x);
}
// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
}
// 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
}
// 这里，some_integer 移出作用域。不会有特殊操作
