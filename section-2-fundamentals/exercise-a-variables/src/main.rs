const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    /* let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles); */

    /* let missiles = 8; // Fix error by doing: let mut missiles = 8
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready; // Error!
    println!("{} missiles left", missiles); */

    /* let mut missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles); */

    /* let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles); */

    /* let (mut missiles, ready) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {}, of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles); */

    /*  let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles); */

    /* let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready); */

    /* let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready); */

    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let _jet: String;
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);

    /* let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let _jet: String;
    READY_AMOUNT = 1; // Error invalid left-hand side of assignment
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready); */
}
