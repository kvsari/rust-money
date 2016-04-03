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

        return format!(
            "{code}{symbol}{amount}",
            code = self.currency.code(),
            symbol = self.currency.symbol(),
            amount = formatted_amount
        );
    }
}
