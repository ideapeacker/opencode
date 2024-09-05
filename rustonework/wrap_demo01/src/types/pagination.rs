use crate::error;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Pagination {
    // start: usize,
    // end: usize,
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse::<i32>()
                .map_err(error::Error::ParseError)?),
            offset: params
                .get("end")
                .unwrap()
                .parse::<i32>()
                .map_err(error::Error::ParseError)?,
        });
    } else {
        Err(error::Error::MissingParameters)
    }
}
