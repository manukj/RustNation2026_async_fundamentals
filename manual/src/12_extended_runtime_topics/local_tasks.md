# Local tasks (LocalSet and spawn_local)

Tokio has "local" replacements for `spawn` and `JoinHandle` called `spawn_local` and `LocalSet`. These allow you to run non-`Send` futures on a single thread. It guarantees that the future will always run on the same thread, so it doesn't require `Send` or `Sync`.

Here's an example of using `spawn_local`:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let local_set = task::LocalSet::new();
    local_set.run_until(async {
        task::spawn_local(async {
            println!("This is a local task!");
        }).await.unwrap();
    }).await;
}
```

You can spawn multiple local tasks on the same `LocalSet`, and they will all run on the same thread. This can be useful for certain types of workloads that require thread-local state or that need to interact with non-`Send` APIs. Here's a `Localset` example that demonstrates this:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let local_set = task::LocalSet::new();
    local_set.run_until(async {
        task::spawn_local(async {
            println!("This is local task 1!");
        }).await.unwrap();
        task::spawn_local(async {
            println!("This is local task 2!");
        }).await.unwrap();
    }).await;
}
```

If we combine that with thread-local runtimes, we now:

1. Control the thread pool, making for a cleaner integration with your system - that may or may not be async.
2. For workloads that are genuinely isolated from each other, this can lead to higher performance. Several webservers use this tactic.
3. Can use non-`Send` APIs and thread-local state without worrying about `Sync`+`Send` bounds.
4. Can use `Rc` and `RefCell` instead of `Arc` and `Mutex` for shared state, which can be more ergonomic in some cases.
5. Can ensure that certain tasks always run on the same thread, which can be important for certain types of workloads (e.g. GUI applications, or workloads that require thread-local state).
