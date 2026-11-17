# Exact Nagao unit elimination

The opt-in finder eliminates one coordinate before F4, then lifts every root
and checks all six original equations. The checked materializer accepts this
backend through `decompose_using`; the original entry point retains its solver.
On six fixed development fixtures, the observed materializer subtotal is
3471593796 versus 4800849304, ratio 0.7231207597 (27.6879% lower). Complete
solutions and cofactor/full-row geometry agree in all 24 paired records.
PCC-20261117-817df2 remains unverified. [pcc:unverified]

[Plan and literature](PLAN.md) · [Raw JSONL](validation/paired-1.jsonl) ·
[Rust recomputation](analyze.rs) · [Summary](validation/summary-1.json) ·
[Report](report.pdf) · [Session](SESSION.md)

Canonical integration record PCC-20261117-372680 reissues the three unchanged
development records after main advanced. [Integration receipt](integration.json)
and the exact [original suffix](development-ledger-fork.jsonl) retain both
histories; none of the original hash links or scientific bytes are rewritten.
All producer standing remains unverified. [pcc:unverified]

## Exact construction

Use the existing monic genus3 Nagao quotient
`Q=((x+mu)^2*H-A^2)/u`, with
`A=-(x+mu)*v+(lambda0+lambda1*x)*u`.
Comparing the degree8 numerator coefficient gives
`Q5=H6+2*mu-lambda1^2-u2`. Over an odd prime field, writing
`lambda1=a1+t*b1` and `mu=c0+t*d0`, its imaginary part gives
`d0=a1*b1+(Im(u2)-Im(H6))/2`.
The denominator is a constant unit. Substituting this into the other five
equations defines a bijection on solutions, including exceptional parameter
values: there is no varying denominator or discarded chart. Degree rises to
at most four. The implementation checks the actual pivot shape and refuses
unsupported systems rather than treating them as empty.

Scalar-preserving polynomial arithmetic is essential: the existing F4
normalizer makes polynomials monic, so it is only used after substitution on
equations. It is never used to rescale the coordinate replacement. Every lift
is canonical and replays the original equations. Existing full-row Cantor
checks and quadratic-remainder norm/Mumford checks remain in the materializer.

## Frozen development comparison

Public fixture parameters are in the preregistered plan. Each field has the
generator residual and one planted sum of six base divisors. Order is
six/unit/six/unit, with fresh RNG seed947 per call. A separate coordinate solve
and materializer call run for every record. The materializer subtotal includes
its solver, root materialization and explicit final full/partial group replay;
coordinate-only counts are separate, not added again. All 24 records complete,
both repeat tallies agree exactly, and foreign-thread hooks are zero.
[pcc:unverified]

| p | Residual | Six-variable subtotal | Unit elimination | Unit / six |
|---:|---|---:|---:|---:|
|53|generator|390320222|354082244|0.907158|
|53|planted six|393984098|357905368|0.908426|
|101|generator|516267201|480730918|0.931167|
|101|planted six|524285687|462874792|0.882867|
|1009|generator|998834924|899422406|0.900472|
|1009|planted six|1977157172|916578068|0.463584|

These are observed, incomplete primitive prices, not complete native costs.
Their sum weights each registered fixture once; it is not a descent-stream
distribution estimate. The table's cost-ratio range is 0.4636–0.9312. The largest
F4 matrix has 231 rows in the original versus 162–175 in the reduced systems.
The cohort contains three nonempty planted full decompositions and one empty
coordinate set; it contains no quadratic partial. A separate bounded
correctness test finds the first partial at `[10]G` on p101/seed409, compares
the exact cofactor geometry and replays its group identity. It is not inserted
into this cost cohort. Producer assertions only. [pcc:unverified]

During publication, main advanced to the ownership-corrected stage43 result:
26 valid partials, zero retained-class matches and no shorter descent on its
registered stream. That result strengthens the reason to price the algebraic
solver itself; this six-fixture diagnostic does not compose its counts with
stage43 or establish that stream's improvement. [pcc:unverified]

## Reproduction and boundaries

Measurement source is published commit
`e4af92fbc62ac21b6ae53d45d961de50ed60275f`.
Its native library is separately pinned to
`232925556a6e39d4589932c60e75a1bf24f9e049`, compiled unmodified with
`cargo build --offline --release --lib --locked`.
The main checkout's removed dependencies are not restored or treated as a
current-main build. `validation/build-manifest.txt` records binary/library
hashes, compiler, host and exact commands. Build with
`build-native.sh BASELINE_CHECKOUT NEW_BUILD_DIR`; the root and exact
dependency artifacts must match. Run `tests --test-threads=1`, then
`IC_SOURCE_REVISION=SOURCE RAYON_NUM_THREADS=1 timeout 600 qualify NEW_RAW`.
Run `analyze RAW NEW_SUMMARY` to recompute the comparison.

No wall ranking: these local development calls were not admitted by the
isolation harness. No paid allocation, fresh unknown-log sample, or full scalar
recovery was run. Container/control work, some inherited solver arithmetic,
serialization and PRNG internals are unpriced. Complete totals and rho-parity
fields remain null. The opt-in backend is integrated with the row materializer,
not the full production descent stream. Current source43 component ownership
and frozen stage41–43 data are unchanged.

| User requirement | Status and remaining evidence |
|---|---|
| Cheaper algebraic finder with graph-compatible rows | Implemented; fixed native diagnostic only |
| Preserve exact arithmetic and relation checks | Exhaustive lifting, native equivalence, positive partial and refusal controls |
| Cold single-target rho parity | Open: production descent integration, complete cost coverage and admitted matched full recovery |

Dashboard exclusion: this is a fixed-fixture component comparison with
incomplete costs, no IC/rho ratio and no whole-pipeline priced curve. It adds
no progress-chart, best-of or leaderboard point. The benchmark diagnostic and
study index link it explicitly. These are synthetic fixtures in the restricted
weak Fp6 family covered by the study's curve-family exception; no standard-panel
or general-curve applicability is asserted.

The original USD5 budget's remaining balance is unknown, so no paid run is
launched. The next comparative experiment needs a known remaining balance,
frozen full-pipeline integration and the maintained admission gates. This
component result does not discharge that experiment.
