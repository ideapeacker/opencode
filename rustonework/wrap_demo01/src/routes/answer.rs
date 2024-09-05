use crate::store::Store;

use crate::types::answer::NewAnswer;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::types::account::Session;

pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl Reply, Rejection> {

    let account_id = session.account_id;
    match store.add_answer(new_answer, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
