//! # Libreria per gestire i prodotti.
//! Questa libreria è per gestire i prodotti.
pub mod product {
    // Rendiamo esportabili per altri file e moduli

    pub mod category {
        /// Il trait PartialEq è utilizzato per confrontare i prodotti, è richiesto dal crate array_tool
        #[derive(Debug, PartialEq)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
            HomeGoods,
            Sports,
            Food,
            Beauty,
            Toys,
            Other,
        }
    }
    use crate::product::category::Category; // Importazione della categoria, sarà disponibile per il resto del modulo

    /// Il trait PartialEq è utilizzato per confrontare i prodotti, è richiesto dal crate array_tool
    #[derive(Debug, PartialEq)]
    pub struct Product {
        // Per le struct:
        pub id: u32, // Rendere pubblico ogni campo che useremo in altri moduli / file
        pub name: String,
        pub price: f64,
        pub category: crate::product::category::Category, // Esempio con percorso assoluti
    }

    impl Product {
        /// # Testing
        /// ```
        /// use testing::product::category::Category;
        /// use testing::product::Product;
        ///
        /// let product = Product::new(1, "Product".to_string(), 10.0, Category::Electronics);
        /// assert_eq!(product.get_id(), 1);
        /// ```

        pub fn new(id: u32, name: String, price: f64, category: Category) -> Self {
            Product {
                id,
                name,
                price,
                category,
            }
        }

        // ✅ Getter pubblici (solo lettura)
        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_price(&self) -> f64 {
            self.price
        }

        pub fn get_category(&self) -> &Category {
            &self.category
        }

        // ✅ Setter privati (modificano lo stato)
        fn set_id(&mut self, id: u32) {
            self.id = id;
        }

        fn set_price(&mut self, price: f64) {
            self.price = price;
        }
    }
}

pub mod order {

    pub mod order_status {
        #[derive(Debug)]
        pub enum OrderStatus {
            Pending,
            Processing,
            Shipped,
            Delivered,
            Cancelled,
        }
    }

    use self::order_status::OrderStatus; // Importazione da altro modulo pubblico esposto
    use crate::customer::Customer;
    use crate::product::Product;

    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub products: Vec<Product>,
        pub customer: Customer,
        pub status: OrderStatus,
        pub quantity: u32,
        pub total_price: f64,
        pub shipping_address: String,
        pub tracking_number: String,
    }

    impl Order {
        pub fn new(
            customer: Customer,
            products: Vec<Product>,
            status: OrderStatus,
            quantity: u32,
            shipping_address: String,
            tracking_number: String,
        ) -> Self {
            let total_price = products.iter().map(|p| p.get_price()).sum();
            Order {
                id: 0,
                products,
                customer,
                status,
                quantity,
                total_price,
                shipping_address,
                tracking_number,
            }
        }

        pub fn calculate_total_price(&self) -> f64 {
            self.products.iter().map(|p| p.get_price()).sum()
        }

        pub fn calculate_shipping_cost(&self) -> f64 {
            let prices: Vec<f64> = self.products.iter().map(|p| p.get_price()).collect();
            println!("Prezzi prodotti: {:?}", prices);

            prices.iter().sum::<f64>() * 0.05
        }

        fn calculate_discount(&self, discount_percentage: f64) -> f64 {
            self.calculate_total_price() * (1.0 - discount_percentage / 100.0)
        }

        fn calculate_final_price(&self, discount_percentage: f64) -> f64 {
            self.calculate_discount(discount_percentage) + self.calculate_shipping_cost()
        }

        pub fn get_order_customer(&self) -> &Customer {
            &self.customer
        }
    }
}

pub mod customer {

    #[derive(Debug)]
    pub struct Customer {
        pub id: u64,
        pub name: String,
        pub email: String,
        pub phone: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String, phone: String) -> Self {
            Customer {
                id,
                name,
                email,
                phone,
            }
        }

        pub fn get_id(&self) -> u64 {
            // ✅ Ora è pubblico
            self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_email(&self) -> &str {
            &self.email
        }

        pub fn get_phone(&self) -> &str {
            &self.phone
        }
    }
}
