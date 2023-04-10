use actix_web::{http, test, web, App};
use rust_api::controllers::user_controller::{delete_user, get_users, UserIdData};

#[actix_rt::test]
async fn test_get_users() {
    let app = test::init_service(App::new().route("/", web::get().to(get_users))).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let expected_body = r#"{"users":["Alice","Bob","Charlie","David"]}"#;
    assert_eq!(body, expected_body);
}

#[actix_rt::test]
async fn test_post_route() {
    let app = test::init_service(App::new().route("/post", web::post().to(delete_user))).await;

    let test_id = "2";
    let form = UserIdData {
        id: test_id.to_owned(),
    };

    let req = test::TestRequest::post()
        .uri("/post")
        .set_form(&form)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let expected_body = format!("User id {{{}}} deleted.", test_id);
    assert_eq!(body, expected_body);
}
