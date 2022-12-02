use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use rust_subscriber::configuration::{get_configuration, DatabaseSettings};


use std::net::TcpListener;
use rust_subscriber::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}



//spins up instance of application and returns local host address
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configration().expect("Failed to get configuration");
    configuration.database.database_name = Uuid ::new_v4().to_string();
    let connection_pool = coonfigure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]

async fn subscribe_returns_a_200_for_valid_form_data(){
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //act
    let body = "name=le%20guin&email=franz_m_char%40gmail.com";

    let response = client
       .post(&format!("{}/subscribtions", &app_address))
       .header("Content-Type", "application/x-www-form-urlencoded")
       .body(body)
       .send()
       .await.expect("failed to execute request");

       //assert
       assert_eq!(200, response.status().as_u16());
  
}
#[tokio::test]

async fn subscribe_returns_a_400_when_data_is_missing() {
    //arange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![(
        "name=le%20guin", "missing the email"),
        ("email=franz_m_char%40gmail.com", "missing the email"),
        ("", "missing both email and name")];

        for (invalid_body, error_message) in test_cases {
            //act
            let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Contetnt-Type","application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to esecute request.");
        }
    
        
}