use once_cell::sync::Lazy;
use reqwest::header::CONTENT_TYPE;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::io::{sink, stdout};
use std::net::TcpListener;
use zero2prod::configuration::{get_config, ApplicationSettings, DbSettings};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
pub struct TestApp {
    pg_pool: PgPool,
    address: String,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("z2p_healthcheck".into(), "debug".into(), stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("z2p_healthcheck".into(), "debug".into(), sink);
        init_subscriber(subscriber);
    }
});

/// spawns app at random port
/// reads configuration file
/// overwrites database name (from config file) to random uuid
/// calls configure_db (which creates a new database and runs the migrations in .migrations on it)
/// receives a connection pool for database connection
/// creates a new server instance with the connection pool
/// spawns the server and returns a TestApp with server address and the connection pool
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    //let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_config().expect("Failed to read config"); //
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    // let connection_string = configuration.database.connection_string();
    let connection_pool = configure_db(&configuration.database).await;

    let server =
        zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        pg_pool: connection_pool,
    }
}

/// Creates a database from config
/// runs migrations defines in `./migrations`
/// and returns a connection pool for the new db
async fn configure_db(config: &DbSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect");
    connection
        .execute(
            format!(
                r#"
            CREATE DATABASE "{}";
            "#,
                &config.database_name
            )
            .as_str(),
        )
        .await
        .expect(format!("Failed to create db {}", &config.database_name).as_str());

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect(format!("Cannot connect to {}", &config.connection_string()).as_str());

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate");
    connection_pool
}
#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let url = format!("{}/health_check", spawn_app().await.address);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    //Act
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(response.status().as_u16(), 200);
    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&app.pg_pool)
        .await
        .expect("could not fetch from subscriptions");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_400_for_valid_form_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let bad_requests = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    //Act
    for (bad_request, error_message) in bad_requests {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
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
