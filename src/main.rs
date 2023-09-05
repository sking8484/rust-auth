use aws_sdk_cognitoidentityprovider;
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
    extract::State
};

#[derive(Debug)]
struct AppState {

    client: aws_sdk_cognitoidentityprovider::Client
}

#[tokio::main]
async fn main() {
    let config = aws_config::from_env()
        .profile_name("TODO")
        .load()
        .await;
    let client = aws_sdk_cognitoidentityprovider::Client::new(&config);
    let app_state = Arc::new(AppState {client});
    let app = Router::new()
        .route("/",get(root))
        .route("/signup", get(sign_up))
        .route("/signin", get(sign_in))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(state): State<Arc<AppState>>) {
    state.client.create_user_pool()
        .pool_name("test_pool")
        .send()
        .await;



}

async fn sign_up(State(state): State<Arc<AppState>>) {
    println!("Signing User Up");
    let result = state.client.sign_up()
        .client_id("TODO")
        .username("junk")
        .password("junkjunkjunk")
        .send()
        .await;

    println!("{:?}", result);
}

async fn sign_in(State(state): State<Arc<AppState>>) {
    println!("Singing User In");
    let result = state.client.initiate_auth()
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", "junk")
        .auth_parameters("PASSWORD", "junkjunkjunk")
        .client_id("TODO")
        .send()
        .await;

    let auth_result = result.expect("AUTH RESULT").to_owned();
    let token = auth_result.authentication_result().expect("STUFF").access_token();
        

    println!("{:?}", token);
}
