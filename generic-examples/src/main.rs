mod match_example {
    // This is an example function that simply takes a parameter as string and return
    // the expected type for "This works" or an error for everything else.
    fn this_an_example(some_parameter: &str) -> Result<i32, String> {
        match some_parameter {
            "This works" => Ok(1),
            &_ => Err("An Error".to_string()),
        }
    }

    fn main() {
        println!("Result: {:#?}", this_an_example("This works"));
        println!("Oh no!: {:#?}", this_an_example("Error"));
    }
}

// This is supposed to return an i32 from a str, but it won't work becuase of the return type.
fn convert_to_integer(number: &str) -> i32 {
    match number.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

fn main() {
    convert_to_integer("452");
}
