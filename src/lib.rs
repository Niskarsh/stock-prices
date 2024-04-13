#![allow(non_snake_case)]

pub fn highestPrice(weekNumber: usize, maxStack: &Vec<f32>) -> f32 {
    maxStack[weekNumber-1]
}

pub fn createMaxStack (stockPrices: &Vec<f32>) -> Vec<f32> {
    let mut counter = 0;
    let mut maxStock: Vec<f32> = vec![];
    for price in stockPrices {
        if counter == 0 {
            maxStock.push(*price);
            counter += 1;
            continue;
        }
        if *price > maxStock[counter-1] {
            maxStock.push(*price);
        } else {
            maxStock.push(maxStock[counter-1]);
        }
        counter += 1;
    }
    maxStock
}