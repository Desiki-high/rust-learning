struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn learn_struct() {
    build_struct();
    mem_struct();
    other_struct();
    display_struct();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        active: user1.active,     //基本类型实现了Copy Trait 发生复制不发生所有权转义
        username: user1.username, //user1.username 发生所有权转移
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.sign_in_count); //可以使用

    // let user4 = User {
    //     email: String::from("another@example.com"),
    //     ..user1  //user1.username 不可用
    // };
}

#[derive(Debug)] //使用derive 派生实现trait Debug方便打印 println!("{:?}", s); 如果使用println!("{}", s); 需要实现trait std::fmt::Display
struct File {
    name: String,
    data: Vec<u8>,
}

fn mem_struct() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::from([1, 2, 3]),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{:#?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

struct AlwaysEqual;
fn other_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

struct Person {
    name: String,
    age: u8,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
fn display_struct() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}
