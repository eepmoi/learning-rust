// #![feature(type_name_of_val)]

struct Rectangle<T> {
    width: T,
    height: T,
}

#[derive(Debug)]
enum Andy<T> {
    A(T),
    B(T),
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// fn print_type_of_2<T: std::fmt::Debug>(x: &T) {
//     println!("value {:?}", x);
//     println!("{}", std::any::type_name::<x>())
// }

fn generic_type<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"

    let abc = "abc";
    print_type_of(&abc);

    // std::any::type_name::<&abc>;

    // needs nightly build
    // use std::any::type_name_of_val;
    // let x = 123;
    // println!("{}", std::any::type_name_of_val(&x));

    let andy = Andy::A("abc");
    println!("{:?}", Andy::A(123));
    println!("{:?}", andy);

    // can't seem to call this directly without wrapping in a function
    // https://doc.rust-lang.org/std/any/fn.type_name.html
    // pub fn type_name<T>() -> &'static str
    // where
    //  T: ?Sized,
    // pub const fn type_name<T: ?Sized>() -> &'static str {
    //     intrinsics::type_name::<T>()
    // }
    // println!("{}", std::any::type_name::<&andy>());

    generic_type("abc");
}
