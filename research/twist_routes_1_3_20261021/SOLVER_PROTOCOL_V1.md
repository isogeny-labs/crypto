# E07–E08 solver execution, version 1

Run the exact coordinate-box H=8 base, first B=8 distinct signed columns, on
all ten prime-order fixtures. Use arities 2,3,4 and seeds 0–7 (0–3 design,
4–7 holdout); choose public targets by seeded uniform x rejection and an
independent sign bit. Freeze code, fixture/base hashes and this protocol before
generating the cells. The broader B=12,16 and alternative-base solver panels
remain separate requirements; this initial execution does not claim them.

E07 independently enumerates polynomial x-coordinate sums and full elliptic
group sums for every ordered base tuple, checks equality after canonicalizing
the first pair, and retains identity-prefix and diagonal branches. Build
product membership, triangular product-chain membership and Boolean-index
membership with exact range checks. The fourth solver encoding eliminates the
first pair using its sum/product: the two remainders of the squarefree base
polynomial enforce distinct first-pair roots. Its diagonal branch is enumerated
separately. This is ordinary rational-field encoding symmetry, not extra
independent relations from the twist.

E08 runs all four encodings under grevlex and lex. Native F4 is the existing
repository source, with degree bound 24, a 500 ms cooperative budget and
one-second hard child limit. msolve 0.7.5 runs both q=0 and q=1 at one thread
under the same one-second hard limit; Singular 4.4.1 independently checks
dimension and ideal equality under that hard limit. Every child has a 768 MiB
address-space limit. At most three cells run concurrently. These L0 runs admit
no wall-time comparison. Different internal stopping behavior stays visible.

For every native basis, retain all expected affine assignments; no rejected
genuine assignment is acceptable. Compare the independent Singular dimension
with the exact affine assignment census, and reduce each available native and
msolve ideal against the independently computed ideal in both directions.
Missing output, nonzero exit, timeout or incomplete basis remains incomplete.
Retain basis shapes and native row-reduction multiplication counts; these are
kernel counts, not complete solver/pipeline prices. Do not compare these with
uncounted external work as a speed ratio. Magma is unavailable and no
fastest-external claim is made. Duplicate the first product/grevlex m=2 case on
each fixture as an A/A control.

The input cells and all failures go in a deterministic raw-case archive after
the run; summaries, extraction instructions and hashes stay beside it. A false
projected-set or dimension check blocks performance interpretation. Stop the
panel after 90 minutes, retaining completed and incomplete cells. No NIST-size
solver run, asymptotic result, full-cost advantage or independent-host replay
is established by this diagnostic.

```sh
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments solver research/twist_routes_1_3_20261021/execution-v1/fixtures.json research/twist_routes_1_3_20261021/execution-v1/runs/trace-20261021T2256Z/E02-E04-bases.json FRESH_SOLVER_DIRECTORY
```
