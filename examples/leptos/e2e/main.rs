use std::thread;

use anyhow::anyhow;
use doco::{Client, Doco, Locator, Result, Server, TestCase, TestRunner};
use tokio::runtime::Builder;

fn has_title(client: Client) -> Result<()> {
    thread::spawn(move || {
        let runtime = Builder::new_current_thread().enable_all().build()?;

        runtime.block_on(async {
            println!("Running end-to-end test...");
            client.goto("/").await?;

            let title = client
                .find(Locator::XPath("/html/body/main/h1"))
                .await?
                .text()
                .await?;

            assert_eq!("Welcome to Leptos!", title);

            Ok(())
        })
    })
    .join()
    .map_err(|_| anyhow!("failed to run test in isolated thread"))?
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

    let test_runner = TestRunner::init(doco).await.unwrap();
    let test_case = TestCase::new(has_title);

    test_runner.run(test_case).await.unwrap();
}
