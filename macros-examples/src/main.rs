// item: An item.
// block: A block expression.
// stmt: A statement.
// pat: A pattern.
// expr: An expression.
// ty: A type.
// ident: An identifier or a keyword.
// path: A type path.
// tt: A token.
// meta: The contents of an attribute.
// lifetime: A lifetime token.
// vis: A possibly empty visibility qualifier.
// literal: A literal expression.

type Date = String;

#[allow(dead_code)]
#[derive(Debug)]
struct Product {
    name: String,
    cost: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ProductDetail {
    product: Product,
    discount: Option<f64>,
    tax: f64,
    price: Option<f64>,
}

// Macros are the Rust metaprogramming feature that creates code in compile-time.
macro_rules! create_model {
    ($name:ident) => {
        #[allow(unused_variables)]
        #[allow(dead_code)]
        #[derive(Debug)]
        struct $name {
            product_details: Vec<ProductDetail>,
            date: Date,
            subtotal: Option<f64>,
            total_tax: f64,
            total: Option<f64>,
        }
    };
}

create_model!(Sale);
create_model!(Purchase);
create_model!(Budget);

macro_rules! calculate_total {
    ($name:ident, $model:ty) => {
        trait $name {
            fn calculate_subtotal(data_model: &$model) -> f64 {
                data_model
                    .product_details
                    .iter()
                    .map(|product_detail| {
                        product_detail.price.unwrap_or(0.0) - product_detail.discount.unwrap_or(0.0)
                    })
                    .sum()
            }

            fn calculate_total(data_model: &$model) -> f64 {
                data_model.subtotal.unwrap_or(0.0) + data_model.total_tax
            }
        }

        // Macros are the Rust metaprogramming feature that creates code in compile-time.
        impl $name for $model {}
    };
}

calculate_total!(CalculationsSale, Sale);
calculate_total!(CalculationsPurchase, Purchase);
calculate_total!(CalculationsBudget, Budget);

fn main() {
    let shoes = Product {
        name: "shoes".to_string(),
        cost: 45.65,
    };
    let product_detail = ProductDetail {
        product: shoes,
        discount: None,
        tax: 21.0,
        price: Some(120.0),
    };
    let mut sale = Sale {
        product_details: vec![product_detail],
        date: "10/12/2020".to_string(),
        subtotal: None,
        total_tax: 21.0,
        total: None,
    };
    sale.subtotal = Some(Sale::calculate_subtotal(&sale));
    sale.total = Some(Sale::calculate_total(&sale));
    println!("This is my sale: {:#?}", sale);
}
