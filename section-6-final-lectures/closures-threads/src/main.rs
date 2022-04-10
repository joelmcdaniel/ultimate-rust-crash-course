use std::thread;

fn main() {
    // Closures

    let add = |x, y| x + y;

    println!("{}", add(1, 2)); // prints 3

    let s = "🍓".to_string();
    let f = move || {
        println!("{}", s); // s is owned by the closure
    };

    f(); // prints 🍓

    let v = vec![2, 4, 6];

    let rslt = v
        .iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);

    println!("{}", rslt); // prints 30

    // Threads
    let handle = thread::spawn(move || {
        // do stuff in a child thread
    });

    // do stuff simultaneously in the main thread

    // wait until thread has exited
    handle.join().unwrap();
}
