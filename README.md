assert_ok
============

This crate contains a macro `assert_ok` which takes an expression and
if the expression evaluates to an `Err`, panics with a useful
message. If in contrast the expression evaluates to `Ok(v)` then it
returns the value `v`.

This is commonly useful in tests. Instead of

```
let z = foo(arg1, arg2).unwrap();
```

or

```
let z = foo(arg1, arg2).expect("foo failed");
```

use

```
let z = assert_ok!(foo(arg1, arg2));
```

It's easier to understand (IMO) and more importantly provides a much
more useful error message in the case that it fails.

There is a similar macro in Tokio, however for libraries or applications
that don't use Tokio, pulling it in for a single macro doesn't make sense.
