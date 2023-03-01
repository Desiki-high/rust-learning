#![allow(dead_code)]
#![allow(unused_variables)]

pub fn learn_immut_ref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");

    let len = calculate_length(&s1); //use the immutable reference

    println!("The length of '{}' is: {}.", s1, len);
}

// in this func we have the right of s without ownership, so we can't modify s
fn calculate_length(s: &String) -> usize {
    s.len()
} // nothing happend

pub fn learn_mut_ref() {
    let mut s1 = String::from("hello");

    change(&mut s1); // we create a mutable reference, a value can only have one mut ref in a time
    println!("The value of s is: {}.", s1);

    let mut s2 = String::from("hello");

    let r1 = &mut s2;
    let r2 = &mut s2; // we can't use r1 after define r2

    //println!("{}", r1);  //error
    println!("{}", r2);

    let mut s3 = String::from("hello");

    let r3 = &s3;
    let r4 = &s3;
    // let r5 = &mut s3; // error, we can't use mut and immut ref at a time and r3 r4 scope will ending when the last use
    println!("{}, {}", r3, r4);

    let r5 = &mut s3; // r3 r4 scope is over
    println!("{}", r5);
}

// in this func we have the right of s without ownership, but we can modify s
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
