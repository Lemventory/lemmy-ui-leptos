use crate::{
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::counts_badge::CountsBadge,
  utils::de_bamboozle::de_bamboozle,
};
use leptos::*;
use leptos_query::QueryResult;

#[component]
pub fn SiteSummary() -> impl IntoView {
  let _i18n = use_i18n();
  let QueryResult { data, .. } = use_site_state().use_query(|| ());

  let site_name = de_bamboozle(data, |data| data.site_view.site.name.clone());
  let site_description = de_bamboozle(data, |data| data.site_view.site.description.clone());

  let users_active_day = de_bamboozle(data, |data| data.site_view.counts.users_active_day);
  let users_active_week = de_bamboozle(data, |data| data.site_view.counts.users_active_week);
  let users_active_month = de_bamboozle(data, |data| data.site_view.counts.users_active_month);
  let users_active_half_year =
    de_bamboozle(data, |data| data.site_view.counts.users_active_half_year);
  let users = de_bamboozle(data, |data| data.site_view.counts.users);
  let communities = de_bamboozle(data, |data| data.site_view.counts.communities);
  let posts = de_bamboozle(data, |data| data.site_view.counts.posts);
  let comments = de_bamboozle(data, |data| data.site_view.counts.comments);

  let admins = de_bamboozle(data, |data| {
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
            each=move || {
                with!(
                    | admins | admins.as_ref().and_then(| admins | admins.as_ref().ok().cloned())
                    .unwrap_or_default()
                )
            }

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
