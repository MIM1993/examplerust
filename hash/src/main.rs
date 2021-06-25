use std::ffi::OsStr;

fn main() {
    use std::collections::HashMap;
    let mut score :HashMap<String,i32> = HashMap::new();

    score.insert(String::from("hello"),1);
    score.insert(String::from("world"),2);


    println!("{:?}",score);


    let temp :Vec<String> = vec![String::from("blue"),String::from("yellow")];
    let initial_scores :Vec<i8> = vec![10,20];
    let score1:HashMap<_,_>= temp.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",score1);

    let key :String = String::from("key");
    let field:String = String::from("field");
    let mut tmp = HashMap::new();
    tmp.insert(key,field);
    // println!("{}",key);
    println!("{:?}",tmp);


    let s = tmp.get(&String::from("key"));
    if let Some(v) = s {
        println!("{}",v)
    }else {
        println!("none")
    }

    let mut tmp1 = HashMap::new();
    tmp1.insert("a",1);
    tmp1.insert("b",2);
    tmp1.insert("c",3);
    tmp1.insert("d",4);
    for (k,v) in &tmp1 {
        println!("{}",format!("key{}  val{}",k,v))
    }

    tmp1.insert("a",100);

    let a = tmp1.get("a");
    match a {
        Some(v) => println!("{}",v),
        None => println!("none")
    }


    let text = "hello world wonderful world";

    let mut map:HashMap<&str,i8> = HashMap::new();

    for word in text.split_whitespace(){
        let c = map.entry(word).or_insert(0);
        *c += 1;
    }

    println!("{:?}",map)

}

