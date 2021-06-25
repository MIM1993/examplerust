use std::ptr::null;
use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
    let t = vec![3,6,2,1,5,100,7,3];
    // let r = largest(&t);
    //
    // println!("{}",r);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let x = largestchar(&char_list);
    // println!("{}",x);

    let z = largest_all(&char_list);
    println!("{}",z);
    let w = largest_all(&t);
    println!("{}",w);


    // let p = Point{
    //     x:5,
    //     y:1.1,
    // };
    // let p2 = Point{
    //     x: String::from("hello"),
    //     y: 'x',
    // };
    // let p3 = p.mixup(p2);
    //
    // println!("{:#?}",p3)

}

fn largest(list: &[i32])->i32{
    if list.len() <= 0{
        return -1
    };
    let mut l = list[0];

    for &i in list {
        if i> l{
            l = i;
        }
    };
    l
}


fn largestchar(list: &[char])->char{
    if list.len() <= 0 {
       return '.'
    }
    let mut l = list[0];
   
   for &i in list {
       if i > l{
           l = i;
       }
   }
    l
}

fn largest_all<T: PartialOrd + Copy>(list: &[T])->T{
    // let x:T =
    // if list.len() <=0 {
    //     return T
    // }
    let mut s =  list[0];
    for &item in list.iter(){
        if s < item{
            s = item;
        }
    }
    s
}

#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y:U,
}

impl <T,U> Point<T,U> {
    fn mixup<V,W> (self,other:Point<V,W>)->Point<T,W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

//范型方法+trait bound
impl <T:Display,U:Copy> Point<T,U> {
    fn mixup<V,W> (self,other:Point<V,W>)->Point<T,W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}




// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
