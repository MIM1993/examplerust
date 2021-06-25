
enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind1{
    V4(String),
    V6(String),
}

enum IpAddrKind2{
    V4(i8,i8,i8,i8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    QUit(),
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn printMessage(&self){
        println!("{:#?}",self);
    }
}




fn main() {
    /*
    let m = Message::Move {x:2,y:3};
    m.printMessage();
    let m1 = Message::Write(String::from("hello world"));
    m1.printMessage();

    let a = Some(2);
    let b :Option<i32>  = None;
    // println!("{}",a)

    let c:Coin = Coin::One;
    Coin_match(c);
    let e:Coin = Coin::Two;
    Coin_match(e);
     */
    //
    // let f :i32 = 3;
    // Num(f);


    // let x :Option<i32> = Some(1);
    // let y = plus_one(x);
    // println!("{:?}",y)

    let some_u8_value = Some(100);

    if let  Some(i) = some_u8_value {
        println!("value={}",i)
    }else if let Some(4) = some_u8_value {
        println!("four")
    }else {
        println!("nothing")
    }

    let s = Option::Some(10);
    let s1 = Some(10);


}



fn some_u8_value(x :Option<u8>){
    match x {
        Some(3) => println!("three"),
        _ => (),
    }
}


enum Coin{
    One,
    Two,
}

fn Coin_match(coin: Coin){
    match coin {
        Coin::One => {
            println!("{}",String::from("hello"));
            println!("{}",1);
        },
        Coin::Two => {
            println!("{}",String::from("world"));
            println!("{}",2)
        }
    }
}

fn Num(n :i32){
    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}