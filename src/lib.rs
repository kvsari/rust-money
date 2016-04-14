use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::ops::{Add, Sub, Div, Mul};

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

/// Overload the '+' operator for Money objects.
///
/// # Panics
/// Panics if the two addends are different currencies
impl<'a> Add<Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn add(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount + rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't add two different currencies.");
        }
    }
}

impl<'a, 'b> Add<&'b Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn add(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount + rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't add two different currencies.");
        }
    }
}

impl<'a, 'b> Add<Money<'a, i64>> for &'b Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn add(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount + rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't add two different currencies.");
        }
    }
}

impl<'a, 'b, 'c> Add<&'b Money<'a, i64>> for &'c Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn add(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount + rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't add two different currencies.");
        }
    }
}

/// Overload the '-' operator for Money objects.
///
/// # Panics
/// Panics if the minuend and subtrahend are different currencies
impl<'a> Sub<Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn sub(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount - rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't subtract two different currencies.");
        }
    }
}

impl<'a, 'b> Sub<&'b Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn sub(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount - rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't subtract two different currencies.");
        }
    }
}

impl<'a, 'b> Sub<Money<'a, i64>> for &'b Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn sub(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount - rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't subtract two different currencies.");
        }
    }
}

impl<'a, 'b, 'c> Sub<&'b Money<'a, i64>> for &'c Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn sub(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount - rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't subtract two different currencies.");
        }
    }
}

/// Overload the '/' operator for Money objects.
///
/// # Panics
/// Panics if the dividend and divisor are different currencies
impl<'a> Div<Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn div(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount / rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't divide two different currencies.");
        }
    }
}

impl<'a, 'b> Div<&'b Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn div(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount / rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't divide two different currencies.");
        }
    }
}

impl<'a, 'b> Div<Money<'a, i64>> for &'b Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn div(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount / rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't divide two different currencies.");
        }
    }
}

impl<'a, 'b, 'c> Div<&'b Money<'a, i64>> for &'c Money<'a, i64> {
    type Output = Money<'a, i64>;

    fn div(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount / rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't divide two different currencies.");
        }
    }
}

/// Overload the '*' operator for Money objects.
///
/// # Panics
/// Panics if the multiplicand and multiplier are different currencies
impl<'a> Mul<Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;
    
    fn mul(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount * rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't multiply two different currencies.");
        }
    }
}

             

impl<'a, 'b> Mul<&'b Money<'a, i64>> for Money<'a, i64> {
    type Output = Money<'a, i64>;
    
    fn mul(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount * rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't multiply two different currencies.");
        }
    }
}
             
             
impl<'a, 'b> Mul<Money<'a, i64>> for &'b Money<'a, i64> {
    type Output = Money<'a, i64>;
    
    fn mul(self, rhs: Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount * rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't multiply two different currencies.");
        }
    }
}                                                    

impl<'a, 'b, 'c> Mul<&'b Money<'a, i64>> for &'c Money<'a, i64> {
    type Output = Money<'a, i64>;
    
    fn mul(self, rhs: &Money<'a, i64>) -> Money<'a, i64> {
        if self.currency == rhs.currency {
            Money {
                amount: self.amount * rhs.amount,
                currency: self.currency,
            }
        } else {
            panic!("Can't multiply two different currencies.");
        }
    }
}
