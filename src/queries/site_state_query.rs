use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_query::{create_query, QueryOptions, QueryScope, ResourceOption};

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use actix_session::Session;
  use actix_web::web;
  use lemmy_client::{LemmyClient, LemmyRequest};
  use leptos_actix::extract;

  let session = extract::<Session>().await?;
  let client = extract::<web::Data<LemmyClient>>().await?;

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
