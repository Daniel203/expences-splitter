pub mod app;
pub mod models;
pub mod pages;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::{Connection, SqliteConnection};
    use leptos::ServerFnError;

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:expences.db").await?)
    }
}
}
