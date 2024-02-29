use crate::utils::get_client_and_session::get_client_and_session;
use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryScope, ResourceOption};

#[server(GetSiteResource, "/serverfn", input = GetUrl)]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use lemmy_client::LemmyRequest;

  let (client, session) = get_client_and_session().await?;
  let jwt = session.get::<String>("jwt")?;

  // TODO: Update once I figure out how to get the custom error types working
  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_site_state() -> QueryScope<(), Result<GetSiteResponse, ServerFnError>> {
  create_query(
    |_| async move { get_site().await },
    QueryOptions {
      resource_option: Some(ResourceOption::Blocking),
      ..QueryOptions::default()
    },
  )
}
