use crate::queries::communities_list_query::use_communities_scope;
use lemmy_client::lemmy_api_common::{
  community::*,
  lemmy_db_schema::{ListingType, SortType},
};
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::A;

#[component]
pub fn Trending() -> impl IntoView {
  // let i18n = use_i18n();
  let QueryResult { data, .. } = use_communities_scope().use_query(|| ListCommunities {
    type_: Some(ListingType::Local),
    sort: Some(SortType::Hot),
    limit: Some(5),
    ..Default::default()
  });

  view! {
    <Transition fallback=|| { "Loading..." }>
      <div class="card w-full bg-base-300 text-base-content mb-3">
        <figure>
          <div class="card-body bg-info">
            <h2 class="card-title text-info-content">"Trending Communities"</h2>
          </div>
        </figure>
        <div>
          <p>
            <For
              each=move || {
                  with!(
                      | data | data.as_ref().and_then(| data | data.as_ref().ok()).map_or_else(||
                      Vec::new(), | data | data.communities.clone())
                  )
              }

              key=|cv| cv.community.id
              children=|cv| {
                  view! {
                    <A
                      class="text-l font-bold link link-accent whitespace-nowrap"
                      href=format!("/c/{}", cv.community.name)
                    >

                      {cv.community.name}
                    </A>
                    &nbsp;
                  }
              }
            />

          </p>
          <A class="btn" href="/create_community">
            "Create a community"
          </A>
          <A class="btn" href="/communities">
            "Explore communities"
          </A>
        </div>
      </div>
    </Transition>
  }
}
