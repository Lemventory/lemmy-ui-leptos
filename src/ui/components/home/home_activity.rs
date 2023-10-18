// use actix_web::web;
use crate::ui::components::post::post_listings::PostListings;
use lemmy_api_common::{
  lemmy_db_views::structs::PaginationCursor,
  post::{GetPosts, GetPostsResponse},
};
use leptos::*;



#[component]
pub fn HomeActivity() -> impl IntoView {
  let error = create_rw_signal::<Option<String>>(None);

  let next_page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  let page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  let cursor_string = create_rw_signal::<Option<String>>(None);

  let prev_cursor_stack = create_rw_signal::<Vec<Option<PaginationCursor>>>(vec![]);

  let refresh = create_rw_signal(true);

  let posts = create_resource(
    move || cursor_string(),
    move |cursor_string| async move {
      let form = GetPosts {
        type_: None,
        sort: None,
        community_name: None,
        community_id: None,
        page: None,
        limit: None,
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        page_cursor: page_cursor(),
      };

      let result = {
        #[cfg(not(feature = "ssr"))]
        {
          use crate::lemmy_client::*;
          Some((Fetch {}).list_posts(form).await)
        }
        #[cfg(feature = "ssr")]
        {
          use crate::lemmy_client::LemmyClient;
          use actix_web::web;
          use leptos_actix::extract;

          extract(|client: web::Data<awc::Client>| async move { client.list_posts(form).await })
            .await
            .ok()
        }
      };

      match result {
        Some(Ok(o)) => {
          next_page_cursor.set(o.next_page.clone());
          Some(o)
        }
        _ => None,
      }
    },
  );

  let err_msg = " Error loading this post.";

  view! {
    <div class="w-full flex flex-col sm:flex-row flex-grow overflow-hidden">
      <main role="main" class="w-full h-full flex-grow p-3 overflow-auto">
        {move || {
            error
                .get()
                .map(|err| {
                    view! {
                      <div class="alert alert-error">
                        <span>{err}</span>
                      </div>
                    }
                })
        }}

        <div class="join">
          <button class="btn join-item">"Posts"</button>
          <button class="btn join-item">"Comments"</button>
        </div>
        <div class="join">
          <button class="btn join-item">"Subscribed"</button>
          <button class="btn join-item">"Local"</button>
          <button class="btn join-item">"All"</button>
        </div>
        <div class="dropdown">
          <label tabindex="0" class="btn">
            "Sort type"
          </label>
          <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
            <li>
              <span>Item 1</span>
            </li>
            <li>
              <span>Item 2</span>
            </li>
          </ul>
        </div>
        <Suspense fallback=|| {
            view! { "Loading..." }
        }>
          {move || {
              posts
                  .get()
                  .map(|res| match res {
                      None => {
                          view! { <div>{err_msg}</div> }
                      }
                      Some(res) => {
                          view! {
                            <div>
                              <PostListings posts=res.posts.into() error/>
                            </div>
                          }
                      }
                  })
          }}

        </Suspense>
        <button
          class="btn"
          on:click=move |_| {
              let mut p = prev_cursor_stack();
              let s = p.pop().unwrap_or(None);
              prev_cursor_stack.set(p);
              page_cursor.set(s);
              refresh.set(!refresh());
          }
        >

          "Prev"
        </button>
        <button
          class="btn"
          on:click=move |_| {
              let mut p = prev_cursor_stack();
              p.push(page_cursor());
              prev_cursor_stack.set(p);
              page_cursor.set(next_page_cursor());
              refresh.set(!refresh());
          }
        >

          "Next"
        </button>
      </main>
      <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 p-4">
        <div class="sticky top-0 p-4 bg-gray-100 rounded-xl w-full"></div>
        <div class="bg-gray-50 rounded-xl border my-3 w-full">
          <div class="max-w-7xl mx-auto py-8 px-4 sm:px-6 lg:py-12 lg:px-8 lg:flex lg:items-center lg:justify-between"></div>
        </div>
      </div>
    </div>
  }
}
