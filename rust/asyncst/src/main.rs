use async_stream::stream;
use tokio::pin;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let st = stream! {
        for i in 0..100 {
            yield i;
        }
    };

    pin!(st);

    while let Some(i) = st.next().await {
        println!("{i}");
    }
}
