mod cmd;
mod core;

#[tokio::main]
async fn main() {
    let result = cmd::Root::run().await;

    println!("{:?}", result);
}