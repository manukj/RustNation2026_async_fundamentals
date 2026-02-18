use futures::future::join_all;

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10).map(|n| tokio::spawn(hello(n))).collect();
    join_all(tasks).await; // This waits for all tasks to complete.
}

async fn hello(n: u8) {
    println!("Hello {n}");
}