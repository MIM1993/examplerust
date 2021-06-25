// extern crate animal;
use animal::animal::dog;
#[test]
fn it_dog() {
    assert_eq!(true, dog::is_dog(), "result `{}`", dog::is_dog());
}