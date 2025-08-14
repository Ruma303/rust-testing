#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
}

impl OrderStatus {
    pub fn is_delivered(&self) -> bool {
        matches!(self, OrderStatus::Delivered)
    }
}
