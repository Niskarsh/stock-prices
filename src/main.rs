
use stock_prices::{createMaxStack, highestPrice};
fn main() {
    let weekly_price = vec![12.0, 34.0, 56.0, 34.0 , 11.0, 1.0];
    let maxStack = createMaxStack(&weekly_price);
    let maxPrice = highestPrice(3, &maxStack);
    println!("Max Stack: {:?} \nMax on Day 3 is: {}", maxStack, maxPrice);
}
