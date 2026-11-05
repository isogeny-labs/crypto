# Native sign-mask software checks

One local Rust unit-test invocation, no curve-source evaluation, performance
measurement or new recovery cohort. Three tests: all27 ternary inputs on3
coordinates with every ordered monomial length0..4; preserved477-term repaired
program on all479 square-set character inputs against direct u128 modular
multiplication and X; input/overflow/width64/constant/repeated-offset controls.
Expected X appears only in checker; Program receives coefficients and signs.
Build rustc --edition=2021 --test lib.rs; no third-party dependencies, no Cargo
workspace build. Cap build120seconds and test30seconds; no repeated run after
success. Retain source hashes, compiler version and build/test exits.
