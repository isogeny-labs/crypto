# Fixed-offset Kummer sidecar orbit bounds

Study ID `KUMMER-FIXED-OFFSET-ORBIT-20261024`. This proof-and-derivation run
uses 21 frozen prime-order groups and 16 canonical nonzero jumps per group.
The standardized [`bounds.csv`](results/bounds.csv) contains all 336 rows. The
[graph](orbit-bounds.svg) and [PDF](REPORT.pdf) aggregate the 16 equal-orbit
jumps into one point for each of the 21 models.

| Recorded quantity | Result |
| --- | ---: |
| Frozen group models | 21 |
| Jumps per model | 16 |
| Bound rows | 336 |
| Standard price rows | 12,096 |
| Rows with `gcd(r,j)=1` and exact orbit `r` | 336 / 336 |
| Minimum logical offsets, smallest model | 34,147 |
| Minimum logical offsets, largest model | 552,130,504,327 |
| Explicit state in current `[u64;4]` representation | 1,092,704–17,668,176,138,464 bytes |
| Regular cached-xADD multiplication bound | 819,528–13,251,132,103,848 M-equivalent |
| Native validation failures | 0 |

The formal statement is conditional and narrow. If a nonempty state stores
`K(T+s)` for each offset `s` in a finite set, and updating every entry after
one fixed jump `J` by a direct differential xADD requires the predecessor
offset, closure includes the entire translation orbit. The
[`FixedOffsetOrbit.lean`](lean/FixedOffsetOrbit.lean) theorem proves that
closure step for any exact orbit witness. The native validator separately
checks the frozen cyclic witnesses: every `r` is prime, every jump scalar is
nonzero, and `r/gcd(r,j)=r`.

All 16 jumps for a given model yield the same bound because a nonzero element
of a prime-order cyclic group has order `r`. The graph shows one aggregate
point per model and retains all 336 rows in the CSV. The price grid is also
constant across its canonical 36 cells: the regular cached xADD contributes 24
field multiplications and 36 field additions, while only the multiplications
have a coefficient in this grid. The cells use `S/M in {0,0.5,1}`, `I/M in
{1,10,100,1000}`, and logical GAE in `{0,0.1,1}`.

The storage column applies to the current explicit Rust `[u64;4]` class. It
does not bound compressed or implicit encodings. The operation column applies
to one regular cached-theta xADD per stored entry and excludes exceptional
repair. It is not a full collector cost and is not compared to rho.

This result does not construct or reject a project-pseudomultiply-recover
chain, label-free recurrence, variable-offset sidecar, or different walk.
The canonical jumps are target-independent orbit witnesses rather than the
per-target arbitrary-jump tables. Those routes remain open.

## Reproduction and checks

From the repository root:

```sh
python3 research/kummer_differential_collector_20261022/orbit-proof/generate.py
python3 research/kummer_differential_collector_20261022/orbit-proof/validate.py
python3 research/kummer_differential_collector_20261022/orbit-proof/plot.py
ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/proof-gate.sh \
  research/kummer_differential_collector_20261022/orbit-proof/lean/FixedOffsetOrbit.lean
```

The input digest is
`b57fbb4c3be313842aa2d3024d1b3d221e8b380caf8fbb4c219c4222f55fe41a`.
[`validation.json`](results/validation.json) records the independently
recomputed row hashes and populations. [`SHA256SUMS`](SHA256SUMS) pins the
review packet.
