#[derive(Debug)]
pub enum MyType {
    Abc,
    Def,
    None,
}

#[derive(Debug)]
pub struct MyTest {
    pub my_type: Option<MyType>,
    pub my_number: Option<u32>,
    pub my_string: Option<String>,
    pub my_bool: Option<bool>,
}

fn main() {
    let x = MyTest {
        my_type: Option::None,
        my_number: Option::None,
        my_string: Option::None,
        my_bool: Option::None,
    };

    println!("{:?}", x);

    let y = MyTest {
        my_type: Option::Some(MyType::Abc),
        my_number: Option::None,
        my_string: Option::Some("MyString".to_string()),
        my_bool: Option::None,
    };

    println!("{:?}", y);
}
