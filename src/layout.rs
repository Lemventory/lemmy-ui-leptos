<<<<<<< HEAD
use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav},
};
use lemmy_client::lemmy_api_common::site::GetSiteResponse;
=======
use crate::ui::components::common::nav::{BottomNav, TopNav};
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, RoutingProgress};

#[component]
<<<<<<< HEAD
pub fn Layout() -> impl IntoView {
=======
pub fn Layout(is_routing: ReadSignal<bool>) -> impl IntoView {
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
  let ui_theme = expect_context::<RwSignal<Option<String>>>();

  view! {
    <ErrorBoundary fallback=|_| view! { Error! }>
      <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
      <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
      <Meta name="description" content="Lemmy-UI-Leptos."/>
      <Meta name="viewport" content="viewport-fit=cover"/>
      // debug where there is no visible console (mobile/live/desktop)
      // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
      // <Script>eruda.init();</Script>
      <Title text="Brand from env"/>

      <div class="flex flex-col h-screen" data-theme=ui_theme>
        <TopNav/>
        <Outlet/>
        <BottomNav/>
      </div>
    </ErrorBoundary>
  }
}
