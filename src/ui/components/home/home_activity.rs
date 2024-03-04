use crate::{
  i18n::*,
  queries::{posts_list_query::use_posts, site_state_query::use_site_state},
  ui::components::{
    common::unpack::Unpack,
    home::{site_summary::SiteSummary, trending::Trending},
    post::post_listings::PostListings,
  },
  utils::derive_query_signal::derive_query_signal,
};
use lemmy_client::lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views::structs::{PaginationCursor, PostView},
  post::GetPosts,
  site::GetSiteResponse,
};
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::*;
use serde::Deserialize;
use web_sys::MouseEvent;

#[component]
pub fn HomeActivity() -> impl IntoView {
  let i18n = use_i18n();
  let QueryResult {
    data: site_response,
    ..
  } = use_site_state().use_query(|| ());
  // let QueryResult {
  //   data: list_posts_response,
  //   ..
  // } = use_posts().use_query(Default::default);

  let query = use_query_map();

  let listing_type = Signal::derive(move || {
    with!(|site_response, query| {
      let site_response = site_response
        .as_ref()
        .map(|site_response| site_response.as_ref().ok())
        .flatten();

      query
        .get("listingType")
        .map(|value| serde_json::from_str(value).ok())
        .flatten()
        .or_else(|| {
          site_response
            .map(|site_response| {
              site_response
                .my_user
                .as_ref()
                .map(|my_user| my_user.local_user_view.local_user.default_listing_type)
            })
            .flatten()
        })
        .or_else(|| {
          site_response
            .map(|site_response| site_response.site_view.local_site.default_post_listing_type)
        })
        .unwrap_or(ListingType::Local)
    })
  });

  let sort_type = Signal::derive(move || {
    with!(|site_response, query| {
      let site_response = site_response
        .as_ref()
        .map(|site_response| site_response.as_ref().ok())
        .flatten();

      query
        .get("sort")
        .map(|value| serde_json::from_str(value).ok())
        .flatten()
        .or_else(|| {
          site_response
            .map(|site_response| {
              site_response
                .my_user
                .as_ref()
                .map(|my_user| my_user.local_user_view.local_user.default_sort_type)
            })
            .flatten()
        })
        .or_else(|| {
          site_response.map(|site_response| site_response.site_view.local_site.default_sort_type)
        })
        .unwrap_or(SortType::Active)
    })
  });

  view! {
    <div class="block">
      <div class="join mr-3 hidden sm:inline-block">
        <button class="btn join-item btn-active">Posts</button>
        <button class="btn join-item btn-disabled">Comments</button>
      </div>
      <div class="join mr-3 hidden sm:inline-block">
        <A
          href=move || with!(| query | query.to_query_string())
          class=move || {
              with!(
                  | listing_type | { let mut class = String::from("btn join-item"); if *
                  listing_type == ListingType::Subscribed { class.push_str(" btn-active") } class }
              )
          }
        >

          Subscribed
        </A>
        <A
          href=move || { with!(| query | query.to_query_string()) }

          class=move || {
              with!(
                  | listing_type | { let mut class = String::from("btn join-item"); if *
                  listing_type == ListingType::Local { class.push_str(" btn-active") } class }
              )
          }
        >

          Local
        </A>
        <A
          href=move || { with!(| query | query.to_query_string()) }

          class=move || {
              with!(
                  | listing_type | { let mut class = String::from("btn join-item"); if *
                  listing_type == ListingType::All { class.push_str(" btn-active") } class }
              )
          }
        >

          All
        </A>
      </div>
      <div class="dropdown hidden sm:inline-block">
        <label tabindex="0" class="btn">
          Sort type
        </label>
        <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <li class=move || {
              with!(
                  | sort_type | if * sort_type == SortType::Active { Some("btn-active") } else {
                  None }
              )
          }>

            <span>{t!(i18n, active)}</span>
          </li>
          <li class=move || {
              with!(
                  | sort_type | if * sort_type == SortType::Hot { Some("btn-active") } else { None }
              )
          }>

            <span>{t!(i18n, hot)}</span>
          </li>
          <li class=move || {
              with!(
                  | sort_type | if * sort_type == SortType::New { Some("btn-active") } else { None }
              )
          }>
            <span>{t!(i18n, new)}</span>
          </li>
        </ul>
      </div>
    </div>
    <main
      role="main"
      class="w-full flex flex-col sm:flex-row flex-grow"
    >// // <Transition fallback=|| {}>
    // //   <div class="flex flex-col">
    // //     <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">
    // //       <Unpack item=posts let:posts>
    // //      <PostListings posts=posts/
    // //      </Unpack>
    // //     </div>
    // //   </div>
    // // </Transition>

    // <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 hidden lg:block">
    // <Trending/>
    // <SiteSummary/>
    // </div>
    </main>
  }
}
