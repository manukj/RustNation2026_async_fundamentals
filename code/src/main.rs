use futures::future::join_all;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10).map(|n| tokio::spawn(hello(n))).collect();
    for task in tasks {
        task.await.unwrap();
    }
}

async fn hello(n: u8) {
    println!("Hello {n}");
}