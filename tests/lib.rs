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
