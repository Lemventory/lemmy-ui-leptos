use crate::contexts::site_resource_context::SiteResource;
use leptos::prelude::{Read, Signal};

pub fn derive_user_is_logged_in(site_signal: SiteResource) -> Signal<bool> {
  Signal::derive(move || {
    site_signal
      .read()
      .as_ref()
      .and_then(|data| data.as_ref().ok())
      .is_some_and(|s| s.my_user.is_some())
  })
}
