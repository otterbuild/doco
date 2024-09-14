#[doco_derive::main]
async fn main() {}

#[doco_derive::test]
async fn first_test_case() {
    async {
        assert!("🦕 doco".contains("🦕"));
    }
    .await
}

#[doco_derive::test]
async fn second_test_case() {
    async {
        assert!("🦕 doco".contains("doco"));
    }
    .await
}
