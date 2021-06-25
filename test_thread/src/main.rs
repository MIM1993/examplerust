// use std::thread;
// use std::time::Duration;
// use serde_json;

fn main() {
    // let handle = thread::spawn(||{
    //    for i in 1..10{
    //        println!("spawn {}",i);
    //        thread::sleep(Duration::from_millis(1));
    //    }
    // });
    //
    // for j in 100..110{
    //     println!("main {}",j);
    //     // thread::sleep(Duration::from_millis(1));
    // }
    //
    // handle.join().unwrap();
    //
    // let arr = vec![1, 2, 3];
    //
    // let handle = thread::spawn(move || println!("{:?}", arr));
    //
    // //所有权已经不在主线程
    // drop(arr);
    //
    // handle.join().unwrap();

    // let mut arr: Vec<i32> = Vec::new();
    //
    // for i in 1..10 {
    //     let tmp = i + 1;
    //     arr.push(tmp);
    // }
    // println!("{:?}", arr)

    let data = r###"{"name": "John Doe","age": "43","address": {"street": "10 Downing Street","city": "London"},"phones": ["+44 1234567","+44 2345678"]}"###;
    // let data = r###"sdsa"###;
    let data = data.as_bytes();

    let json_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(data);
    let mut arr = vec![];
    // println!("{}",json_data)
    if json_data.is_err() {
        let err = json_data.err().unwrap();
        let err_str = err.to_string();
        println!("{}", err_str);
        return;
    } else {
        let json_data: serde_json::Value = json_data.ok().unwrap();
        // let name = json_data["name"].as_array().clone();
        let age = json_data["age"].as_str().unwrap().to_string();
        // let s = String::from(age);
        // println!("{}", age);
        // println!("{}", name);

        // arr.push(name);
        arr.push(age);

        // println!("{}", name.len());
        // println!("ssss{}", json_data["name"].as_str().unwrap_or_default());

        // let state = State {
        //     data: json_data.to_string(),
        //     last: "".to_string(),
        // };
        // let json = serde_json::to_value(&state).unwrap();
        // println!("{}", json);
    }

    println!("{:?}",arr)
}

struct State {
    data: String,
    last: String,
}

#[test]
pub fn test() {
    let data = r###"{"name": "John Doe","age": "43","address": {"street": "10 Downing Street","city": "London"},"phones": ["+44 1234567","+44 2345678"]}"###;
    // let data = r###"sdsa"###;
    let data = data.as_bytes();

    let json_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(data);
    let mut arr = vec![];
    // println!("{}",json_data)
    if json_data.is_err() {
        let err = json_data.err().unwrap();
        let err_str = err.to_string();
        println!("{}", err_str);
        return;
    } else {
        let json_data: serde_json::Value = json_data.ok().unwrap();
        let name = json_data["name"].as_str().unwrap().to_string();
        // let age = json_data["age"].as_array();
        // println!("{}", age);
        // println!("{}", name);

        arr.push(name);
        // arr.push(age);

        // println!("{}", name.len());
        // println!("ssss{}", json_data["name"].as_str().unwrap_or_default());

        // let state = State {
        //     data: json_data.to_string(),
        //     last: "".to_string(),
        // };
        // let json = serde_json::to_value(&state).unwrap();
        // println!("{}", json);
    }

    println!("{:?}",arr)
}
//
// #[test]
// pub fn test1() {
//     let mut arr:Vec<i32> = Vec::new();
//
//     for i in 1..10{
//
//         let tmp = i+1;
//         arr.push(tmp)
//     }
//
//     println!(":?",arr)
// }


fn test_vec() -> Vec<String> {
    let s = "a,b,c";
    s.split(",").map(String::from).collect::<Vec<String>>()
}

#[test]
fn test_vec1(){
    let s = test_vec();
    for i in s {
        println!("{}",i)
    }
}