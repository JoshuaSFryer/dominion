#[derive(Debug)]
struct Silver {
    basic: BasicCard,
    mun: Treasure,
}
impl Silver {
    fn create() -> Self {
        Self {
            basic: BasicCard::create("Silver", 3),
            mun: Treasure { value: 2 },
        }
    }
}
impl Playable for Silver {
    fn on_play(&self) {
        println!("Add {} Money!", self.mun.value);
    }
}

#[derive(Debug)]
pub struct BasicCard {
    price: i32,
    name: String,
}

impl BasicCard {
    fn create(name: &str, price: i32) -> Self {
        Self {
            price,
            name: String::from(name),
        }
    }
}

#[derive(Debug)]
pub struct Treasure {
    value: i32,
}

impl Treasure {
    fn get_value(&self) -> i32 {
        self.value
    }
}

pub trait Playable {
    fn on_play(&self);
}

fn main() {
    println!("Hello, world!");
    let my_silver = BasicCard::create("Silver", 3);
    let my_other_silver = Silver::create();
    Silver::on_play(&my_other_silver);
    println!("{:?}", my_other_silver);
}
