use crate::{
  i18n::*,
  queries::{posts_list_query::use_posts, site_state_query::use_site_state},
  ui::components::{
    common::{filter_bar::FilterBar, unpack::Unpack},
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
  // let QueryResult {
  //   data: list_posts_response,
  //   ..
  // } = use_posts().use_query(Default::default);

  view! {
    <FilterBar/>
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      // // <Transition fallback=|| {}>
      // //   <div class="flex flex-col">
      // //     <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">
      // //       <Unpack item=posts let:posts>
      // //      <PostListings posts=posts/
      // //      </Unpack>
      // //     </div>
      // //   </div>
      // // </Transition>

      <Suspense fallback=move || "Loading...">
        <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 hidden lg:block">
          // <Trending/>
          // <SiteSummary/>
        </div>
      </Suspense>
    </main>
  }
}
