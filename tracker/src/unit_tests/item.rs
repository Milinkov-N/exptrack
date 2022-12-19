use super::*;

macro_rules! item_ok_tc {
    ({ $name:literal, $amount:literal, $price:literal }, $result:ident) => {
        assert_eq!(
            Item {
                name: $name.to_owned(),
                amount: $amount,
                price: $price
            },
            $result
        );
    };
}

macro_rules! item_err_tc {
    ($expecting:ident, $result:ident) => {
        assert_eq!(Err($expecting), $result);
    };
}

#[test]
fn item_from_str_full_syntax() {
    let result = Item::from_str("banana<x3>::66RUB").unwrap();

    item_ok_tc!({ "banana", 3, 66.0 }, result);
}

#[test]
fn item_from_str_no_amount_prefix() {
    let result = Item::from_str("banana<3>::66RUB").unwrap();

    item_ok_tc!({ "banana", 3, 66.0 }, result);
}

#[test]
fn item_from_str_no_price_postfix() {
    let result = Item::from_str("banana<x3>::66").unwrap();

    item_ok_tc!({ "banana", 3, 66.0 }, result);
}

#[test]
fn item_from_str_no_prefix_and_postfix() {
    let result = Item::from_str("banana<3>::66").unwrap();

    item_ok_tc!({ "banana", 3, 66.0 }, result);
}

#[test]
fn item_from_str_float_price() {
    let result = Item::from_str("banana<x3>::66.50RUB").unwrap();

    item_ok_tc!({ "banana", 3, 66.5 }, result);
}

#[test]
fn item_from_str_with_price_underscore() {
    let result = Item::from_str("banana<x3>::66_RUB").unwrap();

    item_ok_tc!({ "banana", 3, 66.0 }, result);
}

#[test]
fn item_from_str_full_syntax_single_colon() {
    let expecting = ItemParseError::EmptyPrice;
    let result = Item::from_str("banana<x3>:66RUB");

    item_err_tc!(expecting, result);
}

#[test]
fn item_from_str_no_open_bracket() {
    let expecting = ItemParseError::InvalidAmount(":66RUB".to_owned());
    let result = Item::from_str("bananax3>:66RUB");

    item_err_tc!(expecting, result);
}

#[test]
fn item_from_str_no_close_bracket() {
    let expecting = ItemParseError::InvalidAmount("x3:66RUB".to_owned());
    let result = Item::from_str("banana<x3:66RUB");

    item_err_tc!(expecting, result);
}
