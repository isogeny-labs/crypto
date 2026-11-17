# Terminal-domain and constant-residual S3 panel

2026-11-15, continued work authorized after the first three completed panels.
Retain the one-CPU/2GiB/600second local/no-paid-resource exception and all
previous evidence. This protocol concerns a new matched finite-panel run.

## Question, workload and arms

Can necessary target-tail domains or constant-residual two-point solving
decide the previously capped non-singleton-base queries, with all setup and
point lifting charged and an exact classical reference?

Keep all33 original instances unchanged. Add16 predeclared holdouts:
n83/131 ×d6/8 ×two distinct-x planted targets, one uniform full-width scalar
target and one cancellation control. The planted construction chooses three
different x-coordinates and a nonidentity target outside the actual base,
before any candidate execution. Fixed native seed rule:
202611150500+100*n+10*d+kind_index, pinned StdRng/Cargo.lock. Exact original
point truth comes from a separate complete pair table, never passed to a
candidate. Preparation runs no candidate solver.

Seven arms, all on the same49 instances, with two repetitions:

1. Existing constant-specialized certified-domain S3.
2. Same encoding, complete identity masks in reverse order (identity first).
3. Same encoding, finite partial sum restricted to actual x(T-B).
4. Same encoding, preserve the actual (last x, partial-sum x) pair from T-B.
5. Slice over each actual final point R and solve P+Q=T-R by constant-target
   two-point S3. No unrestricted wide partial-sum variable. Residuals remain
   in original base order, with fair remaining conflict/time shares. A capped
   residual prevents Refuted unless another slice finds a valid sum.
6. Exact repeated sliced A/A.
7. Fresh complete pair-table reference, charged per query, with an explicit
   deadline during setup/lookups. No free cached precomputation or supplied
   truth. This reference can itself be amortized in a different workload;
   such batch costs are not measured here.

49 ×7 ×2 =686 planned primary rows. Rotate all arms by case index+replicate;
keep every UNKNOWN and every failed attempt. No rows selected after looking.
All methods retain actual supplied signs and original-group replay. All
infinity cases use their explicit branches, never an affine x=0 surrogate.

## Guards and analysis

One CPU,2GiB address space,600seconds total for pilot, final freeze and primary
run. Zero retries or cloud spend. Per query:2000 cumulative SAT conflicts,
750ms safety fuse,500000 variables,1million total finite-domain clauses and
1024 models. Slices share those totals; no budget reset per final point.
Pair-table setup/lookups are charged and deadline checked. A global timeout
preserves the prefix and accounts for every unattempted row.

Engineering pilot: one call per arm at n5/7/9/83/131, target3*base[0]. All
pilot rows must be valid; every n5 arm must Found with replay before primary
freeze. Wide caps are retained, not detector successes. Pilot engineering
elapsed values are only for runtime feasibility, never timing claims.

Primary: exact valid decision counts on the fixed panel and separately on
distinct-x holdouts. Secondary: native SAT conflicts, peak allocation and
actual additions+doublings across setup, solving, lifting and replay. These
counts exclude field/ANF construction, non-group inversions and SAT work;
do not convert them or censored conflict totals into runtime ratios.
Label repeated-target spread and A/A outcome/conflict discrepancies. No
population p-values, isolated timing, scaling or full challenge recovery.
Deterministic native summary (including field/dimension/kind strata) is
hash-frozen before primary data; labels remain explicitly unmasked for this
descriptive complete-panel tabulation. No sample-size or metric adaptation.

Any invalid decision stops the panel. Complete truth and every saved point
tuple are audited in the original group after execution. No independent
signed review is implied. Canonical protocol.json and export.json bind
sources, binary, analysis, raw rows and all controls before reporting.

## Prior work and scope

The repository already documents T-minus-tail intermediate restrictions in
ChainedS3Encoding::constrain_intermediate and the classical pair-table method
in RESEARCH_KOBLITZ_INDEX_CALCULUS.md. Kosters–Yeo gives the trace/S3 context;
the prior native studies cover full-kernel membership and identity branches.
This reuses established group algebra and elimination, not a novel test.
KB tools absent; repository wrapper exits127. The implementation is for
actual finite supplied bases, not an arbitrary-algebra trace-zero chart.
