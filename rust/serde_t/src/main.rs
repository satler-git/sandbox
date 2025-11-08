use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    author: String,
    to: String,
    count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mes = Message {
        author: "alpha".into(),
        to: "beta".into(),
        count: 100,
    };

    dbg!(&mes);
    println!("{}", serde_json::to_string(&mes)?);

    Ok(())
}
