// Funzione da testare
// fn somma(a: i32, b: i32) -> i32 {
//     a + b
// }

// // Modulo per testing
// #[cfg(test)]
// mod tests {
//     use super::*;  // Importa le funzioni del modulo principale

//     #[test] // Testiamo la funzione somma()
//     fn test_somma() {
//         assert_eq!(somma(2, 3), 5);  // Verifica che somma(2, 3) restituisca 5
//     }
// }

pub mod shapes {
    #[derive(Debug)]
    pub struct Circle {
        pub radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
            if radius > 0.0 {
                Ok(Circle { radius })
            } else {
                Err("Radius must be positive".to_string())
            }
        }

        pub fn new_2(radius: f32) -> Result<Circle, String> {
            match radius {
                r if r <= 0.0 => panic!("Radius must be positive"),
                _ => Ok(Circle { radius }),
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

fn private_fn() {}

// Unit test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);

        assert_eq!(
            larger_circle.contains(&smaller_circle),
            true,
            "The larger circle should contain the smaller circle"
        );

        assert_ne!(
            larger_circle.contains(&smaller_circle),
            false,
            "This should never be false"
        );

        assert!(
            larger_circle.contains(&smaller_circle),
            "Larger circle should contain the smaller one"
        );
    }

    #[test]
    fn new_circle_should_have_positive_radius() {
        let result = shapes::Circle::new_1(1.0);
        assert!(
            result.is_ok(),
            "Creating a circle with a negative radius is not allowed. Result: {:?}",
            result.unwrap_err()
        );
    }

    #[test]
    #[should_panic(expected = "Radius must be positive")]
    fn should_not_create_circle_with_negative_radius() {
        shapes::Circle::new_2(-1.0).unwrap(); // unwrap() necessario per attivare il panic
    }

    #[test]
    fn private_function_should_not_be_accessible() {
        private_fn();
    }
}
