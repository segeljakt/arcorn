//! Macros for unwrapping arbitrary enums things without
//! having to implement a bunch of boilerplate methods.
//! Assumes that each enum variant has exactly one field.

#[macro_export]
macro_rules! unwrap {
    {
        $expr:expr , $variant:path
    } => {
        if let $variant(v) = $expr {
            v
        } else {
            unreachable!()
        }
    }
}

#[macro_export]
macro_rules! is {
    {
        $expr:expr , $variant:path
    } => {
        if let $variant(_) = &$expr {
            true
        } else {
            false
        }
    }
}
