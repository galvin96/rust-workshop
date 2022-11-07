///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////


// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }



///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng 
///////////////////////////////////////////

// use std::ops::Add;

// // Implement the generic function below.
// fn sum<T: std::ops::Add + Add<Output = T>> (x: T, y: T) -> T {
//     x + y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }




///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


// fn main() {
//     let a = A {p: Some("p".to_string())};
//     a.a();
// }

// struct A {
//     p: Option<String>
// }


// impl A {
//     fn a(self) -> Self {
//         Self::b(&self.p.as_ref().unwrap());
//         self
//     }
//     fn b(b: &str) {
//         print!("b: {}", b)
//     }
// }





///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////






// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }



// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }


// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1;
//     }

//     pub fn get_val2(&self) -> String {
//         return self.val2.to_string();
//     }

//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1, self.val2.to_string());
//     }
// }



#![allow(dead_code)]

struct Store {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: String,
    price: f32,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn price(&self, item_name: &str) -> Option<f32> {
        for item in &self.items {
            if item.name == item_name {
                return Some(item.price);
            }
        }
        None
    }

    fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
        let mut sum = 0.0;
        for item in shopping_list {
            if self.price(item) == None {
                return None;
            }
            sum += self.price(item).unwrap();
        }
        Some(sum)
    }
}

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item { name: format!("chocolate"), price: 5.0 });
    store.add_item(Item { name: format!("socks"), price: 23.0 });
    store.add_item(Item { name: format!("plush Mozilla dinosaur"), price: 13.0 });
    store
}

#[test]
fn total_price() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), Some(18.0));
}

#[test]
fn total_price_missing() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur", "fork and knife"];
    assert_eq!(store.total_price(&list), None);
}
