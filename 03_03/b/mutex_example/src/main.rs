use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn watch_channel(remote_arc: Arc<Mutex<i32>>, name: String, channel: i32) {
    let mut remote = remote_arc.lock().await;
    *remote = channel;

    println!("{name} took the remote and set channel: {channel}");
    sleep(Duration::from_secs(2)).await;

    println!("{name} done watching the channel: {channel}");
}

#[tokio::main]
async fn main() {
    let remote = Arc::new(Mutex::new(10));

    let mut handles = Vec::new();

    for (name, channel) in [("Toad", 1), ("Mario", 2), ("Peach", 3), ("Luigi", 4), ("Rosalina", 5)] {
        let handle = tokio::task::spawn(watch_channel(remote.clone(), name.to_string(), channel));
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }
}
