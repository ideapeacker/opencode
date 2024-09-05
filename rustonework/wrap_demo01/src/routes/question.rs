use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};
use std::collections::HashMap;
use warp::http::StatusCode;

use warp::reply::Reply;
use warp::Rejection;

use crate::error::Error;
use crate::profanity;
use crate::types::account::Session;
use tracing::{event, info, instrument, Level};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "wrap_demo01", Level::INFO, "querying questions");

    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
    info!(pagination = false);
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => {
            //info!(pagination = true);

            //let questions = store.get(Some(pagination)).await;

            //Ok(warp::reply::json(&questions))
            res
        }
        Err(e) => {
            return Err(warp::reject::custom(e));
        }
    };
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;

    // let title = match profanity::check_profanity(new_question.title).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(e)),
    // };

    // let content = match profanity::check_profanity(new_question.content).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(e)),
    // };

    let question = NewQuestion {
        title: new_question.title,
        content: new_question.content,
        tags: new_question.tags,
    };

    match store.add_question(question, &account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;

    event!(target: "wrap_demo01", Level::INFO, "update questions ID={}", id);
    tracing::event!(tracing::Level::ERROR, "Ready to update question id={}", id);
    if store.is_question_owner(id, &account_id).await? {
        // let title = profanity::check_profanity(question.title);
        // let content = profanity::check_profanity(question.content);

        // let (title, content) = tokio::join!(title, content);

        // if title.is_ok() && content.is_ok() {

        // } else {
        //     Err(warp::reject::custom(
        //         title.expect_err("Expected API call to have failed here"),
        //     ))
        // }
        let question = Question {
            id: question.id,
            title: question.title,
            content: question.content,
            tags: question.tags,
        };

        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => return Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => Ok(warp::reply::with_status(
                format!("Question {} deleted", id),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(Error::Unauthorized))
    }
}
