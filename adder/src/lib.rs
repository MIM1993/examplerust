pub mod animal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    use crate::animal::dog;

    #[test]
    fn testanimal(){
        assert_eq!(true, dog::is_dog())
    }
}
