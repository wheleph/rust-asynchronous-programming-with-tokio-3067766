use tokio::task::JoinHandle;

async fn hello(name: &str) -> String {
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    let handle: JoinHandle<String> = tokio::spawn(hello("Toad"));
    let value = handle.await.unwrap();
    println!("Value: {value}!");
}