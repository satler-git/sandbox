use serde_json::json;

#[derive(serde::Deserialize, Debug)]
struct Test {
    a: usize,
}

fn main() {
    let v = json!({
        "a": 500
    });

    println!("{:?}", serde_json::from_value::<Test>(v));
}
