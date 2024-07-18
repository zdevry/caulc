## caulc

This is a little personal project that I'm doing to learn Rust programming.

**Build**
```
cargo build --release
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
$ caulc '6!^3^4' round 2
2.78e231
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
$ caulc '40cm * 30cm * 55cm' in 'L'
66 L
$ caulc 'G * 5.972e24kg / (6371km)^2' round 2
9.82 m s^-2
$ caulc 'sqrt((230ohm)^2 + ( 2*pi*60Hz*0.3H - 1/(2*pi*60Hz*10uF) )^2)' in 'ohm' round 1
275.8 ohm
$ caulc 'h * c * RydH * (1/4 - 1/16)' in 'eV' round 2
2.55 eV
$ caulc '3mol * R * 320K / 100L' in 'mmHg' round 2
598.69 mmHg
```

**Minor Features**
Specifying a fixed amounts of digits after the decimal point for rounding.
Note that the app doesn't output decimal digits for answer values that it can guarantee to be an integer.
```
$ caulc '1.0 + 1.0' fixed 4
2.0000
$ caulc '1 + 1' fixed 4
2
```

Specifying when the program should use scientific notation
```
$ caulc '102.564' scientific always
1.0256e2
$ caulc '1.65e18' scientific never
1650000000000000000
$ caulc '20' scientific if over 100.0 under 10.0
20
$ caulc '150' scientific if over 100.0 under 10.0
1.5000e2
$ caulc '3' scientific if over 100.0 under 10.0
3.0000e0
```

Specifying that answers in scientific notation should always have a consistent length
```
$ caulc '267.5' round 3 scientific always fixed
+2.675e+002
$ caulc '-4^4^4' round 3 scientific always fixed
-1.341e+154
$ caulc '200!' round 3 scientific always fixed
        inf
```

Remove the units from a quantity expressed in SI units, to convert before stripping units,
use the `per` keyword which is equivalent to dividing one of the unit.
This is useful for passing values to functions that don't accept quantities with units.
```
$ caulc 'log(:45m + 55m)'
2
$ caulc '-log(23mg / 36g mol^-1 / 75mL per mol L^-1)' round 2
2.07
```
