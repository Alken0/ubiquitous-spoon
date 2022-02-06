use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::max;

use super::DEFAULT_RANGE;

#[derive(Default, Clone, Copy)]
pub struct Range(fs::Range);

impl Range {
    pub fn start(&self) -> u64 {
        self.0.start()
    }

    /// returns start + offset or None if the number is too big
    pub fn end(&self) -> Option<u64> {
        let offset = self.0.offset().checked_sub(1).unwrap_or(0);
        self.0.start().checked_add(offset)
    }

    pub fn range(&self) -> fs::Range {
        self.0
    }

    pub fn apply_file_size(&self, file_size: u64) -> Range {
        Range(self.0.apply_filesize(file_size))
    }
}

#[async_trait]
impl<B> FromRequest<B> for Range
where
    B: Send, // required by `async_trait`
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = expect(req.headers())?;
        let range_header = expect(headers.get("Range"))?;
        let range_header = expect(range_header.to_str().ok())?;
        let range = parse_range_with_defaults(range_header);
        return Ok(Range(range));
    }
}

static REGEX_NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]+").unwrap());
fn parse_range_with_defaults(header_value: &str) -> fs::Range {
    let mut reversed_numbers: Vec<u64> = REGEX_NUMBERS
        .find_iter(header_value)
        .map(|e| e.as_str())
        .map(|e| e.parse::<u64>().expect("invalid regex"))
        .collect();
    reversed_numbers.reverse();

    let start = reversed_numbers.pop().unwrap_or(0);
    let end = max(
        start,
        reversed_numbers.pop().unwrap_or(start + DEFAULT_RANGE),
    );
    return fs::Range::new(start, end - start);
}

fn expect<T>(value: Option<T>) -> Result<T, StatusCode> {
    match value {
        Some(v) => Ok(v),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
