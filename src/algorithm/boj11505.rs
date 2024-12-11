pub mod boj11505 {
    use std::{
        fmt::Write,
        io::{self, Read},
    };

    struct SegmentTree {
        tree: Vec<i64>,
        arr: Vec<i64>,
    }

    impl SegmentTree {
        fn init(&mut self, node: usize, l: usize, r: usize) -> i64 {
            if l == r {
                self.tree[node] = self.arr[l];
                return self.tree[node];
            }

            let m = (l + r) / 2;
            self.tree[node] = self.init(node << 1, l, m) + self.init(node << 1 | 1, m + 1, r);
            return self.tree[node];
        }

        fn update(&mut self, node: usize, l: usize, r: usize, index: usize, v: i64) {
            if index < l || r < index {
                return;
            }

            self.tree[node] += v;
            if l == r {
                return;
            }

            let m = (l + r) / 2;
            self.update(node << 1, l, m, index, v);
            self.update(node << 1 | 1, m + 1, r, index, v);
        }

        fn query(&mut self, node: usize, l: usize, r: usize, s: usize, e: usize) -> i64 {
            if r < s || e < l {
                return 0;
            }

            if s <= l && r <= e {
                return self.tree[node];
            }

            let m = (l + r) / 2;
            return self.query(node << 1, l, m, s, e) + self.query(node << 1 | 1, m + 1, r, s, e);
        }
    }

    pub fn solve_main() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut st = SegmentTree {
            tree: vec![0; 4 * (n + 1)],
            arr: Vec::with_capacity(n + 1),
        };

        st.arr.push(0);
        for _ in 0..n {
            st.arr.push(iter.next().unwrap().parse().unwrap());
        }

        st.init(1, 1, n);

        let mut result = String::new();
        for _ in 0..(m + q) {
            let query_type: i64 = iter.next().unwrap().parse().unwrap();
            if query_type == 1 {
                let b: usize = iter.next().unwrap().parse().unwrap();
                let c: i64 = iter.next().unwrap().parse().unwrap();
                let v = c - st.arr[b];
                st.arr[b] = c;
                st.update(1, 1, n, b, v);
            } else {
                let b: usize = iter.next().unwrap().parse().unwrap();
                let c: usize = iter.next().unwrap().parse().unwrap();
                let value = st.query(1, 1, n, b, c);
                writeln!(&mut result, "{:?}", value).unwrap();
            }
        }
        println!("{result}");
    }
}
