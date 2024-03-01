use crate::{
  i18n::*,
  queries::site_state_query::use_site_state,
  ui::components::common::icon::{
    Icon,
    IconType::{Donate, Notifications, Search},
  },
  utils::derive_query_signal::derive_query_signal,
};
use lemmy_client::LemmyRequest;
use leptos::{server_fn::error::NoCustomError, *};
use leptos_query::QueryResult;
use leptos_router::*;

#[server(LogoutAction, "/serverfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  use crate::utils::get_client_and_session::get_client_and_session;
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>("jwt")?;
  client
    .logout(LemmyRequest::from_jwt(jwt))
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

  session.purge();
  Ok(())
}

// #[server(ChangeLangFn, "/serverfn")]
// pub async fn change_lang(lang: String) -> Result<(), ServerFnError> {
//   let _ = set_cookie(
//     "i18n_pref_locale",
//     &lang.to_lowercase(),
//     &core::time::Duration::from_secs(604800),
//   )
//   .await;
//   Ok(())
// }

// #[server(ChangeThemeFn, "/serverfn")]
// pub async fn change_theme(theme: String) -> Result<(), ServerFnError> {
//   // use leptos_actix::redirect;
//   let r = set_cookie("theme", &theme, &core::time::Duration::from_secs(604800)).await;
//   match r {
//     Ok(_o) => Ok(()),
//     Err(_e) => {
//       // redirect(&format!("/login?error={}", serde_json::to_string(&e)?)[..]);
//       Ok(())
//     }
//   }
// }

#[component]
pub fn TopNav() -> impl IntoView {
  let i18n = use_i18n();

  let QueryResult { data, refetch, .. } = use_site_state().use_query(|| ());

  let user_name = derive_query_signal(data, |data| {
    data
      .my_user
      .as_ref()
      .map(|data| data.local_user_view.person.name.clone())
  });

  let display_name = derive_query_signal(data, |data| {
    data
      .my_user
      .as_ref()
      .map(|data| data.local_user_view.person.display_name.clone())
      .flatten()
  });

  let instance_name = derive_query_signal(data, |data| data.site_view.site.name.clone());

  let logout_action = create_server_action::<LogoutAction>();

  Effect::new_isomorphic(move |_| {
    if logout_action.version().with(|v| *v > 0) {
      refetch();
    }
  });

  // let ui_theme = expect_context::<RwSignal<Option<String>>>();
  // let theme_action = create_server_action::<ChangeThemeFn>();

  // let on_theme_submit = move |theme_name: &'static str| {
  //   move |ev: SubmitEvent| {
  //     ev.prevent_default();
  //     let _res = create_local_resource(
  //       move || theme_name.to_string(),
  //       move |t| async move {
  //         let _ = set_cookie("theme", &t, &core::time::Duration::from_secs(604800)).await;
  //       },
  //     );
  //     ui_theme.set(Some(theme_name.to_string()));
  //   }
  // };

  // let lang_action = create_server_action::<ChangeLangFn>();

  // let on_lang_submit = move |lang: Locale| {
  //   move |ev: SubmitEvent| {
  //     ev.prevent_default();
  //     i18n.set_locale(lang);
  //   }
  // };

  view! {
    <Transition>
      <nav class="navbar container mx-auto">
        <div class="navbar-start">
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/" class="text-xl whitespace-nowrap">
                {instance_name}
              </A>
            </li>
            <li>
              <A href="/communities" class="text-md">
                {t!(i18n, communities)}
              </A>
            </li>
            <li>
              <A href="/create_post" class="text-md">
                {t!(i18n, create_post)}
              </A>
            </li>
            <li>
              <A href="/create_community" class="text-md">
                {t!(i18n, create_community)}
              </A>
            </li>
            <li>
              <a href="//join-lemmy.org/donate">
                <span title="t!(i18n, donate)">
                  <Icon icon=Donate/>
                </span>
              </a>
            </li>
          </ul>
        </div>
        <div class="navbar-end">
          <ul class="menu menu-horizontal flex-nowrap">
            <li>
              <A href="/search">
                <span title="t!(i18n, search)">
                  <Icon icon=Search/>
                </span>
              </A>
            </li>
            // <li class="z-[1]">
            // <details>
            // <summary>"Lang"</summary>
            // <ul>
            // <li>
            // <ActionForm action=lang_action on:submit=on_lang_submit(Locale::fr)>
            // <input type="hidden" name="lang" value="FR"/>
            // <button type="submit">"FR"</button>
            // </ActionForm>
            // </li>
            // <li>
            // <ActionForm action=lang_action on:submit=on_lang_submit(Locale::en)>
            // <input type="hidden" name="lang" value="EN"/>
            // <button type="submit">"EN"</button>
            // </ActionForm>
            // </li>
            // </ul>
            // </details>
            // </li>
            // <li class="z-[1]">
            // <details>
            // <summary>"Theme"</summary>
            // <ul>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("dark")>
            // <input type="hidden" name="theme" value="dark"/>
            // <button type="submit">"Dark"</button>
            // </ActionForm>
            // </li>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("light")>
            // <input type="hidden" name="theme" value="light"/>
            // <button type="submit">"Light"</button>
            // </ActionForm>
            // </li>
            // <li>
            // <ActionForm action=theme_action on:submit=on_theme_submit("retro")>
            // <input type="hidden" name="theme" value="retro"/>
            // <button type="submit">"Retro"</button>
            // </ActionForm>
            // </li>
            // </ul>
            // </details>
            // </li>
            <Show
              when=move || { with!(| user_name | user_name.as_ref().is_some_and(Option::is_some)) }

              fallback=move || {
                  view! {
                    <li>
                      <A href="/login">{t!(i18n, login)}</A>
                    </li>
                    <li>
                      <A href="/signup">{t!(i18n, signup)}</A>
                    </li>
                  }
              }
            >

              <li>
                <A href="/inbox">
                  <span title=t!(i18n, unread_messages)>
                    <Icon icon=Notifications/>
                  </span>
                </A>
              </li>
              <li>
                <details>
                  <summary>
                    {move || {
                        with!(
                            | user_name, display_name | display_name.as_ref().map(Clone::clone)
                            .flatten().or_else(|| user_name.as_ref().map(Clone::clone).flatten())
                        )
                    }}

                  </summary>
                  <ul class="z-10">
                    <li>
                      <A href=move || {
                          with!(
                              | user_name | user_name.as_ref().map(Clone::clone).flatten().map(|
                              user_name | format!("/u/{}", user_name)).unwrap_or_default()
                          )
                      }>{t!(i18n, profile)}</A>
                    </li>
                    <li>
                      <A href="/settings">{t!(i18n, settings)}</A>
                    </li>
                    <div class="divider my-0"></div>
                    <li>
                      <ActionForm action=logout_action>
                        <button type="submit">{t!(i18n, logout)}</button>
                      </ActionForm>
                    </li>
                  </ul>
                </details>
              </li>
            </Show>
          </ul>
        </div>
      </nav>
    </Transition>
  }
}

#[component]
pub fn BottomNav() -> impl IntoView {
  let i18n = use_i18n();
  const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  let QueryResult { data, .. } = use_site_state().use_query(|| ());

  let version = derive_query_signal(data, |data| data.version.clone());

  view! {
    <nav class="container navbar mx-auto hidden sm:flex">
      <div class="navbar-start w-auto"></div>
      <div class="navbar-end grow w-auto">
        <ul class="menu menu-horizontal flex-nowrap items-center">
          <li>
            <a href="//github.com/LemmyNet/lemmy-ui-leptos/releases" class="text-md">
              "FE: "
              {FE_VERSION}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet/lemmy/releases" class="text-md">
              {move || {
                  with!(
                      | version | format!("BE: {}", version.as_ref().map(Clone::clone)
                      .unwrap_or_else(|| String::from("Lemmy")))
                  )
              }}

            </a>
          </li>
          <li>
            <A href="/modlog" class="text-md">
              {t!(i18n, modlog)}
            </A>
          </li>
          <li>
            <A href="/instances" class="text-md">
              {t!(i18n, instances)}
            </A>
          </li>
          <li>
            <a href="//join-lemmy.org/docs/en/index.html" class="text-md">
              {t!(i18n, docs)}
            </a>
          </li>
          <li>
            <a href="//github.com/LemmyNet" class="text-md">
              {t!(i18n, code)}
            </a>
          </li>
          <li>
            <a href="//join-lemmy.org" class="text-md">
              "join-lemmy.org"
            </a>
          </li>
        </ul>
      </div>
    </nav>
  }
}
