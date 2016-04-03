# Money

A simple money handling library

## Installation

Add to your `Cargo.toml`

```toml
[dependencies]

money="0.1.0"
```

## Usage

```rust
extern crate money;

use money::Money;
use money::currency;

let amount_payable = Money::new(9999, currency::USD);

println!("Amount Payable: {}", money.format());

// prints "Amount Payable: USD$99.99"
```

## Status

Still very early in the development, support for most currencies is not yet
implemented, and formatting is woefully incomplete. No handling of locale
specific formatting is implemented.

In short, don't use this yet, unless you're prepared to contribute to make it
more usable.

I intend to develop it much further, so if you have suggestions or idea's
please contribute!
