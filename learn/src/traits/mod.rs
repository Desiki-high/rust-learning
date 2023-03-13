mod tarit_bound;
mod tarit_example;
mod tarit_obj;
mod trait_types;
use trait_types::Summary;

pub fn learn_trait() {
    // 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的
    trait_types::learn_trait_type();
    tarit_bound::learn_tarit_bound();
    tarit_example::learn_tarit_example();
    tarit_obj::learn_tarit_obj();
}

//使用特征作为函数参数 ，接收实现了Summary特征 的 item 参数。
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
