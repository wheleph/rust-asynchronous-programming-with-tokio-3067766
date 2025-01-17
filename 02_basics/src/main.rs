use std::{thread, time};

use tokio::time::{sleep, Duration};

fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));
    "Finally done".to_string()
}

async fn async_call(id: i32) -> i32 {
    sleep(Duration::from_secs(1)).await;
    println!("Async Call: ID {id}");
    id
}

#[tokio::main]
async fn main() {
    let blocking_call_handle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = Vec::new();
    for i in 0..10 {
        async_handles.push(tokio::spawn(async_call(i)))
    }

    for ah in async_handles {
        let async_call_result = ah.await.unwrap();
        println!("{async_call_result}");
    }

    let blocking_call_result = blocking_call_handle.await.unwrap();
    println!("{blocking_call_result}");
}
