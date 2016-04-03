
#[derive(Debug)]
pub struct Currency<'a> {
    code: &'a str,
    divisor: i32,
    symbol: char,
}

impl<'a> Currency<'a> {
    pub fn new(code: &'a str, divisor: i32, symbol: char) -> Currency<'a> {
        Currency {
            code: code,
            divisor: divisor,
            symbol: symbol,
        }
    }

    pub fn code(&self) -> &str {
        self.code
    }

    pub fn divisor(&self) -> i32 {
        self.divisor
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
}

pub fn from_code(code: &str) -> Result<Currency, &'static str> {
    match code {
        "USD" => Ok(USD),
        "AUD" => Ok(AUD),
        "JPY" => Ok(JPY),
        _ => Err("Unknown currency"),
    }
}

pub const USD: Currency<'static> = Currency {
    code: "USD",
    divisor: 100,
    symbol: '$',
};

pub const AUD: Currency<'static> = Currency {
    code: "AUD",
    divisor: 100,
    symbol: '$',
};

pub const JPY: Currency<'static> = Currency {
    code: "JPY",
    divisor: 1,
    symbol: '¥',
};
