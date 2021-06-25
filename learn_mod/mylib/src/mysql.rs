pub mod conn{
     #[derive(Debug)]
     pub struct c{
         pub addr: String,
         pub port:String,
     }

   impl c{
        pub fn conn(&self){
            println!("conn ...")
        }
    }

}