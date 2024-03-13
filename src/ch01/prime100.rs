fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

// 100개의 소수를 구한다. primes를 채우기 위해 mutable하게 받는다. [type; length] 형식이다.
fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;

    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    // 0으로 100개의 크기를 갖는 배열을 만든다.
    let mut primes = [0; 100];

    // primes 배열을 mutable하게 넘겨준다.
    get_primes(&mut primes);

    // 배열 타입의 값을 화면에 출력하기 위해 {:?}를 지정한다.
    println!("{:?}", primes);
}
