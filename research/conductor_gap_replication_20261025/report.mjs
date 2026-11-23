// Render already-computed native summaries and provenance. No new estimands.
import {readFileSync,writeFileSync,readdirSync} from 'node:fs';
import assert from 'node:assert/strict';
const root='research/conductor_gap_replication_20261025';
const load=p=>JSON.parse(readFileSync(root+'/'+p,'utf8'));
const a=load('analysis/panel-A.json'),b=load('analysis/panel-B.json');
const independent=load('validation/independent.json'),geometry=load('validation/geometry.json');
const identity=load('analysis/recurrence-factorization.json');
const modes=load('analysis/b-mode-invariants.json');
assert(a.complete&&b.complete);
const f=x=>typeof x==='number'?x.toFixed(5):String(x);
const ci=x=>'['+x.map(f).join(', ')+']';
let text=`# Conductor-gap replication: results and limits

Date: 2026-10-25. Follow-up to [PR #2430](https://github.com/aburan28/crypto/pull/2430).
Classification: bounded controlled measurement, with exact arithmetic and
accounting checks. Every scientific verdict below uses the frozen design;
wall time is descriptive L0 only.

**Assessment.** This is stronger bounded evidence for similar counted-rho
cost across the gap. All five levels pass the nominal +/-5% rule; four
pass the simultaneous correction, while the crater remains inconclusive.
The CM construction comparisons change setup work and preserve the
non-base solver behavior. A general endomorphism-dependent hardness
conclusion is still unestablished.

## Balanced rho panel

| Depth | S2/S1 | 90% whole-class interval | 98% interval | Nominal +/-5% rule | Simultaneous rule |
|---:|---:|---|---|---|---|
`;
for(const r of a.levels)text+=`| ${r.level} | ${f(r.ratio)} | ${ci(r.ci90)} | ${ci(r.ci98)} | ${r.equivalent_5pct_nominal?'passes':'inconclusive'} | ${r.equivalent_5pct_simultaneous?'passes':'inconclusive'} |\n`;
text+=`
**Derived observations.** Each ratio first averages three S2 volcano means
and divides by S1 within one class, then takes the geometric mean across
eight classes. Each level has 192 separately executed one-target solves per volcano.
All 30,720 primary observations have equal effort by level; the 8,192
control observations and 2,944 BSGS checks are separate.

Effort is balanced, while target indices are deliberately shared across
transported vertices: 192/64/32/16/8 indices per vertex at depths 0..4.
The floor therefore has eight distinct scalar indices across its 24
vertices, with separate seeded solver executions. The whole-class bootstrap
retains this dependence; rows are not treated as IID targets.

The secondary equal-level ratio is ${f(a.equal_level_secondary.ratio)},
95% interval ${ci(a.equal_level_secondary.ci95)}. It does not substitute for
the five primary level results. All class means, individual S2/S1 ratios,
90/95/98/99% intervals and exact raw-file hashes are in
[panel-A.json](analysis/panel-A.json).

Class-level observations vary: class 7 has S2/S1 ratios ${f(a.levels[0].class_ratios[7])} at the
crater and ${f(a.levels[4].class_ratios[7])} at depth four. The pooled result does not establish
uniform equivalence for each class; these observed deviations remain
visible in the raw class means and are not explained away by pooling.

![Recorded ratios; light bars are 98% and dark bars 90% whole-class bootstrap intervals.](figures/level-ratios.pdf)

## Controls and validation

| Check | Native observation |
|---|---|
| Real positive control, plain rho / negation rho | ${f(a.controls.rho_plain_over_negation.ratio)}, 95% ${ci(a.controls.rho_plain_over_negation.ci95)} |
| A/A, same method on matching workloads | ${f(a.controls.aa.ratio)}, 95% ${ci(a.controls.aa.ci95)}; ${a.controls.matched_aa_pairs} matched pairs |
| Coordinate model 1 / model 0 | ${f(a.controls.coordinate_model1_over_model0.ratio)}, 95% ${ci(a.controls.coordinate_model1_over_model0.ci95)} |
| All control gates | ${a.controls.passed} |
| BSGS costs equal across transported instances per target | ${a.bsgs_exactness} |
| Planned A / B records retained | ${a.records} / ${b.records} |
| Independent scalar checks | ${independent.verified} correct; ${independent.mutated_scalar_rejections} deliberately changed scalars rejected |
| Independent input and graph checks | ${geometry.classes} classes; ${geometry.vertices} vertices; ${geometry.verified_Phi2_edges} Phi2 edges; ${geometry.irreducible_floor_quadratics} floor quadratic factors irreducible |

The measurement binary independently checked every recovered scalar; exact
ecbench replay re-executed every deterministic observation. A separately
written C++ implementation checks curve and subgroup primality, point
membership, subgroup orders, recovered scalars, and rejection of scalar+1.
It also recomputes the headline ratios from the exported raw-record CSV,
using no library arithmetic or native analyzer code. Its bootstrap uses
independent mt19937_64 draws, so it is a Monte Carlo cross-check rather
than byte-identical reproduction of Rust StdRng intervals.

| Depth | Native geometric mean | Independent recompute | Independent 90% interval |
|---:|---:|---:|---|
`;
for(const r of a.levels){const s=independent.levels.find(x=>x.level===r.level);text+=`| ${r.level} | ${f(r.ratio)} | ${f(s.geometric_mean)} | ${ci(s.ci90)} |\n`;}
text+=`
These checks use a separate implementation on the same host. They do not
claim a second physical runner, isolated timing, a fully priced IC/rho
win, or cryptographic-size validation. The committed
audit workflow can repeat the archive replay on another host. Machine-readable
certificates: [scalar and summary checks](validation/independent.json),
[geometry](validation/geometry.json), [export sources](validation/export-sources.json).

## CM-aware collector panel

Panel B uses four additional prime-order classes, two each for CM fields
-19 and -43, three sampled floor curves per class and an isomorphic model
of every curve. The large-conductor kernels have abscissa field degree at
least (N-1)/8. This is separate from the twist-rational construction used
for the deep volcanoes.

On every model, scalar and generic-recurrence orbit construction feed the
same relation collector and matched rho.negation reference. Direct degree-5
or degree-11 CM maps are additionally tested on maximal-order craters.
These small self-maps act within a crater; transport across the large-N
conductor gap remains a separate cost and construction problem.
All 32 prepared models have exact scalar/recurrence base equality; direct
map equality also holds on all eight crater models. The recurrence
P[k+2] = P[k+1] - [q]P[k], after one scalar initialization, is available
on both sides. A faster base constructor does not by itself establish a
faster unknown-scalar algorithm or a conductor-gap advantage.

The factoring certificate ${identity.id} is retained in
[recurrence-factorization.json](analysis/recurrence-factorization.json)
and checked with 128 additional checker-seeded points. It evaluates
z(lambda^2-lambda+q) = lambda(lambda z)-lambda z+qz over the certificate
field. The group recurrence additionally requires the per-instance
characteristic-polynomial relation and prime-order point checks; a finite
polynomial evaluation is not a complete CM geometry proof.
The prior study's conditional Lean theorem recurrence_matches_orbit gives
the two-state invariant; the present native preflight checks its actual
point realization on all prepared models.

| Factor-base construction | Recorded attempts | Mean charged group GAE |
|---|---:|---:|
${modes.factor_base_costs.map(x=>`| ${x.arm} | ${x.records} | ${f(x.mean_factor_base_gae)} |`).join('\n')}

These are construction-phase group-work counts. Direct-map counts cover
craters only and omit unpriced field work; they are not zero-cost or
fully priced speed claims. Recorded cold-cost floors and all matched cells remain
in the native summaries below.

![Recorded cost floors for model 0 in one class of each CM field. All cells and pricing bands are retained in native JSON.](figures/cm-cost-floors.pdf)

All 1,664 B attempts, including the matched references, retain their
statuses. [panel-B.json](analysis/panel-B.json) gives every class, volcano,
coordinate model and arm; [mode invariants](analysis/b-mode-invariants.json)
checks matching seeds, recovered scalars, queries, relations, ranks and
every non-base counted phase between constructions. No censored workload
is replaced by a success-only average.

Every arm, including rho, is evaluated at 0, 0.1 and 1 GAE per unpriced
native operation using ecbench's common-band implementation. All per-curve
bands, missing counts, uncharged native counters, phase costs, query/rank
totals, peak memory and statuses are in
[diagnostics-A.json](analysis/diagnostics-A.json) and
[diagnostics-B.json](analysis/diagnostics-B.json). Unknown prices stay
unknown. A quotient of two cost floors is a diagnostic, not a bound on
the full-cost quotient. No fastest-tool or full-cost method ranking is
admitted here; no Magma/msolve comparison has been performed.

The band prices the reported counters. It cannot supply missing counts:
the CM action's scalar modular arithmetic, parts of twist-kernel square-root
search and codomain normalization do not have complete separate operation
counters. This setup-accounting gap is distinct from an unknown conversion
price. Neither it nor unmeasured native work is assigned zero physical cost.

## Input coverage and scope

| A class | CM discriminant | Field prime p | Prime subgroup r | Conductor gap N |
|---:|---:|---:|---:|---:|
`;
for(const c of load('prepared-a/a-classes.json')){const x=c.pair.class;text+=`| ${c.class} | -${c.d} | ${x.p} | ${x.n} | ${x.gap_prime} |\n`;}
text+=`
| B class | CM discriminant | Field prime p | Prime subgroup r | Gap N | Kernel-abscissa field degree |
|---:|---:|---:|---:|---:|---:|
`;
for(const c of load('prepared-b-v2/b-classes.json'))text+=`| ${c.class} | -${c.d} | ${c.p} | ${c.r} | ${c.gap_prime} | ${c.kernel_abscissa_degree} |\n`;
text+=`
All exact ICV1, EC1 and generator identities are in the registered
[A identities](prepared-a/identities.json) and
[B identities](prepared-b-v2/identities.json), and every frozen plan reports
registered models. Independent geometry checks verify the field/order
discriminant identity, prime gaps, exact kernel-field degrees, disjoint
volcano j sets, all rational 2-isogeny neighbors and floor completeness.
Scalar-order witnesses exceeding the Hasse-interval width certify the
claimed full order. Endomorphism-order interpretation still uses standard
ordinary-CM/order-lattice mathematics; it is not a new full Lean proof.

The diagrams use the repository's standard volcano renderer. Labels map
to full identities in the adjacent .identities.json files.

![One S1 volcano in A class 0.](figures/volcano-c0-v0.png){height=8.4in}

![One S2 volcano across the gap in the same class.](figures/volcano-c0-v1.png){height=8.4in}

**Interpretation limits.** Eight deterministically selected A classes are
a fixed cohort, not a representative random sample of ordinary curves.
The whole-class bootstrap quantifies sensitivity within that cohort;
its nominal coverage is not a theorem about all CM fields. A is at 32/34
field bits and depth four, explicitly smaller than the parent's depth-six,
52-bit class. B has no deep 2-volcano and independently chosen floor
generators; it does not supply hard-gap target transport. Cofactor-one
CM collection is not implemented on the deep panel. Prime-field F4/SAT
descent, a general exploitable endomorphism algorithm, complete common
pricing, larger independent families and cryptographic-scale coverage
remain open. No scaling exponent, hardness theorem, impossibility claim
or deployed-curve security conclusion follows, and a possible intermediate
algebraic advantage is not ruled out by a generic-rho result.

## Preflight corrections, provenance and reproduction

The first B fixture set failed direct-map preflight because its twist
lacked the required small CM kernel; it also contained an anomalous trace-one
case. It was never used for solver measurements. The rejected
[v1 fixtures](prepared-b/b-classes.json) and
[failure](preflight-v1/base-equality.stderr) remain intact. The dated
[amendments](AMENDMENTS.md) add map eligibility and exclude anomalous
curves before primary data. All 128 disjoint calibration solves verify
and replay, and remain excluded from inference. Calibration estimated
about 94 minutes including startup and replay; the shared main guard
was raised to 120 minutes before primary observations. No sample count
or inference threshold changed after data.
The actual main command span was about 84.3 minutes; the conservative
calibration overestimated replay overhead. Timing here describes execution
and resource use, not an admitted algorithm-performance comparison.

The [original protocol](PROTOCOL.md), dated amendments, committed source,
exact 64 primary specs, native plans, compiler/binary hashes and
[final preflight](PREFLIGHT-v3.json) are retained. The seven-command
[main card](cards/CARD-conductor-gap-main-20261025-1.yaml), its receipt,
verbatim executor relay and director assessment record execution separately
from scientific interpretation. The calibration has the same artifacts.
The assessment documents the canonical task-card checker's post-execution
lint/output-order limitations without altering its code or evidence.

A post-measurement catalogue check caught integer rounding in the initial
JavaScript registry merge. The lossless Rust repair restores 362 legacy
models from the exact prior snapshot and preserves all 2,976 experiment
models. The original receipt's legacy-preservation assertion was insufficient
and is explicitly retracted in [REGISTRY_CORRECTION.md](REGISTRY_CORRECTION.md).
The frozen measurements, identities and preflight artifacts remain intact.

All rows use separately executed one-target cold solves with fresh setup;
each invocation owns its tables and solver state. Transported scalar
indices follow the fixed shared law. Group-operation cost excludes
cross-gap parameter discovery, map construction and transport. Their
native receipts are in the frozen pair/export files; target scalars are
shared to construct corresponding known-answer workloads without timing
a map as free. The parent's warm-transfer and exponent claims are corrected
explicitly in [CLAIM_CORRECTIONS.md](CLAIM_CORRECTIONS.md).

Raw sessions are preserved byte-for-byte in the deterministic archive
\`runs.tar.xz\` (contains \`sessions/\`). From this study directory:

\`\`\`sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
\`\`\`

From the repository root, rebuild the frozen measurement commit using the
recorded compiler; run ecbench verify with --replay-all --exit-code on
each extracted session. The audit workflow contains the complete loop.
The native example's analyze/diagnostics modes regenerate summaries into
fresh output paths; independent.cpp and independent_geometry.cpp supply
the separate arithmetic checks. The analysis CSVs carry exact curve,
generator, target and recovered scalar values for independent recomputation.

Every session has its own descriptive report, recorded-value figure and
inspected PDF under reports/. The raw records and auditor receipts are
never edited. Source generators: render.py, validation-inputs.mjs and this
metadata-only report renderer. No complete-cost IC/rho leaderboard or
benchmark row is promoted without the existing validation gate. The open obligations remain in
[the conductor-gap problem record](../../problems/conductor-gap-upper-floor-and-cost.md).
`;
const destination=process.argv[2]??root+'/REPORT.md';
writeFileSync(destination,text,{flag:'wx'});
console.log(destination);
