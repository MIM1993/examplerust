#[cfg(test)]
mod tests {

    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut one = self.sent_messages.borrow_mut();
            one.push(String::from(msg));
            // let mut two = self.sent_messages.borrow_mut();
            // two.push(String::from(msg));
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message(){
        let m = MockMessenger::new();
        let mut l = LimitTracker::new(&m,10);
        l.set_value(8);
        l.set_value(9);
        l.set_value(10);
        assert_eq!(m.sent_messages.borrow().len(), 3);
        println!("{:?}",m.sent_messages)
    }
}

//消息接口
pub trait Messenger {
    fn send(&self, msg: &str);
}

//限制器是例
pub struct LimitTracker<'a, T: Messenger> {
    message: &'a T,
    max_val: usize,
    value: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            message: messager,
            max_val: max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, val: usize) {
        self.value = val;

        let percentage_of_max = self.max_val as f64 / val as f64;

        if percentage_of_max > 1.0 {
            self.message.send("Error: You are over your quota!")
        } else if percentage_of_max > 0.9 {
            self.message
                .send("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_max > 0.7 {
            self.message
                .send("Warning: You've used up over 75% of your quota!")
        };
    }
}
