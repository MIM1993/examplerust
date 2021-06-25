fn main() {
    let rect = Rectangle { wide: 10, high: 20 };
    //打印
    println!("{:#?}", rect);
    //计算面积
    let a = area(&rect);
    let b = rect.area();
    println!("{}", a);
    println!("{}", b);

    let rect2 = Rectangle { wide: 1, high: 2 };

    let res = rect.can_hold(&rect2);
    println!("{}", res);

    let square = Rectangle::square(3);
    println!("{:#?}", square)
}

//长方形结构体
#[derive(Debug)]
struct Rectangle {
    wide: i32,
    high: i32,
}

//Rectangle 方法
impl Rectangle {
    //计算面积
    fn area(&self) -> i32 {
        self.wide * self.high
    }
    //是否能包住
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.high > rect.high && self.wide > rect.wide
    }

    //正方形
    fn square(l: i32) -> Rectangle {
        Rectangle { wide: l, high: l }
    }
}

//计算面积
fn area(rect: &Rectangle) -> i32 {
    rect.wide * rect.high
}
