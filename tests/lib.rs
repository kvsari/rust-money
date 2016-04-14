extern crate money;

use money::Money;
use money::currency;

#[test]
fn test_basic_formatting() {
    let money = Money::new(12345, currency::USD);
    assert_eq!("USD$123.45", money.format());
}

#[test]
fn test_japan_formatting() {
    let money = Money::new(12345, currency::JPY);
    assert_eq!("JPY¥12345", money.format());
}

#[test]
fn test_create_currency_from_code() {
    let currency = currency::from_code("USD").unwrap();
    assert_eq!(currency.code(), "USD");
    assert_eq!(currency.symbol(), '$');
    assert_eq!(currency.divisor(), 100);
}

#[test]
fn test_create_currency_btc() {
    let currency = currency::from_code("BTC").unwrap();
    assert_eq!(currency.code(), "BTC");
    assert_eq!(currency.symbol(), 'Ƀ');
    assert_eq!(currency.divisor(), 100000000);
}

#[test]
fn test_create_bad_currency_returns_err() {
    let currency = currency::from_code("ABC");
    assert!(currency.is_err());
}
