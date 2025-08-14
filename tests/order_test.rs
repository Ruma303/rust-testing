// Importa tipi dalla libreria principale (crate)
use testing::customer::customer::Customer;
use testing::product::{Product, category::Category};
use testing::order::{Order, order_status::OrderStatus};

// Importa il modulo di supporto per i test (tests/helpers/mod.rs)
mod helpers;

#[test]
fn test_total_bill_without_discount() {
    helpers::setup();

    let customer = Customer::new(
        1,
        "John Doe".to_string(),
        "john@example.com".to_string(),
        "123456789".to_string(),
    );

    let product = Product::new(
        1,
        "Widget".to_string(),
        10.0,
        Category::Electronics,
    );

    let order = Order::new(
        customer,
        vec![product],
        OrderStatus::Pending,
        1,
        "123 Street".to_string(),
        "TRACK123".to_string(),
    );

    assert_eq!(format!("{:.2}", order.calculate_total_price()), "10.00");
}

#[test]
fn test_total_bill_with_discount() {
    helpers::setup();

    let customer = Customer::new(
        2,
        "Jane Smith".to_string(),
        "jane@example.com".to_string(),
        "987654321".to_string(),
    );

    let product = Product::new(
        2,
        "Gadget".to_string(),
        20.0,
        Category::Electronics,
    );

    let order = Order::new(
        customer,
        vec![product],
        OrderStatus::Pending,
        1,
        "456 Avenue".to_string(),
        "TRACK456".to_string(),
    );

    assert_eq!(format!("{:.2}", order.calculate_total_price()), "20.00");
}

#[test]
fn test_order_status_is_delivered() {
    helpers::setup();

    let customer = Customer::new(
        3,
        "Mario Rossi".to_string(),
        "mario@example.com".to_string(),
        "555555555".to_string(),
    );
    let product = Product::new(
        3,
        "Laptop".to_string(),
        1500.0,
        Category::Electronics,
    );

    let order = Order::new(
        customer,
        vec![product],
        OrderStatus::Delivered,
        1,
        "Via Roma 1".to_string(),
        "TRACK789".to_string(),
    );

    assert!(order.status.is_delivered(), "L'ordine dovrebbe risultare consegnato");
}
