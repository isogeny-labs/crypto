# M31 graph comparison and matrix-free projection

This is an additive development component of PR #5120. It reuses the existing
Rust graph engines and fixed-curve arithmetic, and adds an isolated comparison
runner plus an exact sparse projection tape. It does not modify the collector,
the existing graph lane #5076, its frozen captures, or the production filters.

## Fixed mathematical identity

The accepted coordinate field is **p = 2147483647**. The accepted curve is the
existing `prime31_algebraic_ic::PRIME31`:

| Item | Value |
|---|---|
| Curve | y² = x³ + 847514846 x + 2087444724 |
| Generator | (625037715, 2100627251) |
| Prime subgroup order / relation field | ell = 2147446831 |
| Protected target column | 4294967295 (`u32::MAX`) |

Relation reduction uses **F_ell**, not F_p. The separate M31 curve
y² = x³ - 3x + 21 has a different identity and is rejected here. A shared
coordinate prime is never sufficient to combine captures.

## Original code and source binding

The `cryptanalysis/` relative symlinks point to unchanged repository modules:
`large_prime_filter.rs`, `large_prime_filter_mst.rs`, `large_prime_graph.rs`,
their children and the rank-bound module. The native arithmetic is a Cargo path
dependency on `research/prime31_ic_20261115/native`. `build.rs` checks the exact
symlink targets and the SHA-256 manifest of the imported source closure. A changed
import fails the build until its manifest is explicitly reviewed and updated.
The committed Cargo lock pins dependency resolution, including libc 0.2.190.

The source manifest binds the reused source closure. The fresh-process runner
also hashes its actual executable, binding this crate's compiled source and its
dependencies. These hashes establish byte identities; they do not constitute
an independent signed scientific verdict. The small crate does not qualify the
entire production root crate. Original operation hooks are counted atomically;
forest routines retain their separate original witness counters. Those counters
cover different work and must not be combined as if they were one complete cost.

## Input contract and admission

Each line of `rows.jsonl` contains exactly `id`, `small`, `large` and `rhs`, using
the repository's `PartialRelation` serialization. Rows must have stable unique
IDs, sorted distinct columns, canonical nonzero coefficients, canonical RHS,
and at most two LP columns. The harness rejects malformed data rather than
silently normalizing it. It hashes the exact complete JSONL bytes, preserving
row order and signs.

A separate manifest has this schema:

```json
{
  "schema": "m31-graph-input/v1",
  "coordinate_prime": 2147483647,
  "relation_prime": 2147446831,
  "curve_a": 847514846,
  "curve_b": 2087444724,
  "generator": [625037715, 2100627251],
  "target": [123, 456],
  "target_column": 4294967295,
  "input_sha256": "SHA256_OF_EXACT_JSONL_BYTES",
  "small_points": [[4294967295, [123, 456]]],
  "large_points": [],
  "provenance_label": "description of the actual source capture"
}
```

The sample target is a placeholder, not a valid point. Supply actual canonical
curve points for every used column, including the protected target map equal
to Q. Unknown manifest fields, duplicate column/point identities, missing maps,
curve or subgroup changes, input-hash changes, and failed group equations are
rejected before output. The existing Rust `check_curve` and `Arithmetic` verify
the curve/subgroup certificate, every mapped point, and every equation

    sum(small coefficients × mapped points)
      + sum(LP coefficients × mapped points) = rhs × G.

An `Admitted` object has no public mutable fields or constructor; its read-only
accessors expose only admitted data. The CLI's provenance label is descriptive.
Group-equation validity does not attest how the collector produced the stream,
whether rejected attempts were retained, or how much collection cost occurred.

Admission caps are 4096 rows, 8192 mapped points, one million input terms, and
64 MiB per input/manifest file. Every child admits the whole hash-bound capture
before selecting its requested original-order prefix. Admission time and counts
remain separate from graph processing. The prefix digest is explicitly a
**canonical-prefix digest** of the serialized selected relation array; the full
raw JSONL digest remains separately recorded.

## Four-arm fresh-process comparison

```sh
cargo test --locked --manifest-path research/m31_integrated_build_20261123/graph/Cargo.toml --test graph_suite -- --test-threads=1
cargo build --locked --release --manifest-path research/m31_integrated_build_20261123/graph/Cargo.toml
```

Run the resulting `m31-graph-compare` executable:

```sh
m31-graph-compare compare \
  --input rows.jsonl --manifest capture.json \
  --prefixes 256,512,613 --rounds 5 --out-dir NEW_OUTPUT_DIRECTORY
```

The command runs **eager forest, lazy forest, pivot, and MST** on precisely the
same admitted prefixes. Each arm/round uses a fresh child process. Arm order
rotates deterministically between rounds. The coordinator enforces at most
three prefixes, five rounds, 60 children, and one active child. Linux limits are
60 seconds wall time, 60 seconds CPU time, and 4 GiB address space per child;
the address-space cap also bounds resident memory. Threads are fixed to one.
This bounded driver is ready for an admitted scientific panel, but implementing
or testing it does not itself authorize a new collection or performance job.

The coordinator's preflight performs only byte-capped input loading, hashes and
row/shape parsing. Full curve/subgroup/group-equation admission runs exclusively
inside the capped children. `coordinator_preflight_ns` records setup explicitly,
and `total_process_wall_ns` starts at coordinator entry and includes that setup,
the replay loop and final custody checks. Per-child wall clocks stop when the
child exits, before parent result validation. The earlier 05 development receipt
retains the initial implementation's omitted coordinator-admission boundary;
the targeted 06 gate checks this correction, including invalid-identity
rejections retained from all four capped children. No panel-cost claim is made
from either planted development control.

The output directory must be new. It retains each child's stdout JSON, stderr,
exit/timeout, wall time and final summary. Records include process peak RSS,
user/system CPU, input/admission/component intervals, original filter counters,
LP rank, exact projected useful and augmented rank, residual nonzeros, source
terms, source replay, and target outcome. The exact-rank audit and scalar-recovery
audit are charged separately. Pivot/MST extraction includes their original
projection/materialization work; the forest records those stages separately.
Unknown or unmeasured quantities remain null.

Source replay checks every returned witness against the admitted original
relations. The protected target is the largest small-column ID; an ascending
echelon pivot there yields an isolated target equation. Recovered scalars must
satisfy `[scalar]G = Q` using the original curve arithmetic. Lack of a target
pivot reports no recovery, even when other relations have independent rank.

MST/forest atomic refusal yields no rank or target claim. The original pivot
filter may preserve some rows after refusing a fill operation; such output is
explicitly `partial-output`, with valid retained rows and diagnostics but no
assertion of full projection. No fallback or truncated output is silently
counted as a complete arm.

The coordinator validates per-child input, manifest, executable, prefix, source
manifest and limit identities. It requires all four complete arms to agree on
LP/useful/augmented ranks and target outcome, and the eager/lazy forests to agree
on exact output witnesses. Disagreement, refusal, timeout or identity mutation
preserves the files and returns an incomplete/unequal verdict. Other arms may
emit different bases with the same row space; their output hashes need not agree.

`run --input ... --manifest ... --prefix N --arm eager|lazy|pivot|mst` is the
single-worker entry point. **The coordinator applies the process envelope**;
direct worker invocation should be placed under equivalent external limits.

## Matrix-free operator API

`ProjectionTape::build(rows, limits)` builds a sparse LP elimination program in
original row order. Let B be its independent LP source rows and C its pivot
columns. D = L[B,C] is invertible, even if the full L is rank deficient. For
remaining row indices R, the operator is exactly

    H = S_R - L[R,C] D^-1 S_B,
    h = b_R - L[R,C] D^-1 b_B.

The builder records references to earlier normalized pivot nodes plus their
coefficients and normalization factors. It discards the LP echelon working
maps after construction. It never expands an original-source cancellation
witness or the projected small matrix H.

The public methods are:

- `columns()` and `stats()`: immutable column identities and preparation counters.
- `apply(x)`: Hx in the dependent-row order.
- `apply_transpose(y)`: Hᵀy by reverse accumulation through the same tape.
- `projected_rhs()`: h using the same exact projection.

Vectors must have the exact dimension and canonical F_ell entries. Caps bound
tape terms, working/pivot nonzeros and preparation multiply-add terms; failed
construction returns an error and no partial operator. Nonunit gains, singleton
anchors, LP-free rows, rank-deficient L and high target-column IDs are supported.
The optional worker arm `--arm matrix-free` records preparation and one forward /
transpose probe with the bilinear identity checked. Its status is `operator-only`;
it reports no useful-rank, target-recovery or complete-solver result. It is not
silently added to the four fully materialized comparator arms.

### Cost and what remains to integrate

Let m be the source-row count, n the small-column count, Z the original small
nonzeros, and T the stored tape-dependency count. After preparation, each forward
or transpose product costs O(Z + T + m + n) field/vector work. Stored operator
data costs O(Z + T + m + n). Each product allocates O(m+n) scratch vectors. These
are operation/storage bounds for the tape representation, not measured runtime comparisons.

Preparation uses ordered-map sparse LP elimination. For this accepted arity≤2
domain, a pivot row and the current LP working row each have at most two terms:
eliminating one pivot coefficient removes one term and introduces at most one.
Thus the retained LP pivots have at most 2r terms, where r = rank(L). The
dependency tape T can nevertheless be quadratic in m, because later rows can
traverse many earlier pivots. With ordered lookup of r pivots and compression
of n small-column IDs, preparation costs
O(m log(m+2) + (m+T) log(r+2) + Z log(n+2)) map / field work, plus r inversions.
The first term includes unique-source-ID validation. No near-linear bound on T is claimed. Preparation
is charged in full and can dominate a small number of products. The separately
run small-payload rank/recovery audits can have dense fill and remain subject
to the child-process limits.

A complete Krylov solver must supply its own termination/certificate logic,
rank/target-identifiability qualification, and literal scalar point replay before
the operator can be compared as a complete solving method. The four explicit
arms remain useful correctness and cost controls. No extraction, setup, or
collector cost is manufactured from this retained-stream component replay;
`complete_cold_solving_cost` remains null.

## Development validation and prior sources

`tests/graph_suite.rs` contains independent dense block elimination of the raw
`[L|S|b]` system, row-space comparisons, forward/transpose checks, 128 seeded
systems over the actual M31 subgroup-order field, fixed weighted/rank-deficient /
anchored/LP-free cases, admission corruption and cap refusals, original four-arm
target recovery, an underdetermined prefix, and a four-fresh-process CLI control.
The group-valid fixtures are explicitly planted development controls. They are
not represented as authentic collector captures or speed measurements.

Exact execution commands, failures, terminal test results and retained CLI
records are under `evidence/`. Test status must be read from the terminal
receipt, not inferred from this list of authored tests.

Prior source fallback: `RAW_PROJECTION_RANK_IDENTITY_20261123.md`,
`LAZY_INDEPENDENT_ADMISSION_20261123.md`,
`FOREST_SAME_STREAM_EVALUATOR_20261123.md`, original `witness.rs`,
`projected_rank.rs`, and the retained M31 admission handoff in the existing graph
study. The research-KB service/wrapper was unavailable in this runtime; no novelty
or global absence claim is made. The parent reviewed the original Semaev and
double-large-prime literature. This implementation is ordinary exact sparse
elimination represented as a reversible linear computation, not a new complexity
claim for ECDLP.
