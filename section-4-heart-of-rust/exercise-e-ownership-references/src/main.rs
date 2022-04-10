// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // Either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    println!("1 + 2 = {}, even via references", add(&1, &2));
}

// inspect takes a reference to a String, returns nothing, but
// prints whether the contents of the String is plural or singular.
fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

// change takes a *mutable* reference to a String and adds an "s" to
// the String if it doesn't already end with "s"
fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

// eat accepts ownership of (consumes) a String and returns a bool
// indicating whether or not the String both starts with a "b" AND contains an "a"
fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

// Takes *references* to two integer arguments,
// dereferences them and adds them together, and returns the result.
fn add(i1: &i32, i2: &i32) -> i32 {
    *i1 + *i2
}
