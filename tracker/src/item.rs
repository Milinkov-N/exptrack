use std::str::FromStr;

use crate::ExpenseParseError;

#[derive(Debug, PartialEq)]
pub enum ItemParseError {
    EmptyInput,
    EmptyName,
    EmptyAmount,
    InvalidAmount(String),
    EmptyPrice,
    InvalidPrice(String),
}

impl ToString for ItemParseError {
    fn to_string(&self) -> String {
        match self {
            ItemParseError::EmptyInput => String::from("Item input was empty"),
            ItemParseError::EmptyName => String::from("Item name was empty"),
            ItemParseError::EmptyAmount => String::from("Item amount was empty"),
            ItemParseError::InvalidAmount(e) => format!("Item invalid amount ({e})"),
            ItemParseError::EmptyPrice => String::from("Item price was empty"),
            ItemParseError::InvalidPrice(e) => format!("Item invalid price ({e})"),
        }
    }
}

impl Into<ExpenseParseError> for ItemParseError {
    fn into(self) -> ExpenseParseError {
        ExpenseParseError::InvalidItem(self.to_string())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Item {
    name: String,
    amount: u32,
    price: f64,
}

impl Item {
    pub fn total_price(&self) -> f64 {
        self.price * self.amount as f64
    }
}

impl FromStr for Item {
    type Err = ItemParseError;

    /// The syntax for turning string into Item struct look like this:
    /// "\[name\]<\[amount\]>::\[price\]"
    ///
    /// Amount can be prefixed with 'x' character and price postfixed with
    /// either "_RUB" or "RUB"
    ///
    /// ## Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use tracker::Item;
    ///
    /// let item = Item::from_str("banana<x3>::66RUB");
    /// let item = Item::from_str("banana<3>::66_RUB");
    /// let item = Item::from_str("cookies<5>::15");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("::").filter(|s| !s.is_empty());
        let name_and_amount = iter.next().ok_or(ItemParseError::EmptyInput)?;
        let mut na_iter = name_and_amount.split(&['<', '>']).filter(|s| !s.is_empty());

        let name = na_iter.next().ok_or(ItemParseError::EmptyName)?.to_owned();

        let amount_str = na_iter.next().ok_or(ItemParseError::EmptyAmount)?;
        let amount = amount_str
            .trim_start_matches('x')
            .parse()
            .map_err(|_| ItemParseError::InvalidAmount(amount_str.to_owned()))?;

        let price_str = iter.next().ok_or(ItemParseError::EmptyPrice)?;
        let price = price_str
            .trim_end_matches("RUB")
            .trim_end_matches('_')
            .parse()
            .map_err(|_| ItemParseError::InvalidPrice(price_str.to_owned()))?;

        Ok(Item {
            name,
            amount,
            price,
        })
    }
}

#[cfg(test)]
#[path = "unit_tests/item.rs"]
mod item_tests;
