use crate::{
  contexts::site_resource_context::SiteResource,
  serverfns::users::create_block_user_action,
  ui::components::common::{
    fedilink::Fedilink,
    icon::{Icon, IconType},
  },
  utils::{
    derive_user_is_logged_in,
    traits::ToStr,
    types::{PostOrCommentId, ServerAction, ServerActionFn},
  },
};
use hide_post_button::HidePostButton;
use lemmy_client::lemmy_api_common::lemmy_db_schema::newtypes::PersonId;
use leptos::*;
use leptos_fluent::tr;
use leptos_router::{ActionForm, A};
use report_button::ReportButton;
use tailwind_fuse::tw_join;

mod hide_post_button;
mod report_button;

#[component]
pub fn ContentActions<SA>(
  post_or_comment_id: PostOrCommentId,
  saved: Signal<bool>,
  save_action: ServerAction<SA>,
  creator_id: PersonId,
  creator_actor_id: String,
  creator_name: StoredValue<String>,
  apub_link: String,
) -> impl IntoView
where
  SA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);
  let creator_actor_id = StoredValue::new(creator_actor_id);
  let logged_in_user_id = Signal::derive(move || {
    with!(|site_resource| site_resource
      .as_ref()
      .and_then(|data| data.as_ref().ok().map(|data| data
        .my_user
        .as_ref()
        .map(|data| data.local_user_view.person.id))))
    .flatten()
  });

  let block_user_action = create_block_user_action();

  let save_content_label = if matches!(post_or_comment_id, PostOrCommentId::Post(_)) {
    tr!("save-post")
  } else {
    tr!("save-comment")
  };
  let save_icon = Signal::derive(move || {
    if saved.get() {
      IconType::SaveFilled
    } else {
      IconType::Save
    }
  });
  let crosspost_label = tr!("crosspost");

  view! {
    <Fedilink href=apub_link />
    <Show when=move || user_is_logged_in.get()>
      <ActionForm action=save_action class="flex items-center" clone:save_content_label>
        <input type="hidden" name="id" value=post_or_comment_id.get_id() />
        <input type="hidden" name="save" value=move || (!saved.get()).to_str() />
        <button
          type="submit"
          title=save_content_label.clone()
          aria-label=save_content_label

          class=move || {
              tw_join!(
                  "disabled:cursor-not-allowed disabled:text-neutral-content", saved.get()
                    .then_some("text-accent")
              )
          }

          disabled=move || save_action.pending().get()
        >
          <Icon icon=save_icon />

        </button>
      </ActionForm>
      {(matches!(post_or_comment_id, PostOrCommentId::Post(_)))
          .then(|| {
              view! {
                <A href="/create_post" attr:title=crosspost_label.clone() attr:aria-label=crosspost_label.clone()>
                  <Icon icon=IconType::Crosspost />
                </A>
              }
          })}

      <div class="dropdown">
        <div tabindex="0" role="button">
          <Icon icon=IconType::VerticalDots />
        </div>
        <menu tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <Show when=move || {
              logged_in_user_id.get().map(|id| id != creator_id).unwrap_or(false)
          }>
            {if let PostOrCommentId::Post(id) = post_or_comment_id {
                Some(view! { <HidePostButton id=id /> })
            } else {
                None
            }} <li>
              <ReportButton
                creator_name=creator_name
                post_or_comment_id=post_or_comment_id
                creator_actor_id=creator_actor_id
              />
            </li> <li>
              <ActionForm action=block_user_action>
                <input type="hidden" name="id" value=creator_id.0 />
                <input type="hidden" name="block" value="true" />
                <button class="text-xs whitespace-nowrap" type="submit">
                  <Icon icon=IconType::Block class="inline-block" />
                  " "{tr!("block-user")}
                </button>
              </ActionForm>
            </li>
          </Show>
        </menu>
      </div>
    </Show>
  }
}
