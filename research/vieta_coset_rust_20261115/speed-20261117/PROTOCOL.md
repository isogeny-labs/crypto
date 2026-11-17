# Fresh complete-cost timing comparison

Calibration protocol72 calls is frozen separately from primary504 calls.
The primary plan has6 configurations,6 arms,2 excluded warmups and12 retained
targets per configuration (6 per population). Calibration completed72/72 calls
and its native forecast filled only prospective runtime metadata. Main JSON
was frozen04:38:20UTC with canonical hashf88b57e6b52d..., published before
its first affected call around04:56UTC. Matrix/N/limits/analysis did not change.
The original pre-calibration main draft is preserved separately. No old run
is resumed or pooled.

Configurations: coset word62/m8192,16384; coset P224/m2048; rational word62/m2048;
rational P224/m128,512. Word62 is the existing4179340454199820289/A2/B3 field
component case, with no prime-group-order assertion. P224 uses the standard
NIST curve. Rational fixtures are certified Mobius-conjugated squaring fibers,
not measurements of auxiliary-isogeny search.

Every method freshly constructs the full descriptor/certified fibers and full
point factor base. Lookup additionally builds its hash index. The full cold
clock starts before construction and ends after the query; the query clock
starts after construction. Setup objects stay alive through both clocks.
Process startup, setup teardown, external fixture creation, target draws,
scalar verification, serialization and hashing of answer evidence are excluded
from algorithm clocks. Process/preparation time is retained for runtime planning.

Schoolbook is the reference. Karatsuba/Newton with classical Euclid isolates
the GCD change against recursive HalfGcd. HalfGcd A/A uses a fresh exec-created
process, supplying both nuisance-noise and negative-null observations (not two
independent panels). A double-query arm must reproduce the exact answer and
2x query multiplication count. The timing positive requires median query ratio
greater than1.25 and outside A/A noise. A miss prevents timing interpretation;
it never deletes the rows or invalidates unchanged exact counts.

The shared counted Rust engine is measured as implemented, including its
counter overhead. One qualified immutable binary/thread is used for all arms.
Williams6 order balances every position within each retained population.
Targets alternate seeded random abscissae/canonical sqrt and constructed sums;
no uniform curve-point claim. Identity targets remain separate. Matching uses
physical field/curve, representation, size and target index, never aliases alone.

Per stage:60s/4e9 primitive multiplications,65536 split attempts. Worker cap135s;
configuration cap400s calibration/1400s primary; outer1200s/7200s. No retries.
Keep every failure, timeout, missing remainder and excluded warmup. Exact-set
mismatch or a first positive-control failure stops the instrument. Ordinary
budget failures retain later arms/configurations and preclude complete-cell
speed assertions when counterparts are missing/censored.

Native analysis is frozen before calls: min/median/max/all values, paired
ratios and4096-replicate percentile-bootstrap medians conditional on these
targets. Small-N intervals are descriptive; no p-values, population-significance,
exponent or fastest-backend assertion. Analysis is explicitly unblinded and
deterministic, with no posthoc workload selection/dispatch. Frozen controls,
raw observations and prespecified analysis remain visible for external review.

Execution is explicitly isolated local/Git-only with persistent bounded
supervision. Official isolation records the pinned CPU, foreign activity and
actual blockers. Missing PSI/unmovable host threads keep clocks belowL2 and
wall_valid=false; local elapsed ratios are diagnostics, not admitted L2 speed
claims. Counts remain primary. No paid/cloud experiment or local S3 operation;
prospective object addresses are not availability claims. Git sources/configs,
protocols, compiler/build receipts, hashes and raw checkpoints are published.

The prior degree argument and source/protocol/evidence pins are preserved.
The new Lean normal-form bridge is conditional on explicit NF contracts; it
does not silently certify the full tower instantiation or independent signed
standing. Sources and proof-instance obligations are in PLAN.md/PROOF_MAP.md.
