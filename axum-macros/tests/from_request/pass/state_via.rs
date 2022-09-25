use axum::{
    extract::{FromRef, State},
    routing::get,
    Router,
};
use axum_macros::FromRequest;

fn main() {
    let _: Router<AppState> = Router::with_state(AppState::default())
        .route("/b", get(|_: (), _: AppState| async {}))
        .route("/c", get(|_: (), _: InnerState| async {}));
}

#[derive(Clone, Default, FromRequest)]
#[from_request(via(State), state(AppState))]
struct AppState {
    inner: InnerState,
}

#[derive(Clone, Default, FromRequest)]
#[from_request(via(State), state(AppState))]
struct InnerState {}

impl FromRef<AppState> for InnerState {
    fn from_ref(input: &AppState) -> Self {
        input.inner.clone()
    }
}
