use anyhow::Result;
use serde::Deserialize;

fn main() {
    let my_req = serde_json::json!({
        "a": "a",
        "b": "b"
    });

    let de: MyRequest = serde_json::from_value(my_req).unwrap();
    println!("{:?}", de);
}

trait Request {
    fn validate(&self) -> Result<()>;
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "Unchecked<MyRequest>")]
struct MyRequest {
    a: String,
    b: String,
}

#[derive(Deserialize, Debug)]
pub struct Unchecked<T>(T);

impl TryFrom<Unchecked<MyRequest>> for MyRequest {
    type Error = String;

    fn try_from(pu: Unchecked<MyRequest>) -> Result<Self, Self::Error> {
        // if pu.0.a == "b" && pu.b.is_empty() {
        //     return Err("bar should be set when foo equals '5'".to_string());
        // }

        Ok(pu.0)
    }
}
