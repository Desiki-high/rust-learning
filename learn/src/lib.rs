#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
mod borrow;
mod collections;
mod complex;
mod control;
mod generics;
mod lifetime;
mod method;
mod owner;
mod panic;
mod pattern;
mod result;
mod traits;
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

pub fn learn_generics() {
    generics::learn_generics();
}

pub fn learn_trait() {
    traits::learn_trait();
}

pub fn learn_collections() {
    collections::learn_collections();
}

pub fn learn_lifetime() {
    lifetime::learn_lifetime();
}

pub fn learn_panic() {
    panic::learn_panic();
}

pub fn learn_result() {
    result::learn_result();
}
