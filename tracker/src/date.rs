use std::str::FromStr;

use chrono::{DateTime, Datelike, Local, Month};
use num_traits::FromPrimitive;

use crate::ExpenseParseError;

#[derive(Debug)]
pub enum DateError {
    EmptyDay,
    EmptyMonth,
    InvalidDay(String),
    InvalidMonth(String),
}

impl ToString for DateError {
    fn to_string(&self) -> String {
        match self {
            DateError::EmptyDay => String::from("Date day input was empty"),
            DateError::EmptyMonth => String::from("Date month input was empty"),
            DateError::InvalidDay(e) => format!("Date day invalid input ({e})"),
            DateError::InvalidMonth(e) => format!("Date month invalid input ({e})"),
        }
    }
}

impl Into<ExpenseParseError> for DateError {
    fn into(self) -> ExpenseParseError {
        ExpenseParseError::InvalidDate(self.to_string())
    }
}

#[derive(Debug)]
pub struct Date {
    day: u32,
    month: Month,
}

impl Date {
    pub fn new(day: u32, month: Month) -> Self {
        Self { day, month }
    }

    fn parse_month(month: &str) -> Result<Month, DateError> {
        match month.to_lowercase().as_str() {
            "jan" => Ok(Month::January),
            "feb" => Ok(Month::February),
            "mar" => Ok(Month::March),
            "apr" => Ok(Month::April),
            "may" => Ok(Month::May),
            "jun" => Ok(Month::June),
            "jul" => Ok(Month::July),
            "aug" => Ok(Month::August),
            "sep" => Ok(Month::September),
            "oct" => Ok(Month::October),
            "nov" => Ok(Month::November),
            "dec" => Ok(Month::December),
            unknown => Err(DateError::InvalidMonth(unknown.to_owned())),
        }
    }
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "today" => Ok(Self::default()),

            _ => {
                let mut date_iter = s.split_whitespace();
                let day = date_iter.next().ok_or(DateError::EmptyDay)?;
                let month = date_iter.next().ok_or(DateError::EmptyMonth)?;

                Ok(Self::new(
                    day.parse()
                        .map_err(|_| DateError::InvalidDay(day.to_owned()))?,
                    Self::parse_month(month)?,
                ))
            }
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        let date: DateTime<Local> = Local::now();

        Self {
            day: date.day(),
            // Safety: as long as using the DateTime struct it will not panic
            month: Month::from_u32(date.month()).unwrap(),
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.day, self.month)
    }
}
