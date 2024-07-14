## caulc

This is a little personal project that I'm doing to learn Rust.
```
$ cargo build --release
$ target/release/caulc "1 + 2 * (3 + 4)"
15
$ target/release/caulc "4.56e3 + 1.45e-1 / 18.13e-2"
4560.799779371208
$ target/release/caulc "2 + (2 + )"
Error in parsing: unexpected symbol ')'
| 2 + (2 + )
|          ^
```
