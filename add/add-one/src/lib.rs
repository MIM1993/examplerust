#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_works() {

        assert_eq!(2, add_one(1));
    }
}

use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}



