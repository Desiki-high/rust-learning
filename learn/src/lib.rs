mod borrow;
mod complex;
mod control;
mod method;
mod owner;
mod pattern;
mod types;

pub fn learn_types() {
    types::learn_types();
}

pub fn learn_owner() {
    owner::learn_copy();
    owner::learn_func_value();
}

pub fn learn_borrow() {
    borrow::learn_immut_ref();
    borrow::learn_mut_ref();
}

pub fn learn_complex() {
    complex::learn_complex();
}

pub fn learn_control() {
    control::learn_control();
}

pub fn learn_pattern() {
    pattern::learn_pattern();
}

pub fn learn_method() {
    method::learn_method();
}
