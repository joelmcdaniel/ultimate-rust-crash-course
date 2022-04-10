fn main() {
    // Ownership - only one variable can own the value, value moved to s2
    /* let s1 = String::from("abc");
       let s2 = s1;
       println!("{}", s1); // Error!
    */

    // Copy is only stack data, clone is stack and heap (deep copy)
    let s1 = String::from("abc");
    let s2 = s1.clone(); // deep copy
    println!("{}", s1);

    /* let s1 = String::from("abc");
    do_stuff(s1);
    println!("{}", s1); // Error, moved!

    fn do_stuff(s: String) {
        // do stuff
    } */

    // References and Borrowing
    /* let s1 = String::from("abc");
    do_stuff(&s1);
    println!("{}", s1);

    fn do_stuff(s: &String) {
        // do stuff
    } */

    let mut s1 = String::from("abc");
    do_stuff(&mut s1);
    println!("{}", s1);

    fn do_stuff(s: &mut String) {
        s.insert_str(0, "Hi, "); // . operator on method or field auto-dereferences down to value
                                 // manual dereference
                                 // (*s).insert_str(0, "Hi, ");
    }
}
