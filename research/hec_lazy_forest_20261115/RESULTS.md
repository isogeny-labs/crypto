# Exact lazy affine forest: correctness and retained-storage gate

Protocol1ea6e79f7e preceded implementation2e52c43c18. The lifetime amendment in
SESSION.md preceded the corrected implementation. This is a native correctness
instrument and cost derivation, not a timed complete IC implementation.

## Verified result

Both default and optimized rustc test builds pass all four tests. Two are the
unchanged dense reference's tests. The differential test covers216deterministic
streams and110592inserts: fields101/1009, H=0/1/3/19/127, N=1/7/63, four seeds,
unconstrained and planted-consistent streams (planted omitted for H0).
Every Option<Row> agrees coefficient-for-coefficient after every prefix,
as do union and pin counts. Planted emitted rows evaluate to zero.
Explicit cases cover duplicate endpoints/cancellation, arbitrary weights,
pinning, repeated pins, empty rows and shared expression cancellation.

The invariant L_u=w_u L_parent+B_u*A is preserved using immutable expression
IDs. Descending-ID expansion sums contributions before a shared child is
visited, avoiding enumeration of all paths through the expression DAG.
Only emitted rows are expanded. No new factorization or rank heuristic is used.

## Retained storage, synthetic gate only

Native jq reduction counts.jq produces COUNTS.json from GATE.txt. Ratios below
are dense bytes divided by lazy retained vector-capacity bytes over the frozen
synthetic streams, not runtime gains. Dense uses its existing bytes() estimate;
lazy includes arena capacity and all retained child/leaf vectors. Neither is
peak RSS: transient maps, allocator overhead and input ownership are excluded.

| Anchor width | Streams | Minimum ratio | Maximum ratio |
|---:|---:|---:|---:|
|0|24|0.50|0.74|
|1|48|0.13|0.42|
|3|48|0.20|0.41|
|19|48|0.73|3.42|
|127|48|4.30|11.68|

The representation loses retained storage at small H and mixed cases at H19.
It saves retained storage in every H127 stream. These synthetic streams are
often pinned early; their DAG shape is not evidence for real curve cycle costs.
The 11.68 storage ratio does not establish any end-to-end speedup.

The first implementation retained emitted-cycle scratch expressions. Its full
output remains GATE-RETAIN-ALL.txt. The correction traverses/compresses endpoints
first, then records an arena watermark. Emission expands the temporary on/hot
expressions and truncates to that watermark. Stored beta IDs precede it, so
none is invalidated. Merge/pin expressions remain retained. Old superseded
compression labels can still accumulate: no general garbage collector exists.

## Reproduction and limits

Run `sh research/hec_lazy_forest_20261115/gate.sh` with stable Rust and jq on PATH,
CARGO_TARGET_DIR set to a separate target and HEC_LAZY_GATE_OUT to a scratch dir.
It rebuilds the native library, discovers its rlib through Cargo JSON, compiles
both native test binaries, runs gates and emits counts/source hashes.
No scientific Python, timing reducer or cloud worker is involved.

Twelve native library release tests also pass. Source/library hashes are in
source.sha256. Historical closed benchmark records and defaults are untouched.
No new benchmark row or validated standing is requested for this instrument.

Next decision: JOINT-COST.md derives the established sparse-label sieve route
and its explicit yield/rank/expansion dependencies. A complete-cost protocol
must charge actual curve streams, all expansion/map work and downstream solve
before choosing lazy over dense. Existing cold ratios are not multiplied.
