use crate::{
  contexts::site_resource_context::SiteResource,
  utils::{derive_user_is_logged_in, ServerAction, ServerActionFn},
};
use lemmy_client::{
  lemmy_api_common::{
    lemmy_db_schema::newtypes::PersonId,
    person::{BlockPerson, BlockPersonResponse},
  },
  LemmyRequest,
};
use leptos::*;
use leptos_router::{ActionForm, A};
use phosphor_leptos::{Bookmark, ChatText, DotsThreeVertical, Flag, IntersectThree, Prohibit};

mod post_content_actions;
pub use post_content_actions::PostContentActions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentActionType {
  Post { comments: MaybeSignal<i64> },
  Comment,
}

#[server(prefix = "/serverfn")]
pub async fn block_user(id: PersonId, block: bool) -> Result<BlockPersonResponse, ServerFnError> {
  use crate::{constants::AUTH_COOKIE, utils::get_client_and_session};
  let (client, session) = get_client_and_session().await?;

  let jwt = session.get::<String>(AUTH_COOKIE)?;

  client
    .block_person(LemmyRequest {
      body: BlockPerson {
        person_id: id,
        block,
      },
      jwt,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
fn ContentActions<SA, RA>(
  content_action_type: ContentActionType,
  #[prop(into)] id: MaybeSignal<i32>,
  save_action: ServerAction<SA>,
  #[prop(into)] saved: MaybeSignal<bool>,
  report_action: ServerAction<RA>,
  #[prop(into)] creator_id: MaybeSignal<i32>,
) -> impl IntoView
where
  SA: ServerActionFn,
  RA: ServerActionFn,
{
  let site_resource = expect_context::<SiteResource>();
  let user_is_logged_in = derive_user_is_logged_in(site_resource);

  let block_user_action = Action::<BlockUser, _>::server();

  view! {
    <div class="flex items-center gap-x-2">
      {move || {
          if let ContentActionType::Post { comments } = content_action_type {
              Some(
                  view! {
                    <span
                      class="flex items-center"
                      title=move || format!("{} comments", comments.get())
                    >
                      <A
                        href=move || { format!("/post/{}", id.get()) }
                        class="text-sm whitespace-nowrap"
                      >
                        <ChatText class="size-6 inline"/>
                        " "
                        {comments}
                      </A>
                    </span>
                  },
              )
          } else {
              None
          }
      }}
      <ActionForm action=save_action class="flex items-center">
        <input type="hidden" name="id" value=id/>
        <input type="hidden" name="save" value=saved/>
        <button
          type="submit"
          title=if matches!(content_action_type, ContentActionType::Comment) {
              "Save comment"
          } else {
              "Save post"
          }

          class=move || if saved.get() { " text-accent" } else { "" }
          disabled=move || !user_is_logged_in.get() || save_action.pending().get()
        >
          <Bookmark class="size-6"/>
        </button>
      </ActionForm>
      <Show when=move || matches!(content_action_type, ContentActionType::Post { .. })>
        <A href="/create_post">
          <IntersectThree class="size-6"/>
        </A>
      </Show> <div class="dropdown hidden sm:block">
        <label tabindex="0">
          <DotsThreeVertical class="size-6"/>
        </label>
        <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <li>
            <ActionForm action=report_action>
              <input type="hidden" name="id" value=id/>
              <input type="text" name="reason" placeholder="reason"/>
              <button class="text-xs whitespace-nowrap" title="Report post" type="submit">
                <Flag class="size-6 inline-block"/>
                {if matches!(content_action_type, ContentActionType::Comment) {
                    " Report comment"
                } else {
                    " Report post"
                }}

              </button>
            </ActionForm>
          </li>
          <li>
            <ActionForm action=block_user_action>
              <input type="hidden" name="id" value=creator_id/>
              <input type="hidden" name="block" value="true"/>
              <button class="text-xs whitespace-nowrap" title="Block user" type="submit">
                <Prohibit class="size-6 inline-block"/>
                " Block user"
              </button>
            </ActionForm>
          </li>
        </ul>
      </div>
    </div>
  }
}
