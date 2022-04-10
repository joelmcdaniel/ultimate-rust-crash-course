// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use exercise_c_simple_types::{ding, on_off, print_array, print_difference, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    // print_difference(coords.0, coords.1);

    let (x, y) = coords;
    print_difference(x, y);

    let coords_arr: [f32; 2] = [x, y];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}

/* fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    // print_difference(coords.0, coords.1);

    let (x, y) = coords;
    print_difference(x, y);

    let coords_array: [f32; 2] = [x, y];
    print_difference(coords_array[0], coords_array[1]);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len() - 1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);
}

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
} */
