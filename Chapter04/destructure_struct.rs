// destructure_struct.rs

enum Food {
    Pizza,
    Salad
}

enum PaymentMode {
    Bitcoin,
    Debit,
    Credit
}

struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode
}

fn main() {
    let food_order = Order { count: 2,
                             item: Food::Salad,
                             payment: PaymentMode::Credit};

    // let can pattern match to inner fields of types
    let Order { count, item, .. } = food_order;
}
