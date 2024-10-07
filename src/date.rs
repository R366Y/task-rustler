use anyhow::Result;
use chrono::NaiveDate;

pub const DATE_FORMAT: &str = "%d-%m-%Y";
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Date(pub Option<NaiveDate>);

impl TryFrom<String> for Date {
    type Error = chrono::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match NaiveDate::parse_from_str(&value, DATE_FORMAT) {
            Ok(date) => Ok(Date(Some(date))),
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<Date> for String {
    type Error = &'static str;

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        match value.0 {
            Some(date) => Ok(date.format(DATE_FORMAT).to_string()),
            None => Err("Cannot convert None to String"),
        }
    }
}
