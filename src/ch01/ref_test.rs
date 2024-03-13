fn swap1(mut a: usize, mut b: usize) {
    let temp = a;
    a = b;
    b = temp;
}

fn swap2(a: &mut usize, b: &mut usize) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 10;
    let mut b = 20;
    swap1(a, b);
    println!("a: {}, b: {}", a, b);

    a = 10;
    b = 20;
    swap2(&mut a, &mut b);
    println!("a: {}, b: {}", a, b);
}
