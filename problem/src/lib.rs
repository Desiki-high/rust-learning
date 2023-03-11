use std::mem::size_of;

#[derive(Debug)]
struct File {
    _ino: i16,
    _data: String,
}
/// problem1: Rust Struct 中的基本类型成员 如 isize 储存在堆山还是栈上
/// ```
pub fn problem1() {
    let f1 = File {
        _ino: 1,
        _data: String::from("123"),
    };
    println!("{:p}", &f1);
    println!("{:p}", &f1._ino);
    println!("{:p}", &f1._data);
    println!("struct size: {}bytes", size_of::<File>());
    println!("i16 size: {}bytes", size_of::<i16>());
    println!("String size: {}bytes", size_of::<String>());
}

/// problem2: Rust Struct 数组切片和数组引用的区别
/// ```
pub fn problem2() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3]; //这是一个切片，包含指向切片的首位元素的ptr以及切片长度isize
    let r: &[i32; 5] = &a; //这是一个数组的引用 包含一个ptr
    println!("{:p}", &slice);
    println!("{:p}", &r);
    dbg!(size_of::<i64>());
    dbg!(size_of::<&[i32]>());
    dbg!(size_of::<&[i32; 5]>());
    dbg!(size_of::<&String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1() {
        problem1();
    }

    #[test]
    fn problem_2() {
        problem2();
    }
}
