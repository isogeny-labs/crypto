# Trace-zero solver validation and work reduction — 2026-11-15

The production solver now uses packed coefficient elimination, hoisted basis
squares, batched residual inversion and specialized monic quadratic roots.
On the fixed ECC2K-130 controls, whole-process simulated instructions fell
**25.03% for the planted relation** and **54.46% for the uniform target**.
All 65 prior cases retain the exact old decisions and witness order. These
are unsigned local observations, recorded in PCC-20261115-5b3bc5
[pcc:unverified].

[Two-page results report](report.pdf) · [recorded graph](instructions.svg) ·
[complete summary](summary.json) · [accepted canonical export](export.json) ·
[grid audit](grid-audit.json) · [separate arithmetic audit](independent-audit.json) ·
[protocol](PROTOCOL.md) · [production implementation](../../src/cryptanalysis/trace_zero_linearized.rs).

| Fixed d12 cell | Old median Ir | New median Ir | Instructions removed | Frozen criterion |
|---|---:|---:|---:|---|
| n131 planted |269,704,933|202,206,920|25.03%|pass|
| n131 uniform |619,422,026|282,085,987|54.46%|pass|
| n131 cancellation |182,008,866|181,985,615|0.01%|inconclusive|
| n83 planted |292,813,150|189,888,544|35.15%|pass|
| n83 uniform |561,510,012|259,437,764|53.80%|pass|
| n83 cancellation |153,463,249|153,402,769|0.04%|inconclusive|

Each cell has five repetitions per baseline, baseline A/A and candidate arm:
**90 planned, attempted and completed executions**, no invalid answer,
exclusion, interruption or retry. Four of six cells pass the predeclared
5% reduction criterion; both cancellation controls remain inconclusive.
Baseline A/A median relative differences are below 0.000001. Full five-repeat
min/median/max and exact field counts are retained [pcc:unverified].

The two quick cancellation controls finish at the first residual. Their
whole-query work barely changes; retaining them prevents reporting only the
longer searches. The new source preserves the original residual and witness
order, including those immediate answers [pcc:unverified].

## What was validated

| Requested work | Delivered evidence | Standing |
|---|---|---|
| Validate actual relation answers | Separate polynomial arithmetic checks all65 truths,46 known witnesses,416 earlier saved tuples and60 new tuples | Producer cross-check; signed independent review open |
| Preserve implementation behavior | All65 old/new decisions, original point indices/order, model diagnostics and reasons match | Scoped native development checks |
| Reduce execution work | Same-executable, cold-query simulated instruction comparison above; every native/profile answer matches | Four passing fixed cells; native-time gain unestablished |

The separate [Rust arithmetic checker](native/src/bin/check_polynomial.rs)
imports **no production field, group or solver modules**. It uses schoolbook
binary-polynomial multiplication/reduction and Euclid inversion, verifies
coordinate widths, curve equations and trace-kernel admissions, and re-adds
actual saved points. Small negative cases use complete pair tables. Every
wide negative case is checked by independently rebuilding its necessary
symmetric systems and showing all residuals inconsistent. It does not accept
a producer replay flag or timeout as negative evidence [pcc:unverified].

The checker also replays all416 actual tuples in the650 earlier scale rows;
[receipt](checks/independent-old650.json). Both an altered saved ordinate and
a false Refuted row are rejected with exit101; the exact sabotage inputs and
stderr remain in checks/. These are producer-written checks, not an
independently signed PCC verdict [pcc:unverified].

**29 focused tests pass:** four new checks, three earlier linearized checks,
twenty prior point/subspace/transport checks and two terminal checks. New
checks compare packed and generic particular solutions/kernels (including
inconsistency and fallback widths), all small-curve batched/scalar additions
(infinity, opposites and doubling included), all1,024 monic quadratics over
F32 against exhaustive roots and old root order, and exact65-case whole-solver
parity. The final production library type-check also passes. The ordinary
whole-root optimized suite is not represented as green [pcc:unverified].

## Why it removes work

For fixed residual z=x(T-R), the existing S3 equation remains

    v² + z*v + z²*u² + 1 = 0,
    u=x(P)+x(Q) in V,  v=x(P)*x(Q) in span(V·V).

This is the same known binary-linear necessary system. The changes are:

- Precompute every V/W basis square once per query.
- Use three packed machine words for field/coefficient rows through192 bits;
  keep arbitrary-width elimination for wider coefficient spaces. Full affine
  kernels, original particular solutions and conservative model caps remain.
- Compute the first residual scalarly, then share inversions across at most32
  consecutive residuals for bases of at least64 points. Exceptional additions
  use the original scalar routine. Every unused batch-tail operation is charged.
- Solve the monic quadratic directly, avoiding inversion of the known leading
  coefficient1. All actual supplied signs, S3 checks and original group replay
  remain; no public validation bypass is added.

For the n131 uniform cell, informational inversions fall from1,994 to64 and
field squares from358,139 to37,484; field multiplies are121,195 versus111,734.
Inversion internals are already counted, so informational inversions must not
be added again. The field ledger excludes parsing and does not price matrix
or allocator work; whole-process Ir includes those costs [pcc:unverified].

## Measurement boundary and identity

Each primary execution runs two fresh processes: one native operation tally,
one Callgrind run with the tally disabled. Their exact outputs must match.
Callgrind counts the complete cold executable: workload parsing, fixture and
variety construction, admission, solve, roots/sign lifts and replay. It
therefore includes Gaussian elimination, BigUint and allocator work omitted
by earlier group-only ledgers. These two process records are one paired query
execution, not independent target samples.

Ir is a **simulated instruction count**, not cycles or native runtime. The
profile explains where work goes; it does not establish wall-clock speedup.
No fastest-external result against M4RI/FLINT/Magma is established. Host is
virtual x86_64 AMD EPYC9V74, Rust1.99.1, Valgrind3.24; no hardware PMU. The
quiet local admission probe passed after an initial probe correctly rejected
this session's concurrent build. No L2 timing claim follows [pcc:unverified].

All six targets are disclosed anchors from the previous study, not new
heldouts. The degree131 field, curve, original subgroup/G and1,994-point base
are exactly those in the pinned input. E0 is y²+xy=x³+1 and the degree131
modulus is z^131+z^13+z²+z+1. The n83 modulus is z^83+z^45+z²+z+1.

- Instrument source: `902f78ccfce8ab09c42b5dd39f3e9bde103c35aa`.
- Protocol: `4643a05317e1266df27e06984a03d0d9d4a5146b5f616c7f5b855ec97228ee00`.
- Native binary: `15b6156c1e95c9220915ca7eb85883190b4c03791679f3195caab4b1cd7479ea`.
- Input: `2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5`.

[SOURCES.sha256](SOURCES.sha256) and the byte-identical
[old-source snapshot](baseline_snapshot.rs) preserve the instruments. The
post-run grid auditor and report renderer are labeled separately; they do
not amend the frozen native analysis. Earlier studies retain their historical
source pins; changing this production file does not rewrite their evidence.

The12-call largest-class pilot and native analysis smoke passed before the
freeze commit was pushed and primary data began. Its255.815-second estimate
was resource planning only. Actual pilot/freeze/primary ran within the
unchanged local CPU0/2GiB/600second cap, with zero paid launches or retries.
Input preparation and post-run audits are separate validation/setup work.

## Reproduction and custody

~~~sh
cargo test --locked --manifest-path research/trace_zero_fast_20261115/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
research/trace_zero_fast_20261115/native/target/release/trace-zero-fast analyze research/trace_zero_fast_20261115/rows.jsonl
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_fast_20261115/export.json --protocol research/trace_zero_fast_20261115/protocol.json --verify-files
python3 tools/artifacts/artifacts.py pull research/trace_zero_fast_20261115
cd research/trace_zero_fast_20261115
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
~~~

The deterministic `runs.tar.xz` contains511 exact process-output/Callgrind
files from pilot, primary and discovery profile; individual hashes are in
[raw-tree-manifest.json](raw-tree-manifest.json). Extraction, every member hash
and deterministic repacking were checked. `runs/` stays ignored. Policy-sized
raw files may be held by content-addressed `.artifacts.json` manifests after
the established same-repository artifact-autofix workflow uploads and replays
the bytes. The archive is the raw process tree; rows and accepted export remain
separate reviewable records.

The first adapter attempt omitted the explicit instance-column mapping and
source metadata; [export-initial.json](export-initial.json) and its findings
are retained. Correct mapping plus recorded metadata binding yields a canonical
export with zero findings. Raw outcomes are untouched. Packaging initially used
repository-relative paths from the study directory; corrected before publication,
verified511 members and byte-identical deterministic repacking. No trial rerun.

Both pages of this results PDF and both discovery-profile PDF pages were
visually inspected. Existing scoreboard/timeline/performance-gains graphs retain
their historical values: no admitted native-time/full-cost record is supplied.
The append-only ledger/main conflict and ordinary whole-root/benchrec dependencies
remain separate from completed component checks.

Sources checked (KB): unavailable, no loaded tool and wrapper127. Fallback:
existing binary_semaev.rs S3/Vieta/quadratic algebra and Semaev2004/031/Diem2011
references; f2m.rs half-trace and Itoh–Tsujii; Montgomery batch inversion in
koblitz_fast_arith.rs and wide_gf2m.rs. No mathematical novelty claim. Python
only binds metadata, packages bytes and renders recorded values; all curve,
solver, arithmetic validation and analysis work is native Rust.
