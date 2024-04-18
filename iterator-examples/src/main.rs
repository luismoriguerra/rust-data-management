fn main() {
    //An iterator is a trait that allows us to perform the same task over a collection of elements in turn. Iterators are lazy.
    let v = vec![1, 2, 3, 4, 5];

    let v_iter = v.iter();

    println!("{:?}", v_iter);

    println!("##############");

    let v = vec![1, 2, 3, 4, 5];

    //By declaring v_iter as mut, you allow the .next() method to modify its internal state
    //(specifically, to update its position within the vector v).
    let mut v_iter = v.iter();

    println!("{:?}", v_iter.next());

    println!("##############");

    let v = vec![1, 2, 3, 4, 5];

    for x in v.iter() {
        println!("{:?}", x);
    }

    println!("##############");

    struct Fibonacci {
        current: u32,
        next: u32,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {
                current: 0,
                next: 0,
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current == 0 && self.next == 0 {
                self.next = 1;
                return Some(0);
            }

            let new = self.current + self.next;

            self.current = self.next;
            self.next = new;

            Some(self.current)
        }
    }

    let mut series = Fibonacci::new();

    for _ in 0..10 {
        println!("{:?}", series.next());
    }
}
