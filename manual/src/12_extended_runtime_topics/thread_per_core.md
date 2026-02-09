# Thread-per-core patterns

Tokio, by default, spawns one thread per core --- and spawns more threads for blocking tasks. This is a common pattern - and quite familiar to users of other languages and runtimes.

However, it's really easy get yourself into trouble with futures having to be `Sync`+`Send`. Tokio may move your future to another thread at any time; if you future isn't `Sync`+`Send` then your code won't compile. This is a common source of confusion for new users of async Rust.

One solution is to use other runtimes that explicitly separate async runtimes from thread pools. `Glommio` is an example.

You can also do this in Tokio.

```rust
fn start_thread(
    runtime_number: i32, 
    oneshot_reply: tokio::sync::oneshot::Sender<tokio::sync::mpsc::Sender<i32>>
) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(5);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    oneshot_reply.send(tx).unwrap();
    rt.block_on(async {
        if let Some(value) = rx.recv().await {
            println!("Received value: {} (runtime #{})", value, runtime_number);
        }
    });
}

fn main() {
    // Store the thread handles so we can wait for them at the end
    let mut handles = Vec::new();
    let mut thread_callers = Vec::new();

    for i in 0..4 {
        // Make a channel for the thread to reply with its own channel sender
        let (tx, rx) = tokio::sync::oneshot::channel();
        handles.push(std::thread::spawn(move || start_thread(i, tx)));

        // Receive the thread's channel sender and store it for later use
        let thread_caller = rx.blocking_recv().unwrap();
        thread_callers.push(thread_caller);
    }

    // Send a value to each thread's channel sender
    for (i, thread_caller) in thread_callers.into_iter().enumerate() {
        thread_caller.blocking_send(i as i32).unwrap();
    }

    // Wait for thread termination
    for handle in handles {
        handle.join().unwrap();
    }
}
```

> You could also use an MPMC channel such as `flume` or `crossbeam` to automatically fan-out between runtimes.

So why would you put up with all this ceremony?

1. Now you control the thread pool, making for a cleaner integration with your system - that may or may not be async.
2. For workloads that are genuinely isolated from each other, this can lead to higher performance. Several webservers use this tactic.

And yet... we're still not quite `Sync`+`Send` free. 