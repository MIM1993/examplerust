fn main() {
    let mut p = Post::new();
    p.add_text("hello world !");
    println!("1==={}",p.content());

    p.request_review();
    println!("2==={}",p.content());

    p.approve();
    println!("3==={}",p.content());

}

struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}


impl Post{
    pub fn new()->Post{
        Post{
            state:Some(Box::new(Draft{})),
            content:String::new(),
        }
    }

    pub fn add_text(&mut self,text :&str){
        self.content.push_str(text)
    }

    pub fn content(&self)->&str{
        self.state.as_ref().unwrap().content(self)//todo
    }

    pub  fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }

}


//状态接口
trait State{
    fn request_review(self:Box<Self>) -> Box<dyn State>;
    fn approve(self:Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self,_post:&'a Post)->&'a str{
        ""
    }
}

//草案
struct Draft{}

impl State for Draft{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        Box::new(PendingReview{})
    }
    fn approve(self:Box<Self>)->Box<dyn State>{
        self
    }
}

//等待审核
struct PendingReview{}

impl State for PendingReview{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        self
    }

    fn approve(self:Box<Self>)->Box<dyn State>{
        Box::new(Published{})
    }
}

//已发表
struct Published{}

impl State for Published{
    fn request_review(self:Box<Self>)->Box<dyn State>{
        self
    }

    fn approve(self:Box<Self>)->Box<dyn State>{
        self
    }

    fn content<'a>(&self,post:&'a Post)->&'a str{
        &post.content
    }
}