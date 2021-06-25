pub mod test_a{
    #[derive(Debug)]
    pub struct one {
        pub name : String,
        age : i8,
    }

    impl one{
        pub fn print_struct(&self){
            println!("name = {}, age ={}",self.name,self.age)
        }
    }

    pub fn new_one(name:String,age:i8) -> one{
        one{
            name:name,
            age:age,
        }
    }
}