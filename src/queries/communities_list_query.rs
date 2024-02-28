use lemmy_client::lemmy_api_common::community::{
  ListCommunities as ListCommunitiesBody,
  ListCommunitiesResponse,
};
use leptos::{server_fn::codec::GetUrl, *};
use leptos_query::{create_query, QueryOptions, QueryScope};
use std::time::Duration;

#[server(prefix = "/serverfn", input = GetUrl)]
async fn list_communities(
  body: ListCommunitiesBody,
) -> Result<ListCommunitiesResponse, ServerFnError> {
  use actix_session::Session;
  use actix_web::web;
  use lemmy_client::{LemmyClient, LemmyRequest};
  use leptos_actix::extract;

  let session = extract::<Session>().await?;
  let client = extract::<web::Data<LemmyClient>>().await?;

  let jwt = session.get::<String>("jwt")?;

  // TODO: Update once I figure out how to get the custom error types working
  client
    .list_communities(LemmyRequest { body, jwt })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub fn use_communities_scope(
) -> QueryScope<ListCommunitiesBody, Result<ListCommunitiesResponse, ServerFnError>> {
  create_query(
    list_communities,
    QueryOptions {
      stale_time: Some(Duration::from_secs(90)),
      ..Default::default()
    },
  )
}
