# Verification and limits

The arithmetic gate completed all 21 destination models with independent
Cantor checks. It records 13,608 weighted 1+1, 24,192 weighted 1+2, 26,880
sampled weighted all-branch 2+2, 2,310 targeted weighted 2+2, 1,680 weighted
doubles, 39 special doubles, 21 degree-drop doubles, and 21,504 consecutive
weighted-state addition checks. The complete-dispatcher unchecked counters
for weighted 1+2, sampled weighted 2+2, and targeted weighted 2+2 are each
zero. The `weighted_exception_2plus2_unchecked` counter belongs to a
standalone common-formula probe: its 1,680 failed prerequisites were then
handled by the complete dispatcher and verified against Cantor. The finite
gate is not a proof for every characteristic or input.

The two full collectors each wrote seven raw files of 180 rows. Both
candidate and rho scalars were checked by source multiplication in the
producer, then independently replayed by `analyze-projective` (2,520 scalar
replays per arm). It matched each row to workload id, SHA-256, target index,
seed, cap, and planted scalar in the frozen archive. It also compared each
same-process rho arithmetic and logical counter to the frozen rho record:
1,260 exact counter replays per arm. The wrapper's 1,260 instrumented rows
match an earlier complete first pass on all non-normalization fields. The
first-pass rows are retained in `results/firstpass/`.

The separate `validate-projective` binary checked all candidate and rho
phase totals against full cold counters, recomputed all 576 arm/slice/price
rows from raw data, and verified that the phase-breakdown rows add to the
full representative cost. It wrote `results/independent-validation.json`,
`results/price-grid-state-vs-wrapper.csv`, and
`results/cold-phase-breakdown.csv`; its status is `pass`. There are zero
candidate failures, rho failures, or exhausted targets in the completed
cohort. The raw schema still supports explicit null/failure rows.

The weighted-state collector calls `to_affine` only after a projective
splitting test admits a relation candidate. Every one of these conversions
uses two field inversions and is charged: 111,187,728 conversions and
222,375,456 inversions across the cohort. Other inversions, including
construction, transfer, roots, and elimination, remain in the full counters.
The wrapper performs 440,649,754 charged normalization inversions.
The projective-state hash uses raw weighted coordinates, so its deterministic
walk differs from the wrapper despite shared workloads and seeds; the direct
ratio includes this walk difference.

The p=739 regression records the exact non-unit-W input and the outputs of
the transcribed author-code `N6` expression and the homogeneous variant.
The comparison establishes a mismatch for the transcribed code expression;
it does not establish an error in any equation omitted from the published
article. The paper and author-code references and source commit/hash are in
`MANIFEST.md`.

Crate-local verification commands: `cargo fmt --all -- --check`,
`cargo check --release --locked --bins`, `cargo test --release --locked`,
`cargo run --release --bin projective-law-gate`, the two completed collectors,
both analyzer invocations, and the independent validator. The PDF was
rendered from the native summary by `render.py` and `tectonic REPORT.tex`;
its three pages, three embedded graphs, text, and visible table were
inspected. Operation-count prices are evidence here. A wall-time ranking
would need an isolated, fastest-external-tool comparison and is not claimed.

The study-level `VALIDATION.json` now pins a formal replay certificate,
independent validator wrapper, and same-process matched negation-rho reference
with the 0/0.1/1 logical-work band for both law arms. `make_validation.py`
packages the already completed Rust analyzer and validator outputs; it checks
14 raw 180-target files, 5,040 independent source scalar replays, 2,520
archived rho-counter replays, and 576 independent price cells. The gate command
`python3 scripts/validation_gate.py evidence research/projective_law_followup_20261022`
returns `valid`. The wrapper does not create new DLP solves or a wall-time claim.
