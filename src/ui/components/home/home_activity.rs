use crate::{
  i18n::*,
  queries::{posts_list_query::use_posts, site_state_query::use_site_state},
  ui::components::{
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
  let QueryResult {
    data: list_posts_response,
    ..
  } = use_posts().use_query(Default::default);

  let posts = Signal::derive(move || {
    with!(|list_posts_response| list_posts_response
      .as_ref()
      .map(|list_posts_response| list_posts_response.as_ref().ok())
      .flatten()
      .map(|list_posts_response| list_posts_response.posts.clone())
      .unwrap_or_default())
  });

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

  // let from_func = move || {
  //   if let Some(t) = query.get().get("from").cloned() {
  //     if !t.is_empty() {
  //       Some(PaginationCursor(t))
  //     } else {
  //       None
  //     }
  //   } else {
  //     None
  //   }
  // };

  // let ssr_prev = move || query.get().get("prev").cloned();
  // let ssr_limit = move || {
  //   query
  //     .get()
  //     .get("limit")
  //     .cloned()
  //     .unwrap_or("".to_string())
  //     .parse::<i64>()
  //     .ok()
  // };

  // let on_sort_click = move |lt: SortType| {
  //   move |_me: MouseEvent| {
  //     let r = serde_json::to_string::<SortType>(&lt);

  //     match r {
  //       Ok(o) => {
  //         let mut query_params = query.get();
  //         query_params.insert("sort".into(), o);

  //         let navigate = leptos_router::use_navigate();
  //         navigate(&query_params.to_query_string(), Default::default());
  //       }
  //       Err(e) => {
  //         error.set(Some(e.into()));
  //       }
  //     }
  //   }
  // };

  // let csr_infinite_scroll_posts = RwSignal::new(None::<Vec<PostView>>);
  // let csr_paginator = RwSignal::new(None::<PaginationCursor>);

  // let ssr_posts = create_resource(
  //   move || {
  //     (
  //       user.get(),
  //       list_func(),
  //       sort_func(),
  //       from_func(),
  //       ssr_limit(),
  //     )
  //   },
  //   move |(_user, list_type, sort_type, from, limit)| async move {
  //     let form = GetPosts {
  //       type_: list_type,
  //       sort: sort_type,
  //       community_name: None,
  //       community_id: None,
  //       page: None,
  //       limit,
  //       saved_only: None,
  //       disliked_only: None,
  //       liked_only: None,
  //       page_cursor: from,
  //     };

  //     let result = LemmyClient.list_posts(form).await;

  //     match result {
  //       Ok(o) => Some(o),
  //       Err(e) => {
  //         error.set(Some(e));
  //         None
  //       }
  //     }
  //   },
  // );

  // #[cfg(not(feature = "ssr"))]
  // {
  //   let iw = window()
  //     .inner_width()
  //     .ok()
  //     .map(|b| b.as_f64().unwrap_or(0.0))
  //     .unwrap_or(0.0);

  //   let on_resize = move |_| {
  //     let iw = window()
  //       .inner_width()
  //       .ok()
  //       .map(|b| b.as_f64().unwrap_or(0.0))
  //       .unwrap_or(0.0);

  //     let mut query_params = query.get();
  //     if iw >= 2560f64 {
  //       query_params.insert("limit".into(), "30".to_string());
  //     } else if iw >= 1536f64 {
  //       query_params.insert("limit".into(), "20".to_string());
  //     } else {
  //       query_params.remove("limit");
  //     }

  //     if iw >= 640f64 {
  //       csr_infinite_scroll_posts.set(None);
  //       csr_paginator.set(None);
  //     }

  //     let navigate = leptos_router::use_navigate();
  //     navigate(
  //       &format!("{}", query_params.to_query_string()),
  //       Default::default(),
  //     );
  //   };

  //   window_event_listener_untyped("resize", on_resize);

  //   if let Ok(e) = web_sys::Event::new("resize") {
  //     on_resize(e);
  //   }

  //   if iw < 640f64 {
  //     let on_scroll = move |_| {
  //       let h = window()
  //         .inner_height()
  //         .ok()
  //         .map(|b| b.as_f64().unwrap_or(0.0))
  //         .unwrap_or(0.0);
  //       let o = window().page_y_offset().ok().unwrap_or(0.0);
  //       let b = f64::from(document().body().map(|b| b.offset_height()).unwrap_or(1));

  //       let endOfPage = h + o >= b;

  //       if endOfPage {
  //         create_local_resource(
  //           move || (user.get(), list_func(), sort_func()),
  //           move |(_user, list_type, sort_type)| async move {
  //             let form = GetPosts {
  //               type_: list_type,
  //               sort: sort_type,
  //               community_name: None,
  //               community_id: None,
  //               page: None,
  //               limit: None,
  //               saved_only: None,
  //               disliked_only: None,
  //               liked_only: None,
  //               page_cursor: csr_paginator.get(),
  //             };

  //             let result = LemmyClient.list_posts(form).await;

  //             match result {
  //               Ok(mut o) => {
  //                 csr_paginator.set(o.next_page);
  //                 let mut p = csr_infinite_scroll_posts.get().unwrap_or(vec![]);
  //                 p.append(&mut o.posts);
  //                 csr_infinite_scroll_posts.set(Some(p));
  //               }
  //               Err(e) => {
  //                 error.set(Some(e));
  //               }
  //             }
  //           },
  //         );
  //       }
  //     };

  //     window_event_listener_untyped("scroll", on_scroll);
  //   }
  // }

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
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <Transition fallback=|| {}>
        <div class="flex flex-col">
          <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">
            <PostListings posts=posts />
          </div>
        </div>
      // {move || {
      // ssr_posts
      // .get()
      // .unwrap_or(None)
      // .map(|p| {
      // if csr_infinite_scroll_posts.get().is_none() {
      // csr_paginator.set(p.next_page.clone());
      // }
      // view! {
      // <div class="flex flex-col ">
      // <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">

      // <PostListings posts=p.posts.into()/>
      // <PostListings posts=csr_infinite_scroll_posts
      // .get()
      // .unwrap_or_default()
      // .into()/>
      // </div>
      // <div class=" hidden sm:block">

      // {if let Some(s) = ssr_prev() {
      // if !s.is_empty() {
      // let mut st = s.split(',').collect::<Vec<_>>();
      // let p = st.pop().unwrap_or("");
      // let mut query_params = query.get();
      // query_params.insert("prev".into(), st.join(",").to_string());
      // query_params.insert("from".into(), p.into());
      // view! {
      // <span>
      // <A
      // href=format!("{}", query_params.to_query_string())
      // class="btn"
      // >
      // "Prev"
      // </A>
      // </span>
      // }
      // } else {
      // view! { <span></span> }
      // }
      // } else {
      // view! { <span></span> }
      // }}
      // {if let Some(n) = p.next_page.clone() {
      // let s = ssr_prev().unwrap_or_default();
      // let mut st = s.split(',').collect::<Vec<_>>();
      // let f = if let Some(PaginationCursor(g)) = from_func() {
      // g
      // } else {
      // "".to_string()
      // };
      // st.push(&f);
      // let mut query_params = query.get();
      // query_params.insert("prev".into(), st.join(",").to_string());
      // query_params.insert("from".into(), n.0);
      // view! {
      // <span>
      // <A href=format!("{}", query_params.to_query_string()) class="btn">
      // "Next"
      // </A>
      // </span>
      // }
      // } else {
      // view! { <span></span> }
      // }}

      // </div>
      // </div>
      // }
      // })
      // }}

      </Transition>

      <div class="sm:w-1/3 md:1/4 w-full flex-shrink flex-grow-0 hidden lg:block">
        <Trending/>
        <SiteSummary/>
      </div>
    </main>
  }
}
