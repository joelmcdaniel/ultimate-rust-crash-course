fn main() {
    let x = do_stuff(2.0, 12.5); // It works!
    println!("x = {}", x);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz sarsaparilla(s)", qty, oz);
    // return qty * oz;
    qty * oz // tail expression
}
