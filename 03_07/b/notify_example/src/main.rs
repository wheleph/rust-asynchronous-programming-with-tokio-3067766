use std::sync::Arc;

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

async fn order_package(package_notify_arc: Arc<Notify>) {
    println!("Looking of package...");
    sleep(Duration::from_secs(2)).await;

    println!("Sending package...");
    sleep(Duration::from_secs(2)).await;

    println!("Package delivered");
    package_notify_arc.notify_one();
}

async fn grab_package(package_notify_arc: Arc<Notify>) {
    package_notify_arc.notified().await;
    println!("Grabbing package");
}

#[tokio::main]
async fn main() {
    let package_notify_arc = Arc::new(Notify::new());

    let order_package_handle = tokio::spawn(order_package(package_notify_arc.clone()));
    let grab_package_handle = tokio::spawn(grab_package(package_notify_arc.clone()));

    let _ = order_package_handle.await;
    let _ = grab_package_handle.await;
}
