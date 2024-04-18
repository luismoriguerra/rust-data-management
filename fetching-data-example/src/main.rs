use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
const URL: &str = "https://httpbin.org/html";

fn req_data() -> Result<(), Box<dyn std::error::Error>> {
    let url = URL;

    let resp = get(url)?.text()?;

    println!("Response: {}", resp);

    Ok(())
}

fn main_example() {
    // ways to call
    // req_data().unwrap();
    // if let Err(e) = req_data() {
    //     eprintln!("Error: {}", e);
    // }
    // req_data().expect("Request failed");
    // req_data().unwrap_or_else(|e| eprintln!("Error: {}", e));

    match req_data() {
        Ok(_) => println!("Request successful"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[derive(Debug, Deserialize)]
struct Ip {
    origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = URL;

    let resp = reqwest::get(url).await?.text().await?;

    println!("{}", resp);

    println!("########");
    //  Simple Get call that returns json

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<Ip>()
        .await?;

    println!("{:#?}", resp);

    println!("########");

    // using Post form data

    let john = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    };

    // build pattern
    let client = reqwest::Client::builder().build()?;

    let res = client
        .post("https://httpbin.org/post")
        .form(&john)
        .send()
        .await?;

    let t = res.text().await?;

    println!("{}", t);

    println!("########");

    // inspecting status

    let res = client.get("https://httpbin.org/status/400").send().await?;

    match res.status() {
        reqwest::StatusCode::OK => println!(
            "Success, content leng: {:?}",
            res.headers().get(reqwest::header::CONTENT_LENGTH)
        ),
        reqwest::StatusCode::BAD_REQUEST => println!("Bad request"),
        _ => println!("Some other status"),
    }

    println!("########");

    // custom headers

    let res = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "reqwest")
        .header("x-request-id", "100")
        .send()
        .await?;

    let t = res.text().await?;

    println!("{}", t);

    // end
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     s_init(3)?; // Needed for this playground to work

//     let get_employee = |id| {
//       format!("http://127.0.0.1:3030/employee/{}", id)
//     };

//     let res = reqwest::Client::new()
//         .get(get_employee(1))
//         .send().await?
//         .json::<ApiResponse>().await?;

//     println!("{:#?}", res.data);

//     Ok(())
// }
// #[derive(Debug, Serialize, Deserialize)]
// struct Employee {
//     id: u8,
//     employee_name: String,
//     employee_salary: i32,
//     employee_age: u8,
//     profile_image: String,
// }
// #[derive(Debug, Serialize, Deserialize)]
// struct ApiResponse {
//     data: Vec<Employee>,
//     status: String,
//     message: String,
// }
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     s_init(3)?; // Needed for this playground to work

//     let url = "http://127.0.0.1:3030/employees";

//     let res = reqwest::Client::new()
//         .get(url)
//         .send().await?
//         .json::<ApiResponse>().await?;

//     println!("{:#?}", res.data[4]);
//     Ok(())
// }
