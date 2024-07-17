## caulc

This is a little personal project that I'm doing to learn Rust programming.

**Build**
```
$ cargo build --release
```
In the following sections, `caulc` refers to the binary built by the command above.

**Basic Usage**
```
$ caulc '1 + 2 * (3 + 4)'
15
$ caulc '4.56e3 + 1.45e-1 / 18.13e-2' round 4
4560.7998
$ caulc '2 + (2 + )'
Error in parsing: unexpected symbol ')'
| 2 + (2 + )
|          ^
```

**Percentages, Powers, and Factorials**
```
$ caulc '45 / (100-45)%' round 3
81.818
$ caulc '4^3!'
4096
$ caulc '6!^3^4' round 2 scientific always fixed
+2.78e+231
```

**Trigonometry and Logarithms**
```
$ caulc 'sqrt(log(10000))'
2
$ caulc 'sin(30 deg) + tan(45 deg)'
1.5
```

**Units and Constants**
```
$ caulc '10m + 3m'
13 m
$ caulc '40cm * 30cm * 55cm' in 'L'
66 L
$ caulc 'G * 5.972e24kg / (6371km)^2' fixed 3
9.820 m s^-2
$ caulc '34.5 mmol L^-1 * 92.3 mL * 65 g mol^-1' in 'g' round 3
0.207 g
```
