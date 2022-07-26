fn main() {
    // let optional: Option<std::option::Option<None> = None;

    // need to specify time if not using like so b/c complier cannot infer it
    let optional: Option<()> = None;

    let optional_string: Option<String> = None;

    // compile errors
    let optional_error = None;

    // check_optional(optional);

    // let optional = Some(Box::new(9000));
    // check_optional(optional);

    // fn check_optional(optional: Option<Box<i32>>) {
    //     match optional {
    //         Some(p) => println!("has value {p}"),
    //         None => println!("has no value"),
    //     }
    // }
}
