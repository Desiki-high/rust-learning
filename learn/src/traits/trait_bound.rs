// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} 无法规定item1 和 item2 是同一类型

// 单重约束
// pub fn notify<T: Summary>(item1: &T, item2: &T) {} 这里item1 和 item2 是同一类型 都是T 即特征约束

// 多重约束
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}  过于复杂
// 使用 where 约束
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn learn_trait_bound() {
    let pair = Pair { x: 12, y: 32 };
    pair.cmp_display();
}

// 标准库为任何实现了 Display 特征的类型实现了 ToString 特征：
// impl<T: Display> ToString for T {
//     // --snip--
// }
// 我们可以对任何实现了 Display 特征的类型调用由 ToString 定义的 to_string 方法
