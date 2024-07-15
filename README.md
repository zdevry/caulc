## caulc

This is a little personal project that I'm doing to learn Rust programming.

**Basic Usage**
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

**Percentages, Powers, and Factorials**
```
$ target/release/caulc "45 / (100-45)%"
81.81818181818181
$ target/release/caulc "4^3!"
4096
```

**Trigonometry and Logarithms**
```
$ target/release/caulc "sqrt(log(10000))"
2
$ target/release/caulc "sin(ln(3)) + tan(1/2)"
1.4368795315115377
```
