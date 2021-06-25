use std::option::Option::Some;
use std::result::Result::Ok;

fn main() {}

#[test]
fn test001() {
    let fc: Option<&str> = None;
    // fc = Some("pink");
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = fc {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(a) = age {
        if a > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("FUCK");
    }
}

#[test]
fn test002() {
    let mut arr = vec![];
    arr.push(1);
    arr.push(2);
    arr.push(3);

    // while let Some(s) = arr.pop(){
    //     println!("{}",s)
    // }

    for (idx, val) in arr.iter().enumerate() {
        println!("index:{}  value:{}", idx, val)
    }
}

#[test]
fn test003() {
    let p = (3, 4);
    print_coordinates(&p)
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x:{},y:{}", x, y)
}

#[test]
fn test004() {
    if let x = 5 {
        println!("None")
    }

    let tmp = 1;
    match tmp {
        1 => println!("one"),
        _ => println!("anything"),
    };
}

#[test]
fn test005() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => {
            println!("== 50 ==")
        }

        Some(y) => {
            println!("Matched, y = {:?}", y)
        }
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

#[test]
fn test006() {
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other"),
    }

    let y = 5;
    match y {
        1..=10 => println!("one to ten"),
        _ => println!("other"),
    }

    let c = 'f';
    match c {
        'a'..='d' => println!("a to d"),
        _ => println!("other"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test007() {
    let tmp = Point { x: 100, y: 200 };

    let Point { x: a, y: b } = tmp;

    println!("a={};b={}", a, b)
}

#[test]
fn test008() {
    let tmp = Point { x: 1000, y: 200 };

    match tmp {
        Point { x: 100, y } => println!("1"),
        Point { x, y: 200 } => println!("2:{}", x),
        Point { x, y } => println!("3"),
    }
}

#[test]
fn test009() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let tmp = Message::Write(String::from("hello world ! "));

    let c: i32;

    match tmp {
        Message::Quit => {
            println!("Quit");
            c = -1;
        }
        Message::Move { x, y: 50 } => c = x,
        Message::Write(s) => {
            println!("S={}", s);
            c = s.len() as i32
        }
        Message::ChangeColor(r, g, b) => {
            println!("r={},g={},b={}", r, g, b);
            c = r + g + b;
        }
        _ => {
            println!("other");
            c = 666;
        }
    }

    println!("c={}", c)
}

#[test]
fn test010() {
    let x: i32;
    x = 10;
    println!("{}", x)
}

#[test]
fn test011() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }

    let msg = Message::ChangeColor(Color::RGB(100, 200, 300));

    match msg {
        Message::ChangeColor(Color::HSV(x, y, z)) => {
            println!("HSV ==> x:{},y:{},z:{}", x, y, z)
        }
        Message::ChangeColor(Color::RGB(x, y, z)) => {
            println!("RGB ==> x:{},y:{},z:{}", x, y, z)
        }
        _ => println!("other"),
    }
}

#[test]
fn test012() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 100, y: 200 });
    println!("feet:{},  inches:{}, x:{}, y:{}", feet, inches, x, y)
}

#[test]
fn test0123() {
    fn foo(_: i32, y: i32) {
        println!("y:{}", y)
    }

    foo(100, 200);

    let mut set_value = Some(5);
    let set_value1 = Some(10);
    match (set_value, set_value1) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => print!("other"),
    }

    let arr = (1, 2, 3, 4, 5);
    match arr {
        (one, two, _, four, _) => {
            println!("one:{},two:{},four:{},", one, two, four)
        }
    }
}

#[test]
fn test014() {
    // let _x = 10;
    // let y = 20;
    // println!("x:{},y:{}",_x,y);
    //
    // let s = Some(String::from("hello"));
    // if let  Some(_s) = s{
    //     println!("found a string");
    // }
    //
    // println!("{:?}",s)
}

#[test]
fn test015() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let tmp = Point {
        x: 100,
        y: 200,
        z: 300,
    };

    match tmp {
        Point { x, .. } => println!("{}", x),
    }

    let tmp1 = (1, 2, 3, 4, 5);
    if let (x, ..) = tmp1 {
        println!("{}", x)
    }

    let tmp2 = (1, 2, 3, 4, 5);
    if let (x, .., y) = tmp2 {
        println!("x:{},y:{}", x, y)
    }

    // let tmp3 = (1, 2, 3, 4, 5);
    // //can only be used once per tuple pattern
    // if let (.., x, ..) = tmp3 {
    //     println!("x:{}", x)
    // }
}

#[test]
fn test016() {
    let tmp = Some(4);

    match tmp {
        Some(x) if x > 5 => {
            println!("x>5;x={}", x)
        }
        Some(x) => println!("x<5;x={}", x),
        None => ()
    }
}


#[test]
fn test017() {
    let x =Some(10);
    let y = 10;

    match x {
        Some(50)=> println!("Get 50"),
        Some(n) if n == y => {
            println!("Matched, n = {}",n)
        },
        _ => println!("other")
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let a = 5;
    let b = true;
    match a {
        4 |5|6 if b =>{
            print!("true")
        }
        _ => println!("false")
    }
}


#[test]
fn test018() {
    enum Message{
        Hello{id:i32}
    }

    let tmp = Message::Hello {id:10};

    match tmp {
        Message::Hello {id:id_variable@ 3..=14} => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello {id:tid} => println!("tid:{}",tid)
    }

}
