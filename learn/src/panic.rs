use backtrace::Backtrace;
use std::net::IpAddr;
use stdext::function_name;

fn panic() {
    let v = vec![1, 2, 3];

    v[99];
}

// 手动打印调用栈
fn dump_stack_test() {
    let bt = Backtrace::new();
    println!("backtrace dump start ===============");
    println!("{:?}", bt);
}

pub fn learn_panic() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    dbg!(home);
    println!("{}", function_name!());
    dump_stack_test();
    panic();
}
