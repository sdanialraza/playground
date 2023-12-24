mod bill;

use bill::Bill;

fn main() {
    let mut bill = Bill::new();

    let bill = bill.change_daily_usage(vec![3, 8, 9, 2, 8, 5, 4, 4]);

    println!("{}", bill.calculate()); // 137.5

    let bill = bill.change_price_range((1..=5, 10.0));

    println!("{}", bill.calculate()) // 235
}
