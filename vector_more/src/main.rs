fn main() {}

#[test]
fn iterator_demonstration() {
    //  let tmp = vec![1,2,3];
    //  let mut tmp_iter = tmp.iter();
    //
    //  assert_eq!(tmp_iter.next(),Some(&1));
    //
    //
    // let t:i32 =  tmp_iter.sum();
    //  println!("{}",t);
    let tmp = vec![1, 2, 3];

    let v1: Vec<i32> = tmp.iter().map(|x| x + 1).collect();
    println!("{:?}", v1)
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == my_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let my_shoes = shoes_in_my_size(shoes, 10);
    // println!("{:?}",my_shoes)
    assert_eq!(
        my_shoes,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    )
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut c = Counter::new();

    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let tmp:Vec<u32> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();

        println!("{:?}",tmp)
}
