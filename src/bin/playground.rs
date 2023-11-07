use tokio_stream::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut stream = tokio_stream::iter([
        async { 1 },
    ]);


    while let Some(x) = stream.next().await {
        println!("{}", x.await);
    }
}

