# Signed parity assertion correction

The first run, `reports/run-20261115T0820Z/`, stopped at the first negative odd trace. Rust's signed remainder gives `-1 % 2 = -1`, so the assertion `t % 2 == 1` was a producer error, not a mathematical counterexample. No complete CSV was produced. The raw failed output remains in place.

Replace that assertion with `t & 1 == 1` (or equivalently `t % 2 != 0`). All frozen field, coefficient, curve, and trace inputs remain unchanged. Recompile and rerun into a new timestamped directory after another checkpoint push and gate.
