use axum::{routing::get, Router};
use clients::solana_rpc_client::SolanaRpcClient;
use controllers::{token_controller::TokenController, user_controller::UserController};
use repositories::user_repository::UserRepository;
use routes::{token_routes::token_routes, user_routes::user_routes};
use solana_sdk::commitment_config::CommitmentConfig;
use sqlx::PgPool;
use shuttle_runtime::SecretStore;

pub mod repositories;
pub mod models;
pub mod clients;
pub mod routes;
pub mod helpers;
pub mod controllers;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let helius_rpc_url = secrets.get("HELIUS_RPC_URL").expect("helius rpc url not found in secrets");
    let keypair_base58_string = secrets.get("KEYPAIR_BASE58_STRING").expect("keypair not found in secrets");

    let solana_rpc_client = SolanaRpcClient::new(
        &helius_rpc_url, 
        CommitmentConfig::confirmed(),
        &keypair_base58_string
    );
    let token_controller = TokenController::new(solana_rpc_client);
    let token_routes = token_routes(token_controller);

    let user_repository = UserRepository::new(pool);
    let user_controller = UserController::new(user_repository);
    let user_routes = user_routes(user_controller);

    let router = Router::new()
        .route("/hello-world", get(hello_world))
        .nest("/api", user_routes)
        .nest("/solana", token_routes);

    Ok(router.into())
}
