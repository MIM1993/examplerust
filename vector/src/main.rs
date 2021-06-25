fn main() {
    //rintln!("Hello, world!");

    // let v1 :Vec<i32> = Vec::new();
    // v1.push(1);

    let mut v2 :Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    println!("{:?}",v2);

    //索引
    let a :&i32 = &v2[0];
    println!("{}",a);

    //get
    match v2.get(100){
        Some(val) => println!("val = {}",val),
        _ => println!("None")
    }

    //遍历
    for i in &mut v2 {
        println!("{}",*i)
    }

    //结合enum
    #[derive(Debug)]
    enum Context{
        Text(String),
        Int(i32),
        Float(f32),
    }

    let mut v3 = vec![
       Context::Text(String::from("Hello World")),
       Context::Int(100),
       Context::Float(0.01),
    ];

    println!("{:#?}",v3)

}
