use doco::{Client, Doco, Result, Server, Service, WaitFor};

#[doco::test]
async fn reads_from_database(client: Client) -> Result<()> {
    client.goto("/").await?;

    let body = client.source().await?;

    assert!(body.contains("hello world from pg"));

    Ok(())
}

#[doco::main]
async fn main() -> Doco {
    let server = Server::builder()
        .image("doco")
        .tag("axum-postgres")
        .port(3000)
        .build();

    let postgres = Service::builder()
        .image("postgres")
        .tag("latest")
        .env("POSTGRES_PASSWORD", "password")
        .env("POSTGRES_USER", "postgres")
        .env("POSTGRES_PASSWORD", "postgres")
        .wait(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ))
        .build();

    Doco::builder().server(server).service(postgres).build()
}
