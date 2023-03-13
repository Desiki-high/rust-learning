pub fn learn_vector() {
    let v: Vec<i32> = Vec::new();
    // capacity 默认是 0
    let mut v = Vec::new();
    v.push(1);

    // 预先知道要存储的元素个数，避免后续由于push 动态扩容导致的内存分配和拷贝
    let mut _v: Vec<i32> = Vec::with_capacity(10);

    // 读取元素
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    // get 更加安全使用Option 包装
    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // unmut ref
    let first = &v[0];
    // mut ref
    v.push(6);

    // 当数组动态扩容后，之前的引用显然会指向一块无效的内存 操作之前的引用会出错
    // error
    // println!("The first element is: {first}");

    let v = vec![1, 2, 3];
    // 迭代遍历
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }

    // 不同类型构成的vec
    learn_diftype_vec();
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}

trait NewIpAddr {
    fn display(&self);
}

struct V4(String);
impl NewIpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl NewIpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn learn_diftype_vec() {
    // use enum
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip)
    }

    // use dyn trait
    let v: Vec<Box<dyn NewIpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
