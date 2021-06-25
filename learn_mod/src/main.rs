
mod A {
    pub fn p(){
        println!("A")
    }
}

use mylib::factory as mf;

use mylib::testmod as test;

//不能这样写
use mylib::mysql::conn::*;

fn main() {

    // A::p();;
    mf::car_a::print();
    mf::car_b::print();


    let mut o =test::test_a::new_one(String::from("mim"),25);
    println!("{:#?}",o.name);
    o.print_struct();

   let mut mc =  c{
        addr: String::from("192.168.1.0"),
       port : String::from("8888")
    };

    println!("{:#?}",mc);
    mc.conn()

}
