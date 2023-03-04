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
    println!("struct size: {}bytes", std::mem::size_of::<File>());
    println!("i16 size: {}bytes", std::mem::size_of::<i16>());
    println!("String size: {}bytes", std::mem::size_of::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem() {
        println!("Test Start!");
        problem1();
        println!("Test End!");
    }
}
