#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub fn add_2(i: i32) -> i32 {
    i + 2
}

#[derive(Debug)]
struct Guess {
    v: i32,
}

impl Guess {
    fn new(v: i32) -> Guess {
        if v < 1 {
            panic!("guess should be greater than 0");
        }
        if v > 100 {
            panic!("guess should be less than 100");
        }
        Guess{v}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle{
            w: 10,
            h: 15,
        };
        let r2 = Rectangle{
            w: 8,
            h: 12,
        };
        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle{
            w: 10,
            h: 15,
        };
        let r2 = Rectangle{
            w: 12,
            h: 20,
        };
        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn it_adds_two() -> Result<(), String> {
        if add_2(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic(expected = "less than 100")]
    fn invalid_guess_panics() {
        Guess::new(500);
    }
}
