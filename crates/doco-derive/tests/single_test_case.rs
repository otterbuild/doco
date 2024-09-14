#[doco_derive::main]
async fn main() {}

#[doco_derive::test]
async fn single_test_case() {
    async {
        assert!("🦕 doco".contains("🦕"));
    }
    .await
}
