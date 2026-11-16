# Frozen experiment protocol, version 1

Preregistration date: 2026-10-31. Status: pending execution admission.
The protocol commit must precede every research run. Changed arm definitions,
caps, selectors or target populations require a new version and fresh results.

## Objective and references

Test the sixteen families in CANDIDATES.md for (a) verified natural relation
yield and (b) lower degree for a precisely specified algebraic system. Use
the primary ICV1 model in README.md. Stage-level algebraic work is the primary
track; exhaustive sums and tables are correctness references.

For each size freeze two controls: a standard coordinate-space base and a
hash-selected orbit base. Compare candidates at matched admitted point and
column counts. Repeat algebraic comparisons with the identical point set
encoded directly and by selector wherever both are implementable. Existing
subfield cells are positive instrument controls only.

## Deterministic inputs

Use SHA-256 of UTF-8 strings
`n83-fb-designs/v1|purpose|curve_uid|family|seed|ordinal|counter`.
Seeds 315 and 316 are training; 1315 and 1316 are holdout. Decode digest
bytes big-endian. Use rejection sampling for uniform bounded indices and
field words; do not reduce modulo a bound and call it uniform. Record any
retry counter. Never tune a base using holdout targets.

Freeze canonical point dumps, FB1 identity, actual post-filter K and B,
curve UID, generator, cofactor, Frobenius eigenvalue and quotient rule.
Verify each dump with the repository's generic group backend. Preserve the
existing factor-base storage format; new constructor metadata is an extension,
not a replacement for point rows. Hash both raw bytes and canonical sorted
point sets to detect duplicate sets across recipes.

## Stages

**A — exact small controls.** Use existing E0 model representations at field
degrees 5, 7 and 9, registered before publication. Enumerate every nonidentity
prime-subgroup target and every admitted two-summand tuple for seed-space
dimensions 2 and 3. For three summands use degrees 5 and 7, dimension 2.
Keep n=9 proper-subfield arms separate. Export all algebraic equations and
enumerate Boolean assignments when the system has at most 18 variables.
Check every relation with exact group arithmetic. Build complete target
histograms and separate algebraic roots, twist-only roots and group relations.

**B — degree certificates.** For A's identical target panels and point sets,
scan bounded Macaulay degrees 0 through 6, retaining every completed degree.
For an unsatisfiable system, output an identity representing 1 at its first
refutation degree and a dual vector proving absence of 1 at the preceding
degree. Audit using `native/degree_certificate.rs`; keep the certificate and
input hashes. SAT systems require complete Boolean solution sets and a
separate Gröbner/solution-basis check; they do not get a refutation-degree label.
No omitted generator, selector, phase or lift condition is admissible.

**C — scaled holdouts.** Field degrees 11, 13, 17 and 19, then 31. For each
candidate use 64 uniform hash-selected unplanted targets per seed and 16
planted controls, with exact enumeration/MITM references wherever feasible.
Freeze seed dimensions 3, 4 and 5; larger dimensions are a follow-on version.
The arity panel is 2 and 3. Use 60 seconds per algebraic target, 4 GiB RAM,
and a 600-second candidate/seed cap. Record UNKNOWN separately. Do not drop
censored cells from degree distributions or compare means of completed
targets as though the panels were complete.

**D — primary degree-83 admission.** Construct families 1, 2, 5 and 7 first;
all other families remain in the planned queue. Target K=512 and K=600
admitted signed-Frobenius columns. A construction that cannot attain K gets
its measured K plus a matched smaller control. Freeze 16 unplanted and 16
planted targets per seed, arity 5, 300 seconds per target and 12 GiB RAM.
An exact 2+3/3+2 sum join or complete fixed-tag search is the correctness
reference where it fits; exceeding caps is UNKNOWN. Store a deterministic
coverage map for every fixed-tag search. Record global-selector degrees
separately from fixed-tag residual degrees.

The per-cell limits are local stop rules, not a new cloud-spending grant.
Use the existing authorized lab budget and execution admission; do not
expand resources, pool limits or paid concurrency to fill the entire grid.
Execute the queue incrementally and retain unattempted cells explicitly.

**E — usable rows.** For surviving core/envelope and typed-domain arms, run
rank/cycle checks on the same relation stream. Record duplicates, dependent
rows, cycles, row density, rank per verified relation and all graph work.
No preselected logarithms enter the solver. End-to-end measurements require
their own complete, matched protocol if subsequently requested.

## Degree claims and promotion

`D_ref(M)` is the least D for which 1 lies in the row span of the Boolean
Macaulay matrix built from every input generator with formal multiplier
degree plus generator degree at most D, then reduced modulo `x_i²=x_i`.
The input convention, generator list and variable order are frozen.
This is a certifiable bounded-Macaulay refutation metric.

Record separately: input degree, D_ref, first fall degree if measured,
largest matrix degree built, productive degree from a real iterative solver
trace, final reduced-basis degree, and any homogeneous regularity calculation.
None of these fields fills another field by assumption. The certificate
tool does not establish homogeneous regularity.

A finite-panel strict degree result requires paired upper/lower certificates
on the same complete targets and generators. A worst-case claim requires the
entire declared target population. A family-wide degree bound requires a
symbolic argument and independently checked proof; toy certificates alone
cannot establish it at degree 83. A residual bound after fixing selector
bits is conditional unless every branch is covered and charged.

Promotion criteria: zero false positives and false negatives on complete
checked cells; nonzero unplanted group-relation yield; either a strict
certified degree reduction or equal degree with lower width/work; and a
holdout Pareto improvement in yield-adjusted solver work and peak storage.
The strict finite-population degree target is a decrease of at least one
in the maximum D_ref over the identical complete paired target population.
Report common-UNSAT, candidate-only-UNSAT and reference-only-UNSAT populations
separately; changing which targets admit relations cannot silently change the
degree denominator. For sampled yield claims, retain paired target outcomes
and require a paired 95% interval for the yield difference above zero on both
holdout seeds. Insufficient samples remain inconclusive and trigger a new
preregistered panel rather than a claim from favorable point estimates.
Keep yield and degree wins separately when no arm satisfies both. Desired
structured degree at the primary size is at most 5; its verification remains
an acceptance condition, not an assumed property.

## Artifact and accounting contract

One record per arm/seed/target: source and binary hashes, equation and base
hashes, target point, target-selection metadata, exact reference label,
SAT/UNSAT/UNKNOWN, full solution/relation output, independent replay,
FP/FN/UNKNOWN counts, per-degree row/column/rank, certificate hashes,
field/group operations, native Boolean operations, matrix widths, peak RAM,
disk read/write bytes, candidate/survivor counts, unique supports and rank.
Planted recipes live in validation sidecars and never enter solver inputs.

Preparation, membership encoding, failed attempts, algebraic solve, lifting,
replay and relation filtering are exclusive counters. Report actual yield
`verified_hit_targets / attempted_targets`, with UNKNOWN count beside it;
complete reference yield uses its exact population denominator. At sampled
sizes retain binomial intervals and selection multiplicities. Timing is
admitted only under the lab's isolation/A-A requirements. Compare external
CAS on identical exported inputs before making a solver-speed claim.

## Launch gates and stopping

Use the repository's S3 checkpoint gate and EKS Spot skill. Bind the tested
source SHA, immutable executable image, complete spec, original deadline and
resource caps. A local build or unit test is development work; research runs
must wait for admission. No synthetic fixture substitutes for the specified
experiment. Preserve infrastructure failures and partial evidence.

Stop an individual cell at its frozen cap, label it UNKNOWN, and continue
other admitted cells within the existing budget. Stop a candidate's promotion
on any replay/certificate disagreement and investigate it without deleting
the failed record. Never count discarded random branches as exhaustive.
