use assert_ok::assert_ok;

#[test]
fn test_it_accepts_ok() {
    let _z : u32 = assert_ok!("42".parse());
}

#[test]
#[should_panic]
fn test_it_panics_on_err() {
    let _z : u32 = assert_ok!("nope".parse());
}

