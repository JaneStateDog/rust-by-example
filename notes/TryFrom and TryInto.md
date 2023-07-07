---
aliases:
 - TryFrom
 - TryInto
---
Similar to [[From and Into#From|From]] and [[From and Into#Into|Into]], [[TryFrom and TryInto|TryFrom]] and [[TryFrom and TryInto|TryInto]] are generic traits for converting between types. Unlike [[From and Into#From|From]]/[[From and Into#Into|Into]]], the[[TryFrom and TryInto|TryFrom]]/[[TryFrom and TryInto|TryInto]] traits are used for fallible conversions, and as such, return [[Result]]s.
```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```