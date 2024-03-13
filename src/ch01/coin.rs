fn main() {
    // 거스름돈
    let price = 3950;

    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..10 {
                let total = 500 * i500 + 100 * i100 + 50 * i50;
                if total == price {
                    println!("500원 * {} + 100원 * {} + 50원 * {}", i500, i100, i50);
                }
            }
        }
    }
}
