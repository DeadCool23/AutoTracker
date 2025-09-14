mod integration_clickhouse_test {
    use business_logic::services::auth_service::AuthService;
    use business_logic::services_traits::Authorizer;
    use data_access::repositories::clickhouse::{ClickHouseUserRepo, CLICKHOUSE_URL};
    // use models::Document;

    #[tokio::test]
    async fn integration_test_clickhouse_auth_success() {
        let service = AuthService::from(Box::new(ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap()));

        let res = service
            .auth(
                &"uewmleii@icloud.com".to_string(),
                &"Krd!G0RW&".to_string(),
            )
            .await;

        println!("{:#?}", res);
        assert!(res.is_ok());
    }
}