pub mod basic2 {
    use std::collections::{BinaryHeap, HashSet, VecDeque};

    pub fn test() {
        {
            println!("\n[VECTOR TEST]");
            let mut v = Vec::new();
            v.push(3);
            v.push(1);
            v.push(2);
            v.sort();
            println!("v vector: {:?}", v);

            let v2 = Vec::from([3, 4, 5]);
            println!("v2 vector: {:?}", v2);

            let v3 = vec![1, 2, 3, 4, 5];
            println!("v3 vector: {:?}", v3);
            println!("The first element is {:?}", v3[0]);

            let third = v3.get(2);
            match third {
                Some(third) => println!("The third element is {third}"),
                None => println!("There is no third element"),
            }

            if let Some(fourth) = v3.get(3) {
                println!("The fourth element is {fourth}");
            } else {
                println!("There is no fourth element");
            }
        }

        {
            println!("\n[SET TEST]");
            let mut s1 = HashSet::new();
            s1.insert(3);
            s1.insert(3);
            s1.insert(5);
            assert_eq!(s1.len(), 2);

            let s2 = HashSet::from([1, 2, 3, 4, 5]);
            let intersect: Vec<_> = s1.intersection(&s2).collect();
            println!("{:?}", intersect);
        }

        {
            println!("\n[QUEUE TEST]");
            let mut q: VecDeque<_> = VecDeque::new();
            for i in 0..10 {
                q.push_back(i);
            }
            for i in 0..10 {
                q.push_front(i);
            }

            let find_result = q.binary_search(&3);
            if let Ok(index) = find_result {
                println!("{:?}", q[index]);
            }
        }

        {
            println!("\n[PRIORITY QUEUE TEST]");
            let mut pq: BinaryHeap<_> = BinaryHeap::from([-1, 3, 9, -10, 0, 2, 1]);
            while !pq.is_empty() {
                let poped = pq.pop();
                if let Some(cur) = poped {
                    print!("{cur} ");
                }
            }
        }
    }
}
