// fn hof<A, B>(fun: Fn(A) -> B) -> Box<Fn(A) -> B >

fn hof<F>(normal_arg: i32, fun_arg: F) -> i32
where
    F: Fn(i32) -> i32,
{
    fun_arg(normal_arg)
}

fn main() {
    println!("{:?}", hof(5, |x| x + 1));

    println!("#############");

    let mut sum = 0;

    for n in 0..100 {
        if n % 2 != 0 {
            sum += n;
        }
    }

    println!("{:?}", sum);

    println!("#############");

    let sum = (0..100).filter(|x| x % 2 != 0).fold(0, |a, x| a + x);

    println!("{:?}", sum);

    println!("#############");

    let sum = (0..100)
        .map(|n| n * n)
        .filter(|n| n % 2 != 0)
        .fold(0, |a, x| a + x);

    println!("{:?}", sum);

    println!("#############");

    let sum = (0..)
        .map(|n| n * n)
        // With destructuring (&s): You are saying "I want to directly compare the value pointed to by the reference with 1000", so s becomes an i32 inside the closure.
        // Without destructuring (s): If you wrote the closure as |s| s < 1000, since s would be a reference (&i32), you would have to dereference it inside the closure to compare it to an i32, like this: |s| *s < 1000.
        .take_while(|&s| s < 1000)
        .filter(|n| n % 2 != 0)
        .fold(0, |a, x| a + x);

    println!("{:?}", sum);

    println!("###########");
    let a = [1, 2, 3, 4, 5];

    let a_iter = a.iter().skip(2);

    let b: Vec<&i32> = a_iter.collect();

    println!("{:?}", b);

    println!("###########");
    let a = [1, 2, 3, 4, 5];

    let mut a_iter = a.iter();

    // consume the first two elements
    println!("{:?}", a_iter.nth(2));
    println!("{:?}", a_iter.nth(2));

    println!("###########");
    let a = [1, 2, 3, 4, 5];

    let mut a_iter = a.iter().take(2);

    println!("{:?}", a_iter.next());
    println!("{:?}", a_iter.next());
    println!("{:?}", a_iter.next());

    println!("###########");

    let a = [1, 2, 3, 4, 5];

    let b: Vec<i32> = a.iter().map(|x| x + 1).collect();

    println!("{:?}", b);

    println!("###########");

    let a = [1, 2, 3];

    let mut iter = a.iter().cycle();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next()); // and so on...

    println!("###########");
    let a = [1, 2, 3];
    let b = [4, 5];

    let mut iter = a.iter().chain(b.iter());

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    println!("###########");
    let a = ["apple", "banana", "pear"];

    let mut iter = a.iter().enumerate();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    println!("###########");
    let a = ["apple", "banana", "pear"];
    let b = ["one", "two"];

    let mut iter = a.iter().zip(b.iter());

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    println!("###########");
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even: Vec<&i32> = a.iter().filter(|&x| x % 2 == 0).collect();

    println!("{:?}", even);

    println!("###########");
    let a = [1, 2, 3, 4, 5];

    let sum = a
        .iter()
        .filter(|&x| x % 2 == 0)
        .inspect(|x| println!("passed thorugh the filter: {}; we'll sum it", x))
        .fold(0, |sum, i| sum + i);

    println!("Result: {}", sum);

    println!("###########");
    let o = Some("one");

    println!("Is it a Some(T)? {}", o.is_some());
    println!("Is it a None? {}", o.is_none());

    println!("###########");
    let a: Option<&str> = Some("one");
    let b: Option<&str> = None;

    println!("Content a: {}", a.unwrap());
    // println!("Content b: {}", b.unwrap()); // error

    println!("###########");
    let a: Option<&str> = Some("one");
    // let b: Option<&str> = None;

    println!("Content a: {}", a.expect("Content a non present"));
    // println!("Content b: {}", b.expect("Content b non present"));

    println!("###########");

    let a: Option<&str> = Some("one");
    let b: Option<&str> = None;

    println!("Content a: {}", a.unwrap_or("Content a non present"));
    println!("Content b: {}", b.unwrap_or("Content b non present"));

    println!("###########");
    let fallback = vec!["one", "two"];

    let a: Option<&str> = Some("one");
    let b: Option<&str> = None;

    println!("Content a: {}", a.unwrap_or_else(|| fallback[0]));
    println!("Content b: {}", b.unwrap_or_else(|| fallback[1]));

    println!("###########");
    let a: Option<&str> = Some("one");
    let b: Option<&str> = None;

    println!("Content a: {:?}", a.ok_or(0));
    println!("Content b: {:?}", b.ok_or(0));

    println!("###########");
    let a: Option<&str> = Some("one");
    let b: Option<&str> = None;

    let alt: Option<&str> = Some("two");

    println!("Content a: {:?}", a.or(alt));
    println!("Content b: {:?}", b.or(alt));

    println!("###########");
    let a: Option<i32> = Some(1);
    let b: Option<i32> = None;

    let add_one = |x| Some(x + 1);
    let ret_none = |_| None;

    println!("1st chain: {:?}", a.and_then(add_one).and_then(add_one));
    println!("2nd chain: {:?}", b.and_then(add_one).and_then(add_one));
    println!("3rd chain: {:?}", a.and_then(ret_none).and_then(add_one));

    println!("###########");
    let a: Option<i32> = Some(2);
    let b: Option<i32> = None;

    let get_one = || Some(1);
    let ret_none = || None;

    println!("1st chain: {:?}", a.or_else(get_one).or_else(get_one));
    println!("2nd chain: {:?}", b.or_else(get_one).or_else(get_one));
    println!("3rd chain: {:?}", b.or_else(ret_none).or_else(get_one));
    println!("4th chain: {:?}", b.or_else(ret_none));
}
