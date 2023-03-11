#![allow(dead_code)]
#![allow(unused_variables)]
mod example;
mod guard;
mod matchs;
mod option;

pub fn learn_pattern() {
    matchs::learn_match();
    option::learn_option();
    example::example();
    guard::learn_guard();
}
