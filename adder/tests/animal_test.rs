use adder::animal::dog;

#[test]
fn testanimal(){
    assert_eq!(true, dog::is_dog())
}