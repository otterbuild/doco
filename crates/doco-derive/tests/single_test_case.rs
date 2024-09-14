doco_derive::init!();

#[doco_derive::test]
async fn single_test_case() {
    async {
        println!("Running the single test case");
        assert!("🦕 doco".contains("🦕"));
    }
    .await
}
