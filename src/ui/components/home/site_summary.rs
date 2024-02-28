use crate::{
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::counts_badge::CountsBadge,
  utils::derive_query_signal::derive_query_signal,
};
use leptos::*;
use leptos_query::QueryResult;

#[component]
pub fn SiteSummary() -> impl IntoView {
  let _i18n = use_i18n();
  let QueryResult { data, .. } = use_site_state().use_query(|| ());

  let site_name = derive_query_signal(data, |data| data.site_view.site.name.clone());
  let site_description = derive_query_signal(data, |data| {
    data.site_view.site.description.clone().unwrap_or_default()
  });

  let users_active_day = derive_query_signal(data, |data| data.site_view.counts.users_active_day);
  let users_active_week = derive_query_signal(data, |data| data.site_view.counts.users_active_week);
  let users_active_month =
    derive_query_signal(data, |data| data.site_view.counts.users_active_month);
  let users_active_half_year =
    derive_query_signal(data, |data| data.site_view.counts.users_active_half_year);
  let users = derive_query_signal(data, |data| data.site_view.counts.users);
  let communities = derive_query_signal(data, |data| data.site_view.counts.communities);
  let posts = derive_query_signal(data, |data| data.site_view.counts.posts);
  let comments = derive_query_signal(data, |data| data.site_view.counts.comments);

  let admins = derive_query_signal(data, |data| {
    data
      .admins
      .iter()
      .map(|admin| (admin.person.id, admin.person.name.clone()))
      .collect::<Vec<_>>()
  });

  view! {
    <div class="card w-full bg-base-300 text-base-content mb-3">
      <figure>
        <div class="card-body bg-neutral">
          <h2 class="card-title text-neutral-content">{site_name}</h2>
        </div>
      </figure>
      <div class="card-body">
        <p>{site_description}</p>
        <p>
          <CountsBadge>{users_active_day} " users / day"</CountsBadge>
          &nbsp;
          <CountsBadge>{users_active_week} " users / week"</CountsBadge>
          &nbsp;
          <CountsBadge>{users_active_month} " users / month"</CountsBadge>
          &nbsp
          <CountsBadge>{users_active_half_year} " users / 6 months"</CountsBadge>
          &nbsp;
          <CountsBadge>{users} " users"</CountsBadge>
          &nbsp;
          <CountsBadge>{communities} " communities"</CountsBadge>
          &nbsp;
          <CountsBadge>{posts} " posts"</CountsBadge>
          &nbsp;
          <CountsBadge>{comments} " comments"</CountsBadge>
          &nbsp;
          <CountsBadge>Modlog</CountsBadge>
        </p>
        <h3 class="card-title">Admins</h3>
        <p>
          <For
            each=move || { admins.get().unwrap_or_default() }

            key=|a| a.0
            children=move |a| {
                view! { <CountsBadge>{a.1}</CountsBadge> }
            }
          />

        </p>
      </div>
    </div>
  }
}
