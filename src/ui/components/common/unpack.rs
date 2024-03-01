use leptos::*;

#[component]
pub fn Unpack<T: 'static, F: FnOnce(&T) -> Fragment>(
  #[prop(into)] item: MaybeSignal<Option<Result<T, ServerFnError>>>,
  children: F,
) -> impl IntoView {
  with!(|item| {
    match item.as_ref().map(Result::as_ref) {
      Some(Ok(item)) => Some(Ok(children(item))),
      Some(Err(e)) => Some(Err(e.clone())),
      _ => None,
    }
  })
}
