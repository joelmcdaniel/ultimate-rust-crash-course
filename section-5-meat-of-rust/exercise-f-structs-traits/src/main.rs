trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)]
struct Grapes {
    number_of_grapes: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.number_of_grapes -= 1;
    }
}

fn bunny_nibbles<T: Bite>(food: &mut T) {
    for _i in 1..=3 {
        food.bite();
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };

    // carrot.bite();
    // println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes {
        number_of_grapes: 100,
    };

    // grapes.bite();
    // println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles carrot for awhile: {:?}", carrot);

    bunny_nibbles(&mut grapes);
    println!("Bunny eats some grapes: {:?}", grapes);
}
