use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

pub mod currency;

#[derive(Debug)]
pub struct Money<'a, T> {
    amount: T,
    currency: currency::Currency<'a>,
}

impl<'a> Money<'a, i64> {
    pub fn new(amount: i64, currency: currency::Currency) -> Money<i64> {
        Money {
            amount: amount,
            currency: currency,
        }
    }

    pub fn format(&self) -> String {
        let formatted_amount = self.amount as f64 / self.currency.divisor() as f64;

        return format!("{code}{symbol}{amount}",
                       code = self.currency.code(),
                       symbol = self.currency.symbol(),
                       amount = formatted_amount);
    }
}

impl<'a> PartialEq for Money<'a, i64> {
    fn eq(&self, rhs: &Money<'a, i64>) -> bool {
        self.amount == rhs.amount && self.currency == rhs.currency
    }

    fn ne(&self, rhs: &Money<'a, i64>) -> bool {
        self.amount != rhs.amount || self.currency != rhs.currency
    }
}

/// Overload operators < > <= => for Money object
///
/// #Panics
/// Panics if the currencies are different
impl<'a> PartialOrd for Money<'a, i64> {
    fn partial_cmp(&self, rhs: &Money<'a, i64>) -> Option<Ordering> {
        if self.currency == rhs.currency {
            if self < rhs {
                Some(Ordering::Less)
            } else if self == rhs {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            None
        }
    }

    fn lt(&self, rhs: &Money<'a, i64>) -> bool {
        if self.currency == rhs.currency {
            self.amount < rhs.amount
        }
        else {
            panic!("Can't compare different currencies.");
        }
    }

    fn le(&self, rhs: &Money<'a, i64>) -> bool {
        self < rhs || self == rhs
    }
    
    fn gt(&self, rhs: &Money<'a, i64>) -> bool {
        if self.currency == rhs.currency {
            self.amount > rhs.amount
        }
        else {
            panic!("Can't compare different currencies.");
        }
    }
    
    fn ge(&self, rhs: &Money<'a, i64>) -> bool {
        self > rhs || self == rhs
    }
}
