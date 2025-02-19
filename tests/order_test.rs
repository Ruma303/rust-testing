use testing::order::Order;
use testing::customer::Customer;
use testing::product::Product;
use testing::product::category::Category;
use testing::order::order_status::OrderStatus;

mod helpers;

#[test]
fn test_total_bill_without_discount() {
    helpers::setup();
    let customer = Customer::new(1, "John Doe".to_string(), "john@example.com".to_string(), "123456789".to_string());
    let product = Product::new(1, "Widget".to_string(), 10.0, Category::Electronics);
    let order = Order::new(customer, vec![product], OrderStatus::Pending, 1, "123 Street".to_string(), "TRACK123".to_string());

    assert_eq!(format!("{:.2}", order.calculate_total_price()), "10.00");
}

#[test]
fn test_total_bill_with_discount() {
    let customer = Customer::new(2, "Jane Smith".to_string(), "jane@example.com".to_string(), "987654321".to_string());
    let product = Product::new(2, "Gadget".to_string(), 20.0, Category::Electronics);
    let order = Order::new(customer, vec![product], OrderStatus::Pending, 1, "456 Avenue".to_string(), "TRACK456".to_string());

    assert_eq!(format!("{:.2}", order.calculate_total_price()), "20.00");
}
