use rust_testing::shapes::Circle;

fn main() {
    let c = Circle::new(10.0);
    println!("{:?}", c);
}

fn my_fn() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_can_create_circle() {
        let c = Circle::new(3.0);
        assert_eq!(c.radius, 3.0);
    }

    #[test]
    #[ignore]
    fn huge_test() {
      my_fn();
    }
}