use lemmy_client::lemmy_api_common::post::{GetPost as GetPostBody, GetPostResponse};
use leptos::prelude::{server, server_fn::codec::GetUrl, ServerFnError};

#[server(prefix = "/serverfn", input = GetUrl)]
pub async fn get_post(body: GetPostBody) -> Result<GetPostResponse, ServerFnError> {
  use crate::utils::{get_client_and_session, GetJwt};
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get_jwt()?;

  client
    .get_post(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
