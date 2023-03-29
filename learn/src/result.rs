use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn learn_result() {
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Result<T, E> 做为函数的返回值 可以根据Err<E>处理错误
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    dbg!(read_username_from_file());
    dbg!(read_username_from_file_plus());
    dbg!(read_username_from_file_pro());
}

/// 该函数返回一个 Result<String, io::Error> 类型，当读取用户名成功时，返回 Ok(String)，失败时，返回 Err(io:Error)
fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}

/// 使用？宏取代match
fn read_username_from_file_plus() -> Result<String, io::Error> {
    // 如果结果是 Ok(T)，则把 T 赋值给 f，如果结果是 Err(E)，则返回该错误
    // 只有错误值能直接返回，正确的值不行
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// 使用？实现链式调用
/// File::open 遇到错误就返回，没有错误就将 Ok 中的值取出来用于下一个方法调用
fn read_username_from_file_pro() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
