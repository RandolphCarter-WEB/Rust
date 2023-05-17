use std::fs::OpenOptions;

/// Option<Type> can use if you don't know the type of the value.
fn plus_one(s: Option<i32>) {
    match s {
        Some(i) => println!("{}", i+1),
        None => println!("None")
    }
}

fn main() {
    {
        let test_inner :_ = "hello! This is Inner";
        println!("[testInner] notice : {}", test_inner);
    }

    ///&str type cannot use additional && &str dont have
    let test_outer :&str = "hello! This is Outer";
    println!("[testOuter] notice : {}", test_outer);
    println!("[testOuter] notice len : {}", test_outer.len());

    /// String type can use additional
    let mut test_text :String = String::from("hello! I'm replace Outer");
    test_text.push_str(" Oh and i's pushed Text");
    println!("[testOuter] notice : {}", test_text);
    println!("[testOuter] notice len : {}", test_text.len()); /// Just string length
    println!("[testOuter] notice capacity : {}", test_text.capacity()); /// mem storage size

    let num :_ = 10;
    let result :_
        = if num == 1 {
            10
        } else if num == 2 {
            20
        } else if num == 3 {
            30
        } else if num == 4 {
            40
        } else if num == 5 {
            50
        } else {
            0
    };

    println!("[testOuter] notice : {}", result);

    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else")
    };

    let test_num : Option<i32> = Some(10);
    plus_one(test_num);


}
