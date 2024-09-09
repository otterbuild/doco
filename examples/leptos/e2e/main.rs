use std::time::Duration;

use doco::server::Server;
use doco::{Doco, Result, TestCase};
use fantoccini::Locator;
use tokio::time::sleep;

struct Leptos;

impl TestCase for Leptos {
    async fn execute(&self, host: String, port: u16) -> Result<()> {
        println!("Waiting for Leptos app to start...");
        for _ in 0..10 {
            if reqwest::Client::new()
                .get(format!("http://{host}:{port}/"))
                .send()
                .await
                .is_ok()
            {
                break;
            } else {
                sleep(Duration::from_secs(1)).await;
            }
        }

        println!("Connecting to WebDriver...");
        let client = fantoccini::ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver");

        println!("Running end-to-end test...");
        client
            .goto(&format!("http://{host}:{port}/"))
            .await
            .unwrap();

        sleep(Duration::from_secs(60)).await;

        let title = client
            .find(Locator::XPath("/html/body/main/h1"))
            .await?
            .text()
            .await?;

        assert_eq!("Welcome to Leptos!", title);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("Running end-to-end tests with doco...");

    let server = Server::builder()
        .image("doco")
        .tag("leptos")
        .port(8080)
        .build();

    let doco = Doco::builder().server(server).build();

    doco.run(Leptos).await.unwrap();
}
