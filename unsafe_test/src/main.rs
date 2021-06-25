use std::ops::Add;
use std::net::Shutdown::Write;


fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

#[test]
fn test001() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2)
    }
}

#[test]
fn test002() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // let r = &mut arr[..];

    let (a, b) = arr.split_at_mut(5);

    println!("a = {:?}", a);
    println!("b = {:?}", b)
}

#[test]
fn test003() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

#[test]
fn test004() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe { println!("abs: {}", abs(-100)) }
}

#[no_mangle]
pub extern "C" fn call_from_C() {
    println!("hello")
}

#[test]
fn test005() {
    unsafe trait Foo {}

    struct Test {}

    unsafe impl Foo for Test {}

    struct Counter {}

    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    impl Iterator<String> for Counter {
        fn next(&mut self) -> Option<String> {
            Some(String::from("hello"))
        }
    }
}

#[test]
fn test006() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let tmp1 = Point{x:1,y:2};
    let tmp2 = Point{x:100,y:200};

    let tmp3 = tmp1.add(tmp2);
    println!("{:?}",tmp3)
}


#[test]
fn test007(){
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    person.fly();
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);
}


#[test]
fn test008(){
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    println!(" {}", <Dog as Animal>::baby_name());

}


#[test]
fn test009(){

    /*
    **********
    *        *
    * (1, 3) *
    *        *
    **********
    */

    use std::fmt;

    trait OutlinePrint: fmt::Display{
        fn outline_print(&self){
            let output = self.to_string();
            let len = output.len();
            println!("{}","*".repeat(len+4));
            println!("*{}*"," ".repeat(len+2));
            println!("* {} *",output);
            println!("*{}*"," ".repeat(len+2));
            println!("{}","*".repeat(len+4));
        }
    }


    struct Person{
        name:String,
    }

    impl fmt::Display for Person{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f,"{}",self.name)
        }

    }

    impl OutlinePrint for Person{}

    let tmp = Person{name:String::from("hello")};

    tmp.outline_print();

}


#[test]
fn test010(){
    use std::fmt;
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f,"[{}]",self.0.join("## "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("{}",w)

}



#[test]
fn test011(){
    fn add_one(x:i32)->i32{
        x+1
    }

    fn do_twice(f:fn(i32)->i32,x:i32)->i32{
        f(x) + f(x+1)
    }


    let tmp = do_twice(add_one,100);
    println!("{}",tmp)
}


#[test]
fn test012(){

    let nums = vec![1,2,3,4,5];
    let s: Vec<String> = nums.iter().map(|i|i.to_string()).collect();

    println!("{:?}",s)
}


#[test]
fn test013(){
    enum Status{
        Value(u32),
        Stop
    }

    let status_list:Vec<Status> = (0u32..20).map(Status::Value).collect();

}

#[test]
fn test014(){
    fn returns_closure()-> Box<dyn Fn(i32)->i32>{
        Box::new(|x|x+1)
    }

}