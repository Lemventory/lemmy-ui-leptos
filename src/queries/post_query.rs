use lemmy_client::lemmy_api_common::post::{GetPost as GetPostBody, GetPostResponse};
use leptos::{server, ServerFnError};
use leptos_query::{create_query, QueryOptions, QueryScope};
use std::time::Duration;

#[server(prefix = "serverfn")]
async fn get_post(body: GetPostBody) -> Result<GetPostResponse, ServerFnError> {
  use crate::utils::get_client_and_session::get_client_and_session;
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>("jwt")?;

  client
    .get_post(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_post() -> QueryScope<GetPostBody, Result<GetPostResponse, ServerFnError>> {
  create_query(
    get_post,
    QueryOptions {
      stale_time: Some(Duration::from_secs(300)),
      ..Default::default()
    },
  )
}
