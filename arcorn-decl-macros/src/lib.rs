/// Macro for unwrapping arbitrary enums things without
/// having to implement a bunch of boilerplate methods.
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
