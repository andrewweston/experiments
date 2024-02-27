use anyhow::Result;
use serde::de::{Deserializer, Error};
use serde::Deserialize;

fn main() {
    let my_req = serde_json::json!({
        "a": "a",
        "b": "b"
    });

    let de: MyRequest = serde_json::from_value(my_req).unwrap();
    println!("{:?}", de);
}

#[derive(Deserialize, Debug)]
#[serde(remote = "Self")]
struct MyRequest {
    a: String,
    b: String,
}

impl<'de> Deserialize<'de> for MyRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.a == "b" && this.b.is_empty() {
            return Err(D::Error::custom("bar should be set when foo equals '5'"));
        }

        Ok(this)
    }
}
