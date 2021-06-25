use std::fmt::Display;

fn main() {
    // {
    //     let x = 5;            // ----------+-- 'b
    //     //           |
    //     let r = &x;           // --+-- 'a  |
    //     //   |       |
    //     println!("r: {}", r); //   |       |
    //     // --+       |
    // }                         // ----------+

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(string1.as_str(), string2);
    // println!("{}",result)

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{}", first_sentence);
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "hello";

    
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let mut v: &str = "";
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

struct ImportantExcerpt<'a> {
    part: &'a str,
}

//添加范型
fn longest1<'a, T, U>(x: &'a str, y: &'a str, ann: T, any: U) -> &'a str
where
    T: Display,
    U: Copy,
{
    let mut v: &str = "";
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
