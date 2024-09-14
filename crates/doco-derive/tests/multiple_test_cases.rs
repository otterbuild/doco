doco_derive::init!();

#[doco_derive::test]
async fn first_test_case() {
    async {
        println!("Running the first test case");
        assert!("🦕 doco".contains("🦕"));
    }
    .await
}

#[doco_derive::test]
async fn second_test_case() {
    async {
        println!("Running the second test case");
        assert!("🦕 doco".contains("doco"));
    }
    .await
}
