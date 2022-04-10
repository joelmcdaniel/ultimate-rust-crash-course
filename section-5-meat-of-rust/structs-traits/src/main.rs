struct RedFox {
    enemy: bool,
    life: u32,
}

// implementation block
impl RedFox {
    // associated function
    // (like class method, i.e. constructor)
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    // methods (always take some form of self)
    // fn move(self)...
    // fn borrow(&self)...
    // fn mut_borrow(&mut self)
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

// trait with default behavior block
trait Run {
    fn run(&self) {
        println!("I'm running");
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    /* let fox = RedFox {
        enemy: true,
        life: 70,
    }; */

    let fox = RedFox::new();
    let life_left = fox.life;
    let is_enemy = fox.enemy;
    // fox.enemy = false; // Error (not mutable)
    println!("{}", fox.get_noise());

    print_noise(5_u8);

    let robot = Robot {};
    robot.run();
}
