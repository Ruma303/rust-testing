pub mod category;
use crate::product::category::Category;

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub category: Category,
}
impl Product {
    pub fn new(id: u32, name: String, price: f64, category: Category) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }

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

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
    }
}
