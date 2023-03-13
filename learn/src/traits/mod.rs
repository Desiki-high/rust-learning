mod trait_adv;
mod trait_bound;
mod trait_example;
mod trait_obj;
mod trait_types;
use trait_types::Summary;

pub fn learn_trait() {
    // 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的
    trait_types::learn_trait_type();
    trait_bound::learn_trait_bound();
    trait_example::learn_trait_example();
    trait_obj::learn_trait_obj();
    trait_adv::learn_trait_adv();
}

//使用特征作为函数参数 ，接收实现了Summary特征 的 item 参数。
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
