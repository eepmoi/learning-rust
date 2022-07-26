use std::collections::HashMap;

#[derive(Debug)]
struct Test {
    key: String,
    value: String,
}

fn main() {
    create_test_from_hashmap()
}

fn create_test_from_hashmap() {
    let test = Test {
        key: "key".to_string(),
        value: "value".to_string(),
    };

    let h = HashMap::from([
        ("123".to_string(), "abc".to_string()),
        ("456".to_string(), "def".to_string()),
    ]);

    let mut vec = Vec::<Test>::new();
    for (k, v) in h {
        vec.push(Test {
            key: "key".to_string(),
            value: "value".to_string(),
        });
    }

    println!("{:?}", vec);

    // v.push(test)

    // create vector of hashmaps with k/v pairs
    // let h2 = HashMap::new();
    // let h2 = HashMap::<String, String>::new();
    // let mut v = Vec::<HashMap<String, String>>::new();
    // let h3 = HashMap::from([("abc".to_string(), "def".to_string())]);
    // let mut v = Vec::new();
    // v.push(h3);

    // for (k, v) in h {
    //     let h4 = HashMap::from([(k, v)]);
    //     // let h3 = HashMap::from([("abc".to_string(), "def".to_string())]);
    //     // let h3 = HashMap::from((k.as_str(), v.as_str()));
    //     v.push(1)
    //     // h2.insert("abc", "123");
    // }
}

fn example_from_internet() {
    let mut foo = HashMap::new();
    foo.insert("".to_string(), "".to_string());
    let mut f = Vec::new();
    f.push(foo);
}

fn print_hashmap() {
    let h = HashMap::from([
        ("Identity".to_string(), "JoeBloggs".to_string()),
        ("Commit".to_string(), "123456".to_string()),
    ]);

    for (k, v) in h {
        println!("key: {} | value: {}", k, v)
    }
}
