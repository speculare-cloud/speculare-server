use crate::errors::AppError;
use crate::errors::AppErrorType;

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, get, web, HttpResponse};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OauthCallback {
    state: String,
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct MeInfo {
    login: String,
    #[serde(rename = "staff?")]
    staff: bool,
}

#[get("/login")]
pub async fn login(session: Session) -> Result<HttpResponse, AppError> {
    let state = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect::<String>();
    let url = format!(
		"https://api.intra.42.fr/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope=public&state={}",
		std::env::var("CLIENT_ID").expect("Missing client_id"),
		std::env::var("REDIRECT_OAUTH_URI").expect("Missing redirect_uri_oauth"),
		state
	);
    session.set("state", state)?;
    Ok(HttpResponse::Found().header("Location", url).finish())
}

#[get("/oauth_callback")]
pub async fn oauth_callback(
    id: Identity,
    session: Session,
    query: web::Query<OauthCallback>,
) -> Result<HttpResponse, AppError> {
    // Retrieve state and check if it's the same one
    let state = session.get::<String>("state")?.unwrap();
    if state != query.state {
        return Err(AppError {
            cause: None,
            message: Some("Invalid state".to_string()),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    let timeout = std::time::Duration::new(5, 0);
    let client = reqwest::ClientBuilder::new()
        .timeout(timeout)
        .connect_timeout(timeout)
        .build()?;

    // Exchange the code with a access_token for the current user
    let form = reqwest::multipart::Form::new()
        .text("grant_type", "authorization_code")
        .text(
            "client_id",
            std::env::var("CLIENT_ID").expect("Missing client_id"),
        )
        .text(
            "client_secret",
            std::env::var("CLIENT_SECRET").expect("Missing client_id"),
        )
        .text("code", query.code.to_string())
        .text(
            "redirect_uri",
            std::env::var("REDIRECT_OAUTH_URI").expect("Missing redirect_uri_oauth"),
        )
        .text("state", state);

    // Retrieve the /oauth/token info
    let res = client
        .post("https://api.intra.42.fr/oauth/token")
        .multipart(form)
        .send()
        .await?
        .json::<AccessToken>()
        .await?;

    let access_token = res.access_token;
    // Retrieve the /users/me info to see if he's authorized to login
    // If not return error
    let res = client
        .get("https://api.intra.42.fr/v2/me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<MeInfo>()
        .await?;

    // Apply the identity
    id.remember(res.login.to_owned());

    // Clear the session as it was only a holder for the state value
    session.clear();

    // Redirect to the vuejs app once the login is finished
    Ok(HttpResponse::Found()
        .header(
            "Location",
            std::env::var("REDIRECT_LEGACY_URI").expect("Missing redirect_uri_legacy"),
        )
        .finish())
}

#[delete("/logout")]
pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    // Redirect to the vuejs app once the logout is finished
    HttpResponse::Found()
        .header(
            "Location",
            std::env::var("REDIRECT_LEGACY_URI").expect("Missing redirect_uri_legacy"),
        )
        .finish()
}
