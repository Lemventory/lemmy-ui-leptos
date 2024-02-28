use lemmy_client::lemmy_api_common::site::GetSiteResponse;
use leptos::*;
<<<<<<< HEAD
use leptos_query::{use_query, QueryOptions, QueryResult, RefetchFn, ResourceOption};
=======
use leptos_query::{create_query, QueryOptions, QueryScope, ResourceOption};
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)

#[server(GetSiteResource, "/serverfn", "GetJson")]
async fn get_site() -> Result<GetSiteResponse, ServerFnError> {
  use actix_session::Session;
  use actix_web::web;
  use lemmy_client::{LemmyClient, LemmyRequest};
  use leptos_actix::extract;

  let session = extract::<Session>().await?;
  let client = extract::<web::Data<LemmyClient>>().await?;

  let jwt = session.get::<String>("jwt")?;

<<<<<<< HEAD
  client
    .get_site(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(Into::into)
}

pub fn use_site_state() -> QueryResult<Result<GetSiteResponse, ServerFnError>, impl RefetchFn> {
  use_query(
    || (),
    |_| async move { get_site().await },
    QueryOptions {
      resource_option: ResourceOption::Blocking,
=======
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
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
      ..QueryOptions::default()
    },
  )
}
