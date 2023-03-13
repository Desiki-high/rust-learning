pub fn learn_trait_adv() {
    learn_trait_defalut_parameter();
    learn_diftarit_samefn();
    learn_define_tarit_withbound();
    learn_newtype();
}

use std::fmt::{self, Display};
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
// 当使用泛型类型参数时，可以为其指定一个默认的具体类型
// 当用户不指定 RHS 时，默认使用两个同样类型的值进行相加，然后返回一个关联类型 Output。
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

// 并没有为 Point 实现 Add<RHS> 特征，而是实现了 Add 特征（没有默认泛型类型参数），
// 这意味着我们使用了 RHS 的默认类型，也就是 Self: Point
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 实现了Add<Meters> 特征 ，所以是 Millimeters + Meters 返回Output: Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn learn_trait_defalut_parameter() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// 两个特征具有相同的方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// 结构体本身也有一个fly方法
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// trait和struct中的同名关联函数
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

// 如何去调用这个方法 ？？ -> 完全限定语法
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn learn_diftarit_samefn() {
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法

    // 完全限定语法
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // receiver_if_method : self、&self 和 &mut self 只有方法才拥有
    // 完全限定语法可以用于任何函数或方法调用
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// supertrait
// 让某个特征 A 能使用另一个特征 B 的功能(另一种形式的特征约束)，这种情况下，不仅仅要为类型实现特征 A，还要为类型实现特征 B

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// 如果你想要实现 OutlinePrint 特征，首先你需要实现 Display 特征
impl OutlinePrint for Point {}

fn learn_define_tarit_withbound() {
    Point { x: 10, y: 20 }.outline_print();
}

// impl<T> std::fmt::Display for Vec<T> {}
// 绕过孤儿规则 为Vec<T> 实现 Display 二者都定义在标准库中

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn learn_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
