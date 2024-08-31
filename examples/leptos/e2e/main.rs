use fantoccini::Locator;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        println!("Stopping Leptos app...");
        self.0.kill().unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Running end-to-end tests with doco...");

    println!("Building Leptos app...");
    if !Command::new("cargo")
        .arg("leptos")
        .arg("build")
        .spawn()
        .expect("failed to start building Leptos")
        .wait()
        .unwrap()
        .success()
    {
        panic!("failed to build Leptos");
    }

    println!("Starting Leptos app...");
    let mut leptos = Command::new("cargo")
        .arg("leptos")
        .arg("watch")
        .spawn()
        .expect("failed to start Leptos");
    let leptos = ChildGuard(leptos);

    println!("Waiting for Leptos app to start...");
    for _ in 0..10 {
        if reqwest::Client::new()
            .get("http://localhost:3000/")
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
    client.goto("http://localhost:3000/").await.unwrap();

    let title = client
        .find(Locator::XPath("/html/body/main/h1"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!("Welcome to Leptos!", title);
}
