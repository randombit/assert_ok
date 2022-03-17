//! A macro that asserts that a [`Result`] is [`Ok`]

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

/// Assert that a [`Result`] is [`Ok`]
///
/// If the provided expresion evaulates to [`Ok`], then the
/// macro returns the value contained within the [`Ok`]. If
/// the [`Result`] is an [`Err`] then the macro will [`panic`]
/// with a message that includes the expression and the error.
#[macro_export]
macro_rules! assert_ok {
    ( $x:expr ) => {
        match $x {
            std::result::Result::Ok(v) => v,
            std::result::Result::Err(e) => {
                panic!("Error calling {}: {:?}", stringify!(x), e);
            }
        }
    };
}
