#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirts = 0;
        let mut blue_shirts = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_shirts += 1,
                ShirtColor::Blue => blue_shirts += 1,
            }
        }
        if red_shirts > blue_shirts {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let inv = Inventory{shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]};

    let user_pref1 = Some(ShirtColor::Red);
    let given1 = inv.giveaway(user_pref1);
    println!("User with preference {:?} gets {:?}", user_pref1, given1);

    let user_pref2 = None;
    let given2 = inv.giveaway(user_pref2);
    println!("User with preference {:?} gets {:?}", user_pref2, given2);
}
