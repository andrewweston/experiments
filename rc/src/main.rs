use std::sync::Arc;

struct MyRequest {
    a: String,
    b: String,
}

fn main() {
    let req = Arc::new(MyRequest {
        a: "a".to_string(),
        b: "b".to_string(),
    });
    println!("{}", req.b);

    // let req_2 = Arc::clone(&req);
    // let a = req_2.a;
    // println!("{}", a);
}
