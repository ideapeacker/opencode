use crate::error::Error;
use crate::types::answer::{Answer, AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Question, QuestionId};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::account::{Account, AccountId};

#[derive(Clone, Debug)]
pub struct Store {
    // questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    // answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection:{}", e),
        };
        // Store {
        //     questions: Arc::new(RwLock::new(Self::init())),
        //     answers: Arc::new(RwLock::new(HashMap::new())),
        // }
        Store {
            connection: db_pool,
        }
    }
    // fn init() -> HashMap<QuestionId, Question> {
    //     let file = include_str!("../question.json");
    //     serde_json::from_str(file).expect("can't read question.json")
    // }

    // pub async fn insert(self, answer: Answer) {
    //     let id = answer.id().clone();
    //     self.answers.write().await.insert(id, answer);
    // }
    pub async fn add_question(
        &self,
        new_question: NewQuestion,
        account_id: &AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags, account_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .bind(account_id.0)
            .map(|row: PgRow| Question {
			    id: QuestionId(row.get("id")),
                title: row.get("title"),
			    content: row.get("content"),
                tags: row.get("tags"),
		    })
            .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(error) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", error);
                    Err(Error::DatabaseQueryError(error))
                },
            }
    }

    pub async fn delete_question(
        &self,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1 AND account_id = $2")
            .bind(question_id)
            .bind(account_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 AND account_id = $5 RETURNING id, title, content, tags")
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .bind(question_id)
            .bind(account_id.0)
            .map(|row:PgRow| Question{
                id:QuestionId(row.get("id")),
                title:row.get("title"),
                content:row.get("content"),
                tags: row.get("tags"),
            }).fetch_one(&self.connection).await{
            Ok(question)=>Ok(question),
            Err(e)=> {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }

        }
    }
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error> {
        // let res: Vec<Question> = self.questions.read().await.values().cloned().collect();
        // if let Some(p) = pagination {
        //     if p.start() <= p.end() && res.len() >= p.end() {
        //         res[p.start()..p.end()].to_vec()
        //     } else {
        //         res
        //     }
        // } else {
        //     res
        // }
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);

                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_answer(
        &self,
        new_answer: NewAnswer,
        account_id: AccountId,
    ) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INTO answers (content, corresponding_question, account_id) VALUES ($1, $2, $3)",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .bind(account_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
    pub async fn add_account(self, account: Account) -> Result<bool, Error> {
        match sqlx::query("INSERT INTO accounts(email, password) VALUES($1, $2)")
            .bind(account.email)
            .bind(account.password)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = err
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = err.as_database_error().unwrap().message(),
                    constraint = err.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn get_account(self, email: &str) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM accounts where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    pub async fn is_question_owner(
        &self,
        question_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, Error> {
        match sqlx::query("SELECT * FROM questions where id = $1 and account_id = $1")
            .bind(question_id)
            .bind(account_id.0)
            .fetch_optional(&self.connection)
            .await
        {
            Ok(question) => Ok(question.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
