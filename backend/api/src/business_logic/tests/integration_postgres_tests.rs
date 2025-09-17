mod integration_postgres_test {
    use business_logic::services::{
        auth_service::AuthService,
        search_service::SearchService,
        route_service::RouteService
    };
    use business_logic::services_traits::{Authorizer, TrackInfoSearcher, CarSearcher, RouteGetter};
    use data_access::repositories::postgres::{PgUserRepo, PgCarRepo, PgTrackInfoRepo, PgSnapRepo, PG_URL};
    use models::bulder::UserBuilder;
    use models::{Document, Role};

    #[tokio::test]
    async fn integration_test_psql_auth() {
        let service = AuthService::from(Box::new(PgUserRepo::from(&PG_URL).await.unwrap()));
        let email = "uewmleii@icloud.com".to_string();
        let pswd = "Krd!G0RW&".to_string();

        let res = service
            .auth(
                &email,
                &pswd,
            )
            .await;

        println!("{:#?}", res);
        let cuser = UserBuilder::new()
            .name("Парамон".to_string())
            .surname("Артемьева".to_string())
            .lastname(None)
            .email(email)
            .role(Role::user)
            .is_verified(true)
            .passport(Some(Document {
                serial: "9217".to_string(),
                number: "389203".to_string()
            }))
            .build();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), cuser);
    }

    #[tokio::test]
    async fn integration_test_psql_search_track_info_by_date() {
        let service = SearchService::from(
            Box::new(PgCarRepo::from(&PG_URL).await.unwrap()),
            Box::new(PgTrackInfoRepo::from(&PG_URL).await.unwrap())
        );

        let res = service
            .search_track_info_by_date(&"01.01.2025".to_string())
            .await;

        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), []);
    }

    #[tokio::test]
    async fn integration_test_psql_search_car_by_gos_num_mask() {
        let service = SearchService::from(
            Box::new(PgCarRepo::from(&PG_URL).await.unwrap()),
            Box::new(PgTrackInfoRepo::from(&PG_URL).await.unwrap())
        );

        let res = service
            .search_cars_by_gos_num_mask(&"А7**М*77".to_string())
            .await;

        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), []);
    }

    #[tokio::test]
    async fn integration_test_psql_route_get() {
        let service = RouteService::from(
            Box::new(PgUserRepo::from(&PG_URL).await.unwrap()),
            Box::new(PgSnapRepo::from(&PG_URL).await.unwrap()),
            Box::new(PgTrackInfoRepo::from(&PG_URL).await.unwrap())
        );

        let res = service
            .get_car_route(
                &"О987МС36".to_string(),
                &"xrldnhuaz@protonmail.com".to_string(),
                &"01.01.2025".to_string(),
            )
            .await;

        println!("{:#?}", res);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), None);
    }
}
