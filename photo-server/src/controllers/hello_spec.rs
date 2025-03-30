// use actix_web::{body::to_bytes, test, App};
// use crate::controllers::{hello::hello, hello::echo};


// #[actix_rt::test]
// async fn test_hello_endpoint() {
//     let app = test::init_service(App::new().service(hello)).await;
//     let req = test::TestRequest::get().uri("/").to_request();
//     let resp = test::call_service(&app, req).await;
//     assert!(resp.status().is_success());
//     let body = to_bytes(resp.into_body()).await.unwrap();
//     assert_eq!(body, "Hello world!");
// }

// #[actix_rt::test]
// async fn test_echo_endpoint() {
//     let app = test::init_service(App::new().service(echo)).await;
//     let req = test::TestRequest::post()
//         .uri("/echo")
//         .set_payload("test body")
//         .to_request();
//     let resp = test::call_service(&app, req).await;
//     assert!(resp.status().is_success());
//     let body = to_bytes(resp.into_body()).await.unwrap();
//     assert_eq!(body, "test body");
// }
