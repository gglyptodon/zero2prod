use reqwest::header::CONTENT_TYPE;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_config;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let url = format!("{}/health_check", spawn_app());
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    //Arrange
    let address = spawn_app();
    let configuration = get_config().expect("Failed to read config");
    let connection_string = configuration.database.connection_string();
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Could not connect to db");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    //Act
    let response = client
        .post(format!("{}/subscriptions", &address))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(response.status().as_u16(), 200);
    //let saved = sqlx::query!("SELECT email, name from subscriptions",)
    //    .fetch_one(&mut connection).await
    //    .expect("could not fetch from subscriptions");
    //assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    //assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_400_for_valid_form_data() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let bad_requests = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    //Act
    for (bad_request, error_message) in bad_requests {
        let response = client
            .post(format!("{}/subscriptions", &address))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(bad_request)
            .send()
            .await
            .expect("Failed to execute request");

        //Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "API did not return 400 when payload was {}",
            error_message
        );
    }
}
