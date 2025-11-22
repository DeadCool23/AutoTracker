mod end_to_end_tests {
    use std::env;
    use api::handlers::response_status_code::ResponseStatusCode;
    use api::handlers::v1::auth_services::auth_service::{AuthRequest, AuthResponse};
    use api::handlers::v1::route_get_service::{RouteRequest, RouteResponse};
    use api::handlers::v1::search_services::{
        car_search_services::CarSearcherResponse, search_requests::SearchByPassportRequest,
    };
    use api::router;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use serde_json;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_route_endpoint() {
        let app: Router = router::init();
        // ==== AUTH ====

        let pswd = env::var("TEST_PASSWORD")
            .expect("TEST_PASSWORD environment variable not set");
        let request = AuthRequest {
            email: "uewmleii@icloud.com".to_string(),
            pswd: pswd,
        };

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/user/auth")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let auth_response: AuthResponse = serde_json::from_slice(&bytes).unwrap();

        println!("Auth response: {:#?}", auth_response);
        assert_eq!(auth_response.status.code, ResponseStatusCode::OK as isize);
        assert!(auth_response.user.is_some());
        let user = &auth_response.user.unwrap();
        assert!(user.passport.is_some());

        // ==== CAR SEARCH ====

        let passport = user.passport.clone().unwrap();

        let request = SearchByPassportRequest { passport };

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/car/search/by-passport")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let car_search_response: CarSearcherResponse = serde_json::from_slice(&bytes).unwrap();

        println!("Car search response: {:#?}", car_search_response);
        assert_eq!(
            car_search_response.status.code,
            ResponseStatusCode::OK as isize
        );
        assert_eq!(car_search_response.cars.len(), 2);

        // ==== Route Get ====

        let request = RouteRequest {
            gos_num: car_search_response.cars[0].gos_num.clone(),
            user_login: user.email.clone(),
            date: "01.01.2025".to_string(),
        };

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/car/route")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let route_response: RouteResponse = serde_json::from_slice(&bytes).unwrap();

        println!("Route get response: {:#?}", route_response);
        assert_eq!(route_response.status.code, ResponseStatusCode::OK as isize);
        assert!(route_response.route.is_none());
    }
}
