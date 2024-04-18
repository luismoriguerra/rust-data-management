// fn neverending() -> ! {
//     loop{}
//   }

fn main() {
    fn two(x: i32) -> i32 {
        if x != 2 {
            2;
        }
        x
    }

    println!("Function two: {}", two(27));

    let captured_value = String::from("This has been captured from the environment");

    // By an immutable reference &T
    // By a mutable reference &mut T
    // By value T
    let capture = || println!("{}", captured_value);

    capture();

    println!("Captured value: {}", captured_value);

    // The last thing we need to know is that we can
    //force the closure to take ownership of the environment
    // it captures with the move keyword:
    let to_be_owned = vec![1, 2, 3];

    let check_out = move |val| to_be_owned.contains(val);

    assert!(check_out(&2));
    assert!(!check_out(&4));

    // move occurs because `to_be_owned` has type `Vec<i32>`,
    // which does not implement the `Copy` trait
    // move change the ownership and and it's not possible to use to_be_owned anymore
    // it was removed from the scope
    // println!("To be owned: {:?}", to_be_owned);
}
