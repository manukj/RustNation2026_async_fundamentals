use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create the channel
    //      Split the (transmitter, receiver)
    //      Buffer size of 32 messages
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a task to send messages
    let sender_task = tokio::spawn(async move {
        for i in 0..10 {
            let message = format!("Message {}", i);
            if let Err(e) = tx.send(message).await {
                eprintln!("Failed to send message: {}", e);
                return;
            }
        }
    });

    // Loop to receive messages
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}