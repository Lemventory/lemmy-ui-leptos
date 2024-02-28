mod config;
mod errors;
pub mod host;
mod layout;
mod queries;
#[cfg(feature = "ssr")]
pub mod server;
mod ui;
mod utils;

use crate::{
  i18n::*,
  layout::Layout,
  ui::components::{
    communities::communities_activity::CommunitiesActivity,
    home::home_activity::HomeActivity,
    login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};
use leptos::*;
use leptos_meta::*;
use leptos_query::provide_query_client;
use leptos_router::*;

leptos_i18n::load_locales!();

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_i18n_context();
  provide_query_client();

  let ui_theme = create_rw_signal::<String>(String::from("retro"));
  provide_context(ui_theme);

  let (is_routing, set_is_routing) = create_signal(false);
  view! {
    <Router set_is_routing=set_is_routing>
      <Routes>
        <Route path="/" view=move || view! { <Layout is_routing=is_routing/> } ssr=SsrMode::Async>
          <Route path="/*any" view=NotFound/>

          <Route path="" view=HomeActivity/>

          <Route path="create_post" view=CommunitiesActivity/>
          <Route path="post/:id" view=PostActivity/>

          <Route path="search" view=CommunitiesActivity/>
          <Route path="communities" view=CommunitiesActivity/>
          <Route path="create_community" view=CommunitiesActivity/>
          <Route path="c/:id" view=CommunitiesActivity/>

          <Route path="login" view=LoginActivity/>
          <Route path="logout" view=CommunitiesActivity/>
          <Route path="signup" view=CommunitiesActivity/>

          <Route path="inbox" view=CommunitiesActivity/>
          <Route path="settings" view=CommunitiesActivity/>
          <Route path="u/:id" view=CommunitiesActivity/>

          <Route path="modlog" view=CommunitiesActivity/>
          <Route path="instances" view=CommunitiesActivity/>
        </Route>
      </Routes>
    </Router>
  }
}

#[component]
fn NotFound() -> impl IntoView {
  #[cfg(feature = "ssr")]
  {
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! { <h1>"Not Found"</h1> }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
