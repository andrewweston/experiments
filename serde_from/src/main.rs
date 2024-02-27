use anyhow::Result;
use serde::Deserialize;

// test tag
fn main() {
    let my_req = serde_json::json!({
        "a": "a",
        "b": "b",
        "c": "c"
    });

    let de: MyRequest = serde_json::from_value(my_req).unwrap();
    println!("{:?}", de);
}

pub trait Request {
    fn validate(&self) -> Result<()>;
}

#[derive(Deserialize, Debug)]
pub struct MyRequest {
    pub a: String,
    pub b: String,
    pub c: String,
}

impl Request for MyRequest {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
