use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            response::{Response, IntoResponse},
            routing::get,
            extract::{Path, State, RawQuery},
            http::{Request, header::HeaderMap},
            body::Body as AxumBody,
            Router,
        };
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use leptos::{log, view, provide_context, get_configuration};
        use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
        use expences_splitter::state::AppState;
        use expences_splitter::app::App;

        async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
            request: Request<AxumBody>) -> impl IntoResponse {

            log!("{:?}", path);

            handle_server_fns_with_context(path, headers, raw_query, move |cx| {
                provide_context(cx, app_state.pool.clone());
            }, request).await
        }

        async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
                move |cx| {
                    provide_context(cx, app_state.pool.clone());
                },
                |cx| view! { cx, <App/> }
            );
            handler(req).await.into_response()
        }


        #[tokio::main]
        async fn main() {
            use expences_splitter::app::*;
            use expences_splitter::fileserv::file_and_error_handler;

            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

             let pool = SqlitePoolOptions::new()
                .connect("sqlite:expences.db")
                .await
                .expect("Could not make pool.");

            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("could not run SQLx migrations");

            let app_state = AppState{
                leptos_options,
                pool: pool.clone(),
                routes: routes.clone(),
            };

            // build our application with a route
            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
                .fallback(file_and_error_handler)
                .with_state(app_state);

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    } else {
        pub fn main() {
            // no client-side main function
            // unless we want this to work with e.g., Trunk for a purely client-side app
            // see lib.rs for hydration
        }
    }

}
