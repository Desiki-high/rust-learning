fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn largest<T>(list: &[T]) -> T
where
    T: core::cmp::PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn learn_generics() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    learn_struct_generics();
    learn_enum_generics::<(), ()>();
    learn_method_generics();
    learn_const_generics();
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn learn_struct_generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_float = Point { x: 5, y: 4.0 };
}

fn learn_enum_generics<T, E>() -> Result<(), E> {
    Ok(())
}

struct NewPoint<T> {
    x: T,
    y: T,
}

impl<T> NewPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl NewPoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn learn_method_generics() {
    let p = NewPoint { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2); // T:i32 U:f64 V:i32 W:char
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn display_array_ref<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn learn_const_generics() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array_ref(&arr[1..]);

    let arr: [f32; 2] = [1.0, 2.0];
    display_array_ref(&arr);

    let arr: [char; 2] = ['1', '2'];
    display_array_ref(&arr);

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
