fn main() {
    let moon = 384400.0;
    let car_kph = 80.0;
    let ktx_kph = 300.0;

    println!("달까지 자동차로 {}일", moon / car_kph / 24.0);
    println!("달까지 KTX로 {}일", moon / ktx_kph / 24.0);
}
