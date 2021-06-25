use std::fmt::{Display, Debug};
use std::iter::Sum;

fn main() {
    println!("Hello, world!");

    let x = NewsArticle {
        headline: String::from("hello"),
        location: String::from("world"),
        author: String::from("MIM"),
        content: String::from("100"),
    };

    let y = Tweet {
        username: String::from("god"),
        content: String::from("hahaha"),
        reply: true,
        retweet: false,
    };

    // let tmp = x.summarize();
    // println!("{}",tmp)

    notify(x);
    notify(y);

}

pub trait Summary {
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self)->String{
    //     format!("{} {} {}",self.author,self.headline,self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("notify : {}", item.summarize())
}

/*
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
*/

pub fn notify1<T:Summary>(item1:T,item2:T){

}

pub fn notify2<T,U>(item:T)
    where T: Summary + Display,
          U: Clone + Debug {

}

//