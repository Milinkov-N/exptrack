mod date;
mod item;

pub mod prelude {
    pub use super::Expense;
    pub use super::ExpenseParseError;
    pub use super::Store;
}

use std::str::FromStr;

use date::Date;
use item::Item;

#[derive(Debug)]
pub enum ExpenseParseError {
    EmptyDate,
    EmptyStore,
    EmptyItem,
    InvalidDate(String),
    InvalidStore(String),
    InvalidItem(String),
    InvalidTotalPrice(String),
}

#[derive(Debug)]
pub struct Expense {
    pub date: Date,
    pub store: Store,
    pub item: Item,
    pub total_price: f64,
}

impl Expense {
    pub fn builder() -> ExpenseBuilder {
        ExpenseBuilder::new()
    }

    pub fn parse_iter<'a, I>(iter: &mut I) -> Result<Self, ExpenseParseError>
    where
        I: Iterator<Item = &'a str>,
    {
        let date = Self::parse_date(iter)?;
        let store = iter.next().ok_or(ExpenseParseError::EmptyStore)?.to_owned();
        let item = iter.next().ok_or(ExpenseParseError::EmptyItem)?;

        Ok(Expense::builder()
            .date(&date)
            .store(&store)
            .item(item)
            .build()?)
    }

    fn parse_date<'a, I>(iter: &mut I) -> Result<String, ExpenseParseError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut date = String::from(iter.next().ok_or(ExpenseParseError::EmptyDate)?);

        if date.as_str() != "today" {
            date.push(' ');
            date.push_str(iter.next().ok_or(ExpenseParseError::EmptyDate)?);
        }

        Ok(date)
    }
}

pub struct ExpenseBuilder {
    date: Result<Date, ExpenseParseError>,
    store: Result<Store, ExpenseParseError>,
    item: Result<Item, ExpenseParseError>,
}

impl ExpenseBuilder {
    pub fn new() -> Self {
        Self {
            date: Err(ExpenseParseError::EmptyDate),
            store: Err(ExpenseParseError::EmptyStore),
            item: Err(ExpenseParseError::EmptyItem),
        }
    }
    pub fn date(mut self, date: &str) -> Self {
        self.date = Date::from_str(date).map_err(|e| e.into());

        self
    }

    pub fn store(mut self, store: &str) -> Self {
        self.store = Store::from_str(store).map_err(|_| ExpenseParseError::EmptyStore);

        self
    }

    pub fn item(mut self, item: &str) -> Self {
        self.item = Item::from_str(item).map_err(|e| e.into());

        self
    }

    pub fn build(self) -> Result<Expense, ExpenseParseError> {
        let total_price = match self.item.as_ref() {
            Ok(item) => item.total_price(),
            Err(_) => 0.0,
        };

        Ok(Expense {
            date: self.date?,
            store: self.store?,
            item: self.item?,
            total_price,
        })
    }
}

#[derive(Debug, Default)]
pub enum Store {
    #[default]
    Magnit,
    Five,
    Lenta,
    Other(String),
}

impl FromStr for Store {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "магнит" | "magnit" => Ok(Store::Magnit),
            "пятерочка" | "пятёрочка" | "пятерка" | "пятёрка" | "five" | "5ka" => {
                Ok(Store::Five)
            }
            "лента" | "lenta" => Ok(Self::Lenta),
            other => Ok(Store::Other(other.to_owned())),
        }
    }
}
