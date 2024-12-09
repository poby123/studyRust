pub mod basic1 {
    fn sum_array(arr: &[i32]) -> i32 {
        let mut result = 0;
        for i in arr {
            result += i;
        }
        result
    }

    fn swap(a: &mut i32, b: &mut i32) {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }

    pub fn test() {
        {
            println!("\n[ARRAY TEST]");
            let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            println!("arr: {:?}", arr);
            println!("arr[0]: {:?}", arr[0]);
            println!("sum of arr: {}", sum_array(&arr));

            let odd_arr: Vec<_> = arr.into_iter().filter(|x| x % 2 == 1).collect();
            println!("odd_arr: {:?}", odd_arr);
            assert_eq!(odd_arr.iter().all(|&x| x % 2 == 1), true);

            let even_arr: Vec<_> = arr.into_iter().filter(|x| x % 2 == 0).collect();
            println!("even_arr: {:?}", even_arr);
            assert_eq!(even_arr.iter().all(|&x| x % 2 == 0), true);

            println!("arr: {:?}", arr);
        }

        {
            println!("\n[SWAP TEST]");
            let mut a = 1;
            let mut b = 2;
            println!("before swap: a = {a}, b = {b}");
            swap(&mut a, &mut b);
            println!("before swap: a = {a}, b = {b}");
        }

        {
            println!("\n[STRING TEST]");
            let s = String::from("Hello?! 12345");
            println!("s =  {s}");

            if let Some(find_index) = s.find("123") {
                println!("find '123' result: {find_index}");
            }

            if let Some(find_index) = s.find("hi") {
                println!("find 'hi' result: {find_index}");
            } else {
                println!("find 'hi' is not found");
            }

            let arr = s.split(" ");

            for str in arr {
                println!("item: {str}");
            }
        }
    }
}
