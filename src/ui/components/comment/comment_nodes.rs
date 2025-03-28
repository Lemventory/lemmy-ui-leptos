use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::prelude::*;

#[component]
pub fn CommentNodes(#[prop(into)] _comments: Signal<Vec<CommentView>>) -> impl IntoView {
  view! {
    <ul>
      <For each=move || _comments.get() key=|cv| cv.comment.id let:cv>
        <li>
          <CommentNode comment_view=cv />
        </li>
      </For>
    </ul>
  }
}
