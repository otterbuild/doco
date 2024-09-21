use doco::{Client, Doco, Locator, Result, Server};

#[doco::test]
async fn has_title(client: Client) -> Result<()> {
    client.goto("/").await?;

    let title = client
        .find(Locator::XPath("/html/body/main/h1"))
        .await?
        .text()
        .await?;

    assert_eq!("Welcome to Leptos!", title);

    Ok(())
}

#[doco::main]
async fn main() -> Doco {
    let server = Server::builder()
        .image("doco")
        .tag("leptos")
        .port(8080)
        .build();

    Doco::builder().server(server).build()
}
