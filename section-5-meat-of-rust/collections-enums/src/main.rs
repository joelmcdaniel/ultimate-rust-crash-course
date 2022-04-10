use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

impl DispenserItem {
    fn display(&self) {}
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop(); // x is 6

    println!("x = {:?}", x); // x = Some(6)
    println!("{}", v[1]); // 4

    let mut v = vec![2, 4, 6];
    println!("{}", v[2]);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();
    println!("have_five = {}", have_five);

    let color = Color::Red;
    println!("color is {:#?}", color);

    use DispenserItem::*;
    let item = Ammo(69);
    println!("item = {:#?}", item);

    /* enum Option<T> {
        Some(T),
        None,
    } */

    let mut x = None; // x: Option<i32> is infered
    x = Some(5);
    x.is_some(); // true
    x.is_none(); // false
    for i in x {
        println!("{}", i); // prints 5
    }

    let my_variable = Option::Some(5);

    if let Some(x) = my_variable {
        println!("value is {}", x);
    }

    match my_variable {
        Some(x) => {
            println!("value is {}", x);
        }
        None => {
            println!("no value");
        }
    }

    /* #[must_use]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    } */

    let res = File::open("foo");
    if res.is_ok() {
        let f = res.unwrap();
    }
    /* match res {
        Ok(f) => { /* do stuff */ }
        Err(e) => { /* do stuff */ }
    } */
}
