pub fn learn_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // error
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    learn_struct();
    learn_static();
}

/// 生命周期标注语法
/// &i32  一个引用
/// &'a i32     具有显式生命周期的引用
/// &'a mut i32 具有显式生命周期的可变引用
/// 当把具体的引用传给 longest 时，那生命周期 'a 的大小就是 x 和 y 的作用域的重合部分，换句话说，'a 的大小将等于 x 和 y 中较小的那个。
/// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
/// 1. 函数参数的生命周期
/// 2. 函数体中某个新建引用的生命周期(悬垂引用) 改为返回所有权
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/// ImportantExcerpt 结构体中有一个引用类型的字段 part，因此需要为它标注上生命周期。结构体的生命周期标注语法跟泛型参数语法很像，需要对生命周期参数进行声明 <'a>。该生命周期标注说明，结构体 ImportantExcerpt 所引用的字符串 str 必须比该结构体活得更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法中的生命周期
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn learn_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    dbg!(i);
    // error: when use i , the first_sentence is drop the ref is unsafe
    // let i;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("{:?}", i);
}

fn learn_static() {
    // 'static，拥有该生命周期的引用可以和整个程序活得一样久。
    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
