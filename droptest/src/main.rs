use std::mem::drop;

fn main() {

    let a = CustomSmartPointer{
        data:String::from("hello"),
    };


    // a.drop();

    drop(a);


    let b = CustomSmartPointer{
      data:String::from("world"),
    };

    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!",self.data)
    }
}
