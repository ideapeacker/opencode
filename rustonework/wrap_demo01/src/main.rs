use sqlx;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::http::Method;
use warp::Filter;

mod error;
mod profanity;
mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "wrap_demo01=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        )
    });
    // "postgres://username:password@localhost:5432/rustwebdev"
    // Linux postgres 账户密码 : postgres123!
    // POSTGRESQL 默认管理员账户 postgres 密码 : postgres1234!
    let store =
        store::Store::new("postgres://postgres:postgres1234!@localhost:5432/rustwebdev").await;

    sqlx::migrate!().run(&store.clone().connection).await?;

    let store_filter = warp::any().map(move || store.clone());
    //let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions);
    // .with(warp::trace(|info| {
    //     tracing::info_span!("get_questions request", method = %info.method(),
    //     path = %info.path(),
    //     id = %uuid::Uuid::new_v4())
    // }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authenticate::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authenticate::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authenticate::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answer"))
        .and(warp::path::end())
        .and(routes::authenticate::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authenticate::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authenticate::login);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
       // .with(log)
        .with(warp::trace::request())
        .recover(error::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}
