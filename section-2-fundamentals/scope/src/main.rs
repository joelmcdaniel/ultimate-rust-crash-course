fn main() {
    /* let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    println!("{}, {}", x, y); // Error */

    /* let x = 5;
    {
        let x = 99; // shadowing in nested scope
        println!("{}", x); // Prints "99"
    }
    println!("{}", x); // Prints "5" */

    // shadowing in same scope
    /* let mut x = 5; // x is mutable
    let x = x; // x is now immutable
    println!("{}", x); */

    let meme = "More cowbell!";
    let meme = make_image(meme);
}
