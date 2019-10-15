// result1.rs
// Make this test pass! Scroll down for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative=-10,
    Zero=0,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value > 0 {
            true => Ok(PositiveNonzeroInteger(value as u64)),
            false => Err(if value < 0 {CreationError::Negative} else {CreationError::Zero})
        }
        
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
















// `PositiveNonzeroInteger::new` is always creating a new instance and returning an `Ok` result.
// It should be doing some checking, returning an `Err` result if those checks fail, and only
// returning an `Ok` result if those checks determine that everything is... okay :)
