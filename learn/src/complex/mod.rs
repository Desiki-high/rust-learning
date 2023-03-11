#![allow(dead_code)]
#![allow(unused_variables)]
mod array;
mod enums;
mod slice;
mod string;
mod structs;
mod tuple;

pub fn learn_complex() {
    slice::learn_slice();
    string::learn_string();
    tuple::learn_tuple();
    structs::learn_struct();
    enums::learn_enum();
    array::learn_array();
}
