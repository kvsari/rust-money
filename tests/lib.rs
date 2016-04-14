extern crate money;

use std::cmp::Ordering;

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
fn test_create_bad_currency_returns_err() {
    let currency = currency::from_code("ABC");
    assert!(currency.is_err());
}

#[test]
fn test_currency_eq() {
    let dollars = currency::from_code("USD").unwrap();
    let bucks = currency::from_code("AUD").unwrap();
    let yen = currency::from_code("JPY").unwrap();
    let bux = currency::from_code("USD").unwrap();

    assert!(bux == dollars);
    assert!(dollars == bux);
    assert!(dollars == dollars);
    assert!(dollars != bucks);
    assert!(bucks != yen);
    assert!(yen != bux);
}

#[test]
fn test_money_eq() {
    let money = Money::new(1050, currency::AUD);
    let same = Money::new(1050, currency::AUD);
    let short = Money::new(1000, currency::AUD);
    let american = Money::new(1050, currency::USD);
    
    assert!(money == same);
    assert!(same == same);
    assert!(same == money);
    assert!(money != short);
    assert!(money != american);
}

#[test]
fn test_money_ord() {
    let large = Money::new(1050, currency::USD);
    let same = Money::new(1050, currency::USD);
    let small = Money::new(25, currency::USD);
    let diff = Money::new(1050, currency::AUD);

    assert_eq!(small.partial_cmp(&large), Some(Ordering::Less));
    assert_eq!(large.partial_cmp(&small), Some(Ordering::Greater));
    assert_eq!(large.partial_cmp(&same), Some(Ordering::Equal));
    assert_eq!(large.partial_cmp(&diff), None);

    assert!(large > small);
    assert!(large >= small);
    assert!(small < large);
    assert!(small <= large);
    assert!(large >= same);
    assert!(large <= same);    
}

#[test]
#[should_panic]
fn test_different_currencies_ord_should_panic() {
    let dollar = Money::new(1050, currency::USD);
    let yen = Money::new(1050, currency::JPY);

    assert!(dollar > yen);
}
