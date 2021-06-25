fn main() {
    let s = "hello world";
    println!("{}",s);
    let mut s1 = s.to_string();
    println!("{}",s1);

    s1.push_str(" !");
    println!("{}",s1);

   let mut s2 =  String::from("السلام عليكم");
    println!("{}",s2);

    let hello = String::from("안녕하세요");
    println!("{}",hello);


    let s3 = String::from("hello ");
    let s4 = String::from("world");

    // let s5 = s3 + &s4;
    // println!("{}",s5);
    // println!("{}",s3);
    // println!("{}",s4);

    let s6 = format!("{} {} !",s3,s4);
    println!("{}",s6);

    let h = &s6[0..];
    println!("{}",h);


    for c in "नमस्ते".chars(){
        println!("{}",c)
    }

    let mut n:i32 = 0;
    for b in "नमस्ते".bytes(){
        println!("{}",b);
        n=n+1;
    }
    println!("{}",n);


}
