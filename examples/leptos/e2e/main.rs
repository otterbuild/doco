use doco::{Client, Doco, Locator, Result, Server, TestCase};

struct Leptos;

impl TestCase for Leptos {
    async fn execute(&self, client: Client) -> Result<()> {
        println!("Running end-to-end test...");
        client.goto("/").await?;

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
