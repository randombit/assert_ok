use assert_ok::assert_ok;

#[test]
fn test_it_accepts_ok() {
    let _z: u32 = assert_ok!("42".parse());
}

#[test]
#[should_panic]
fn test_it_panics_on_err() {
    let _z: u32 = assert_ok!("nope".parse());
}

#[test]
#[should_panic(expected = "Error calling \"nope\".parse(): ParseIntError { kind: InvalidDigit }")]
fn test_it_panics_on_err_with_correct_message() {
    let _z: u32 = assert_ok!("nope".parse());
}