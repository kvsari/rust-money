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

#[test]
fn test_add_monies() {
    let bag = Money::new(2995, currency::USD);
    let hat = Money::new(9995, currency::USD);
    let total = Money::new(12990, currency::USD);
    assert!((bag + hat) == total);

    let shirt = Money::new(1000, currency::AUD);
    let pants = Money::new(2000, currency::AUD);
    let clothes = Money::new(3000, currency::AUD);
    assert!((shirt + &pants) == clothes);

    let ram = Money::new(20000, currency::USD);
    let cpu = Money::new(10000, currency::USD);
    let pc = Money::new(30000, currency::USD);
    assert!((&ram + cpu) == pc);

    let eraser = Money::new(100, currency::JPY);
    let pencil = Money::new(100, currency::JPY);
    let stationery = Money::new(200, currency::JPY);
    assert!((&eraser + &pencil) == stationery);
}

#[test]
#[should_panic]
fn test_different_currencies_add_monies_should_panic() {
    let bag = Money::new(2995, currency::USD);
    let hat = Money::new(9995, currency::AUD);
    let total = Money::new(12990, currency::USD);

    assert!((bag + hat) == total);
}

#[test]
fn test_sub_monies() {
    let account = Money::new(200, currency::USD);
    let spend = Money::new(100, currency::USD);
    let balance = Money::new(100, currency::USD);
    assert!((account - spend) == balance);
    
    let money = Money::new(3000, currency::AUD);
    let spend = Money::new(1000, currency::AUD);
    let left = Money::new(2000, currency::AUD);
    assert!((money - &spend) == left);

    let cash = Money::new(50, currency::USD);
    let bill = Money::new(50, currency::USD);
    let broke = Money::new(0, currency::USD);
    assert!((&cash - bill) == broke);

    let bank = Money::new(100, currency::JPY);
    let borrow = Money::new(1000, currency::JPY);
    let debt = Money::new(900, currency::JPY);
    assert!((&borrow - &bank) == debt);

}

#[test]
#[should_panic]
fn test_different_currencies_sub_monies_should_panic() {
    let account = Money::new(200, currency::USD);
    let spend = Money::new(100, currency::AUD);
    let balance = Money::new(100, currency::USD);
    assert!((account - spend) == balance);
}

#[test]
fn test_div_monies() {
    let x = Money::new(200, currency::USD);
    let y = Money::new(100, currency::USD);
    let z = Money::new(2, currency::USD);
    assert!((x / y) == z);

    let a = Money::new(200, currency::USD);
    let b = Money::new(100, currency::USD);
    let c = Money::new(2, currency::USD);
    assert!((a / &b) == c);

    let d = Money::new(200, currency::USD);
    let e = Money::new(100, currency::USD);
    let f = Money::new(2, currency::USD);
    assert!((&d / e) == f);

    let g = Money::new(200, currency::USD);
    let h = Money::new(100, currency::USD);
    let i = Money::new(2, currency::USD);
    assert!((&g / &h) == i);
}

#[test]
#[should_panic]
fn test_different_currencies_div_should_panic() {
    let x = Money::new(200, currency::USD);
    let y = Money::new(100, currency::AUD);
    let z = Money::new(2, currency::USD);
    assert!((x / y) == z);
}

#[test]
fn test_mul_monies() {
    let x = Money::new(200, currency::USD);
    let y = Money::new(2, currency::USD);
    let z = Money::new(400, currency::USD);
    assert!((x * y) == z);
}

#[test]
#[should_panic]
fn test_different_currencies_mul_should_panic() {
    let x = Money::new(200, currency::USD);
    let y = Money::new(2, currency::AUD);
    let z = Money::new(400, currency::USD);
    assert!((x * y) == z);
}
