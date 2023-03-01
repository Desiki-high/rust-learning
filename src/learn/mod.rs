mod borrow;
mod owner;
mod types;

pub fn learn_types() {
    types::learn_int();
    types::learn_float();
    types::learn_math();
    types::learn_bit_operation();
    types::learn_range();
    types::learn_complex();
    types::learn_char();
    types::learn_bool();
}

pub fn learn_owner() {
    owner::learn_copy();
    owner::learn_func_value();
}

pub fn learn_borrow() {
    borrow::learn_immut_ref();
    borrow::learn_mut_ref();
}
