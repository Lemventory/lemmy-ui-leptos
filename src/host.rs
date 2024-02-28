<<<<<<< HEAD
use crate::config::{LEMMY_UI_HTTPS, LEMMY_UI_LEMMY_INTERNAL_HOST};
=======
use crate::config::{HTTPS, INTERNAL_HOST};
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
use cfg_if::cfg_if;
use lemmy_client::{ClientOptions, LemmyClient};

#[cfg(feature = "ssr")]
fn get_internal_host() -> String {
<<<<<<< HEAD
  std::env::var("LEMMY_UI_LEMMY_INTERNAL_HOST")
    .unwrap_or_else(|_| LEMMY_UI_LEMMY_INTERNAL_HOST.into())
=======
  std::env::var("INTERNAL_HOST").unwrap_or_else(|_| INTERNAL_HOST.into())
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
}

#[cfg(not(feature = "ssr"))]
fn get_external_host() -> String {
<<<<<<< HEAD
  cfg_if! {
    if #[cfg(not(feature = "bypass_internal_proxy"))] {
      let location = leptos::window().location();

      format!(
        "{}:{}",
        location.hostname().unwrap(),
        location.port().unwrap()
      )
    } else {
      if let Some(s) = option_env!("LEMMY_UI_LEMMY_INTERNAL_HOST") {
        s.into()
      } else {
        LEMMY_UI_LEMMY_INTERNAL_HOST.into()
      }
    }
  }
=======
  let location = leptos::window().location();

  format!(
    "{}:{}",
    location.hostname().unwrap(),
    location.port().unwrap()
  )
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
}

pub fn get_host() -> String {
  cfg_if! {
      if #[cfg(feature="ssr")] {
        get_internal_host()
      } else {
        get_external_host()
      }
  }
}

pub fn get_https() -> String {
  cfg_if! {
      if #[cfg(feature="ssr")] {
<<<<<<< HEAD
        std::env::var("LEMMY_UI_HTTPS").unwrap_or(format!("{LEMMY_UI_HTTPS}"))
      } else {
        if let Some(s) = option_env!("LEMMY_UI_HTTPS") {
          s.into()
        } else {
          format!("{LEMMY_UI_HTTPS}")
        }
=======
        std::env::var("HTTPS").unwrap_or_else(|_| format!("{HTTPS}"))
      } else {
        option_env!("HTTPS").map_or_else(|_| format!("{HTTPS}"), Into::into)
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
      }
  }
}

fn should_use_https() -> bool {
  let https_env_var;
  cfg_if! {
      if #[cfg(feature="ssr")] {
<<<<<<< HEAD
        https_env_var = std::env::var("LEMMY_UI_HTTPS");
      } else {
        https_env_var = option_env!("LEMMY_UI_HTTPS");
      }
  }

  https_env_var.map_or(LEMMY_UI_HTTPS, |var| var == "true")
=======
        https_env_var = std::env::var("HTTPS");
      } else {
        https_env_var = option_env!("HTTPS");
      }
  };

  https_env_var.map_or(HTTPS, |var| var == "true")
>>>>>>> 9c66740 (Add lemmy client and start changes to accomodate it)
}

pub fn get_client() -> LemmyClient {
  LemmyClient::new(ClientOptions {
    domain: get_host(),
    secure: should_use_https(),
  })
}
