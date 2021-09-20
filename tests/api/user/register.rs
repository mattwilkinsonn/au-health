use cynic::{MutationBuilder, Operation, QueryBuilder};
use uuid::Uuid;

use crate::{
    gql::gql_schema::queries::{NewUser, Register, RegisterArguments},
    helpers::TestApp,
};

#[tokio::test]
async fn register_returns_user_with_same_email_on_valid_input() {
    let app = TestApp::new().await;

    let user = NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let query: Operation<Register> = Register::build(&RegisterArguments { user: user.clone() });

    let response = app.send_graphql_request(query).await;

    assert_eq!(response.register.email.clone(), user.email.clone());
}

#[tokio::test]
async fn register_persists_user_in_database_on_valid_input() {
    let app = TestApp::new().await;

    let user = NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let query: Operation<Register> = Register::build(&RegisterArguments { user: user.clone() });

    let response = app.send_graphql_request(query).await;

    let user_id = Uuid::parse_str(&response.register.id.0).expect("unable to parse uuid");

    let db_user = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, created_at, updated_at, role as "role: _"
     FROM user_account
     WHERE id = $1 LIMIT 1;"#,
        user_id
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failure getting user from db");

    assert_eq!(user.email, db_user.email);
}

#[tokio::test]
async fn register_sets_cookie_on_client_on_valid_input() {
    let app = TestApp::new().await;

    let user = NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let query: Operation<Register> = Register::build(&RegisterArguments { user: user.clone() });

    let response = app.send_graphql_request(query).await;

    assert!(app.auth_cookie_present());
}

#[tokio::test]
/// We only allow @student.american.edu and @american.edu domains.
async fn register_fails_on_invalid_email_domain() {
    let app = TestApp::new().await;

    let invalid_user = NewUser {
        email: "mattwilki17@gmail.com".to_string(),
        password: "hunter2".to_string(),
    };

    let query: Operation<Register> = Register::build(&RegisterArguments {
        user: invalid_user.clone(),
    });

    let response = app.send_graphql_request(query).await;

    response
}
