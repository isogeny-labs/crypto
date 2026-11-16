# Pre-run compiler amendment

The frozen `7b877a4c5719` Rust source did not compile: `E0689`, an
ambiguous integer type at `rem_euclid` in the outer exponent loop.
Before any research execution, the outer loop lower bound was annotated
`1_i64`; no population, formula, output or search criterion changed.
The compiler diagnostic is retained in the commit history. The next
commit freezes this compilation-only correction before the first run.
