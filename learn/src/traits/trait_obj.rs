// #[derive(Debug)]
// enum UiObject {
//     Button,
//     SelectBox,
// }

// fn draw(o: UiObject) {
//     println!("{:?}", o);
// }

// pub fn learn_trait_obj() {
//     let objects = [UiObject::Button, UiObject::SelectBox];

//     for o in objects {
//         draw(o)
//     }
// }

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    fn test(&self) {
        println!("test");
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

// 特征对象：dyn trait  注意 dyn不能单独作为特征对象的定义，因为编译器在编译期不知道该类型的大小，可以是各种实现了该trait的类型
// 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。
// 特征对象指向实现了 Draw 特征的类型的实例，也就是指向了 Button 或者 SelectBox 的实例，
// 这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
            component.draw();
        }
    }
}

// 不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
// 方法的返回类型不能是 Self
// 方法没有任何泛型参数

pub fn learn_trait_obj() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    }
    .test();
    // screen.components[0].test();  wrong: 特征对象中只包含了该特征的方法
    screen.run();
}

// 我们也可以使用 泛型 + 特征约束实现 但是这种实现要求 components 中只能包含同种类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// pub fn learn_trait_obj() {
//     let screen = Screen {
//         components: vec![
//             SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No"),
//                 ],
//             },
//             SelectBox {
//                 width: 70,
//                 height: 5,
//                 options: vec![
//                     String::from("!Yes"),
//                     String::from("!Maybe"),
//                     String::from("!No"),
//                 ],
//             },
//         ],
//     };

//     screen.run();
// }
