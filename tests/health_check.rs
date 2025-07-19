use axum_test::TestServer;
use zero2prod::app::app;

#[tokio::test]
async fn health_check_works() {
  let server = TestServer::new(app()).unwrap();

  let response = server.get("/health_check").await;

  response.assert_status_ok();
  response.assert_text("");
}
