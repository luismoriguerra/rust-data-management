use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Write;
use std::{fs::File, io::Error};

mod readfiles {
    use std::process::Command;

    fn main_exmaple() -> std::io::Result<()> {
        let process = Command::new("ls")
            .current_dir("./")
            .spawn()
            .expect("failed to execute process");

        let _ = process.wait_with_output()?;

        Ok(())
    }
}

mod reading_csv_json {
    use csv::ReaderBuilder;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Record {
        #[serde(deserialize_with = "csv::invalid_option")]
        year: Option<u16>,
        who: String,
        event: String,
    }

    fn main() -> Result<(), csv::Error> {
        let csv = r#"year,who,event
1492,Christopher Columbus,landed in the Americas
1  9 6 9,Neil Armstrong,landed on the Moon
"#;

        let mut reader = ReaderBuilder::new()
            .flexible(true)
            .from_reader(csv.as_bytes());

        for record in reader.deserialize() {
            let record: Record = record?;
            match record.year {
                Some(y) => println!("In {}, {} {}.", y, record.who, record.event),
                None => (),
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    name: String,
    age: u32,
    salary: u32,
}

fn main() {
    let json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "salary": 65000
    }
    "#;

    let employee: Employee = serde_json::from_str(json).unwrap();

    println!("{:?}", employee);
    println!("######");
    fn read_and_handle_error() -> Result<(), Error> {
        let employee = Employee {
            name: "John Doe".to_string(),
            age: 43,
            salary: 65000,
        };

        let json_string = serde_json::to_string(&employee).expect("parse error");
        let mut output = File::create("./out.json")?;

        println!("file created successfully");

        write!(output, "{}", json_string)
    }

    read_and_handle_error().expect("error");

    println!("######");

    let value = json!({
        "name": "John Doe using json!",
        "age": 43,
        "salary": 65000
    });

    println!("name: {} and age: {}", value["name"], value["age"]);

    println!("simple {}", value);
    println!("with tostring {}", value.to_string());

    // using dynamic values in json object
    println!("######");

    fn get_area(lenght: u32, hight: u32) -> Value {
        (lenght * hight).into()
    }

    let lenght = 3;
    let hight = 4;
    let shape = json!({
        "type": "rectangle",
        "lenght": lenght,
        "hight": hight,
        "area2": lenght * hight,
        "area": get_area(lenght, hight) // but looks this is not needed to return Value
    });

    println!("shape: {}", shape);
    println!("shape area: {}", shape["area"]);
}
