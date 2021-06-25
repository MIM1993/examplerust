use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    panic!("debug")
    // let arr = vec![1,2,3];
    // v[99];

    // 创建指向所需的文件的路径
    // let path = Path::new("hello.txt");
    // let display = path.display();
    // println!("{:?}", display.to_string());
    //
    // let f = File::open(&path);
    //
    // let mut f1 = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(&path) {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("create err:{:?}", error),
    //         },
    //         othe_errr => panic!("other err: {:?}", othe_errr),
    //     },
    // };
    //
    // //f1必须是可变变量
    // f1.write_all(String::from("hello world").as_bytes())
    //     .expect("write failed");

    // match read_username_from_file1(){
    //     Ok(v)=> println!("{}",v),
    //     Err(e) => panic!("{:?}",e)
    // }

}

fn read_username_from_file() -> Result<String, io::Error> {
    let path = Path::new("hello.txt");
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    let r = match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };

    return r;
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let path = Path::new("hello.txt");
    let mut s = String::new();

    let mut f = File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}