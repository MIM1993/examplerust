
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;
use std::path::Path;

fn main() {
    // panic!("debug")
    // let arr = vec![1,2,3];
    // v[99];

    // 创建指向所需的文件的路径
    let path = Path::new("hello.txt");
    path
    let display = path.display();
    println!("{:?}",display.to_string());


    let f = File::open(&path);

    let mut f1 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&path) {
                Ok(fc)=>fc,
                Err(error)=>panic!("create err:{:?}",error)
            },
            othe_errr => panic!("other err: {:?}",othe_errr)
        }
    };

    //f1必须是可变变量
   f1.write_all(String::from("hello world").as_bytes()).expect("write failed");



}
