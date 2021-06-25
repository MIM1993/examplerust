

fn main() {
    // let x = vec![1,2,3];
    //
    // let equal_to_x =  move |y| x == y;
    //
    // let y= vec![1,2,3];
    //
    // println!("{:?}",x);
    //
    // println!("{}",equal_to_x(y));


    let big_val = std::i32::MAX;
    // let x = big_val+1;
    let x = big_val.wrapping_add(1);
    println!("{}",x)

}
//
// fn main() {
//     let x = vec![1, 2, 3];
//
//     let equal_to_x = move |z| z == x;
//
//     println!("can't use x here: {:?}", x);
//
//     let y = vec![1, 2, 3];
//
//     assert!(equal_to_x(y));
// }

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calculation: calc,
            value: None,
        }
    }

    fn value(&mut self, val: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let tmp = (self.calculation)(val);
                self.value = Some(tmp);
                tmp
            }
        }
    }
}
