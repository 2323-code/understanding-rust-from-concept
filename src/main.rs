// fn func_ex_div_some(x: i32, y: i32) ->Option<i32> -> Option<i32> {
//     let ans = if y == 0 { None } else { Some(x / y) };
//     ans}

// fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
//     if y == 0 {
//         Err("div by zero")
//     } else {
//         Ok(x / y)
//     }
// }

// fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
//     if let Some(x) = ans {
//         println!("{}", x)
//     } else {
//         println!("None")
//     }
// }

// fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
//     match ans {
//         Some(x) => println!("{}", x),
//         None => println!("None"),
//     }
// }

// fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
//     match ans {
//         Ok(res) => println!("{}", res),
//         Err(str) => println!("{}", str),
//     }
// }

// fn main() {
//     func_ex_print_some(func_ex_div_some(10, 5));
//     func_ex_print_some(func_ex_div_some(10, 0));
//     func_ex_print_some_match(func_ex_div_some(10, 5));
//     func_ex_print_some_match(func_ex_div_some(10, 0));
//     func_ex_print_result(func_ex_div_result(10, 5));
//     func_ex_print_result(func_ex_div_result(10, 0));
// }

// mod module_hello {
//     pub fn print_hello(name: &str) {
//         println!("Hello, {}!", name);
//     }
// }

// fn main() {
//     module_hello::print_hello("Alice");
// }

// fn main() {
//     let s = "Hello".to_string();
//     let t = s;
//     println!("{}", t);
//     println!("{}", s);
// }

fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg)
}

fn main() {
    let s = "Hello".to_string();
    let s_ref = &s;
    let s_ref2 = &s;
    myprint(s_ref);
    myprint(s_ref);

    myprint(s_ref2);
    myprint(s_ref2);
}
