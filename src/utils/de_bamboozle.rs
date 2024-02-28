use leptos::{with, ServerFnError, Signal};

pub fn de_bamboozle<T, R>(
  base_result: Signal<Option<Result<T, ServerFnError>>>,
  map_result: fn(&T) -> R,
) -> Signal<Option<Result<R, ServerFnError>>> {
  Signal::derive(move || {
    with!(|base_result| base_result.as_ref().map(|foo| foo
      .as_ref()
      .map_err(Clone::clone)
      .map(|res| map_result(res))))
  })
}
