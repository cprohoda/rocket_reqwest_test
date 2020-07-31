use serde::{Serialize, Deserialize};
use futures::executor::block_on;
use tokio::runtime::Runtime;

async fn request(client: reqwest::Client, my_data: MyData) -> Result<(),reqwest::Error> {
    let req = client.post("http://localhost:8000/api").json(&my_data);
    let res = req.send().await?;

    println!("{:?}", res);
    
    let res_text = res.text().await?;

    let res_data: MyData = serde_json::from_str(&res_text).expect("response deserialization failed");
    println!("{:?}", res_data);

    Ok(())
}

fn main() {
    let client = reqwest::Client::new();

    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(request(client, MyData {a: 1i32}));
}

#[derive(Serialize,Deserialize,Debug)]
struct MyData {
    a: i32,
}
