
#[macro_export]
macro_rules! assert_ok {
    ( $x:expr ) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                panic!("Error calling {}: {:?}", stringify!(x), e);
            }
        }
    }
}

