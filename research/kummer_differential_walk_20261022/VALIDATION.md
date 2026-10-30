# Reproduction

From the repository root, with Rust dependencies cached:

```sh
cd research/kummer_differential_walk_20261022
TMPDIR=/var/tmp cargo build --release --locked --offline
cd ../..
research/kummer_differential_walk_20261022/target/release/kummer-differential-walk
python3 research/kummer_differential_walk_20261022/validate.py
MPLCONFIGDIR=/var/tmp/kummer-mpl python3 research/kummer_differential_walk_20261022/plot.py
```

The native producer compares every quotient pair with a distinct Jacobian
computation and every label with a separately multiplied source combination.
The Python validator independently checks row joins, projective equality,
the recurrence, and counts. The preliminary Legendre-product raw rows are
frozen diagnostic input and are not overwritten by the production command.
