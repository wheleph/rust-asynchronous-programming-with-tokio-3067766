use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

async fn person(semaphore: Arc<Semaphore>, name: String) {
    println!("{name} enters the bank");
    teller(semaphore, &name).await;
    println!("{name} leaves the bank");
}

async fn teller(semaphore: Arc<Semaphore>, name: &String) {
    let permit = semaphore.acquire().await.unwrap();

    println!("{name} got a ticket for service");
    sleep(Duration::from_secs(1)).await;
    println!("{name} is in the chair ready to be served");
    sleep(Duration::from_secs(3)).await;
    println!("{name} has been served and walks away");

    drop(permit);
}

#[tokio::main]
async fn main() {
    let semaphore_arc = Arc::new(Semaphore::new(4));

    let mut people_handles = Vec::new();
    for n in 1..=10 {
        let name = format!("Person_{n}");
        let handle = tokio::spawn(person(
            semaphore_arc.clone(),
            name,
        ));
        people_handles.push(handle);
    }

    for h in people_handles {
        let _ = h.await;
    }
}
