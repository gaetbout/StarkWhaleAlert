// This file is a copy of https://github.com/jpopesculian/twitter-v2-rs/blob/master/Cargo.toml
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

use twitter_v2::authorization::{Oauth2Client, Oauth2Token, Scope};
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};

pub struct Oauth2Ctx {
    client: Oauth2Client,
    verifier: Option<PkceCodeVerifier>,
    state: Option<CsrfToken>,
    token: Option<Oauth2Token>,
}

async fn login(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    let mut ctx = ctx.lock().unwrap();
    // create challenge
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    // create authorization url
    let (url, state) = ctx.client.auth_url(
        challenge,
        [
            Scope::TweetRead,
            Scope::TweetWrite,
            Scope::UsersRead,
            Scope::OfflineAccess,
        ],
    );
    // set context for reference in callback
    ctx.verifier = Some(verifier);
    ctx.state = Some(state);
    // redirect user
    Redirect::to(url.to_string().parse().unwrap())
}

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> impl IntoResponse {
    let (client, verifier) = {
        let mut ctx = ctx.lock().unwrap();
        // get previous state from ctx (see login)
        let saved_state = ctx.state.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No previous state found".to_string(),
            )
        })?;
        // check state returned to see if it matches, otherwise throw an error
        if state.secret() != saved_state.secret() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid state returned".to_string(),
            ));
        }
        // get verifier from ctx
        let verifier = ctx.verifier.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No PKCE verifier found".to_string(),
            )
        })?;
        let client = ctx.client.clone();
        (client, verifier)
    };

    // request oauth2 token
    let token = client
        .request_token(code, verifier)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // set context for use with twitter API
    println!("{:?}", token.clone());
    ctx.lock().unwrap().token = Some(token);

    Ok(Redirect::to("/debug_token".parse().unwrap()))
}

async fn debug_token(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    // get oauth token
    let oauth_token = ctx
        .lock()
        .unwrap()
        .token
        .as_ref()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
        .clone();
    // get underlying token
    Ok::<_, (StatusCode, String)>(Json(oauth_token))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "oauth2_callback=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // serve on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // initialize Oauth2Client with ID and Secret and the callback to this server
    let client_id =
        std::env::var("TWITTER_OAUTH2_CLIENT_ID").expect("could not find TWITTER_OAUTH2_CLIENT_ID");
    let client_secret = std::env::var("TWITTER_OAUTH2_CLIENT_SECRET")
        .expect("could not find TWITTER_OAUTH2_CLIENT_SECRET");
    let oauth_ctx = Oauth2Ctx {
        client: Oauth2Client::new(
            client_id,
            client_secret,
            format!("http://{addr}/callback").parse().unwrap(),
        ),
        verifier: None,
        state: None,
        token: None,
    };

    // initialize server
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/debug_token", get(debug_token))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(Mutex::new(oauth_ctx))));

    // run server
    println!("\nOpen http://{}/login in your browser\n", addr);
    tracing::debug!("Serving at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
