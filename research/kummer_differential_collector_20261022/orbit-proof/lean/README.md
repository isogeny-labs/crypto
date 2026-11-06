# Fixed-offset orbit theorem

`FixedOffsetOrbit.lean` proves the representation-independent closure step.
If a stored offset is present and the state is closed under translation by
the negative of a fixed jump, all iterates are present.  Given an exact orbit
witness with `m` distinct iterates, the state therefore contains all `m`
distinct offsets.

The Lean theorem is conditional on `ExactOrbit`.  `../validate.py` constructs
the cyclic witness numerically for each frozen row by checking that `r` is
prime, `0 < j < r`, and `gcd(r,j)=1`, hence the orbit of subtraction by `j`
in `Z/rZ` has size `r`.  The checker also recomputes every CSV field.  Lean
does not import the CSV or certify the Python parser.

Run from the repository root:

```sh
ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/proof-gate.sh \
  research/kummer_differential_collector_20261022/orbit-proof/lean/FixedOffsetOrbit.lean
```

The theorem concerns only the explicitly stated fixed-offset, one-xADD-per-
entry state.  It does not rule out compressed, implicit, label-free, chained,
or otherwise different sidecars.
