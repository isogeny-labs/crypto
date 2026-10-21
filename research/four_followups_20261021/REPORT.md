# Four follow-ups: evidence status after the factor-base sweep and pencil pilot

**Scope:** four authorized leads on the 21 exact-order Scholten models and the
original 60 one-target workloads per model. The [frozen protocol](PROTOCOL.md)
defines the comparisons. This report records the complete factor-base/jump
sweep, a 540-target pencil pilot, sampled branches of the published complete law,
and an exact Kummer ordinary-addition gate. The full law and pencil cohort
remain open requirements. Values below are observed operation counts or values
derived from them; wall times are diagnostics only.

## Factor-base and jump sweep

The five frozen arms each ran 1,260 separate targets. **6,300/6,300 candidate
scalars and 6,300/6,300 in-process rho scalars passed a fresh source-curve
multiplication**, with no failures or cap exhaustions. The analyzer paired all
records by workload hash, seed, cap,
target index and planted scalar, and reproduced the incumbent's 1,260 count
and phase records exactly. The representative price is `M + 0.5S + 10I` plus
0.1 source-addition equivalents (GAE) per declared native logical operation.

| Arm | Processed divisors | Complete GAE | Candidate / wrapped external | Candidate / matched rho | Candidate cells below external, of 36 |
| --- | ---: | ---: | ---: | ---: | ---: |
| Full base, 16 jumps (replay) | 219,581,773 | 2,666,291,894 | 1.000000 | 2.448830 | 0 |
| Full base, 8 jumps | 220,092,171 | 2,664,487,778 | 0.999323 | 2.447173 | 15 |
| Full base, 32 jumps | 219,864,588 | 2,671,135,032 | 1.001816 | 2.453278 | 3 |
| Half base, 16 jumps | 282,983,105 | 3,285,713,695 | 1.232316 | 3.017732 | 0 |
| Quarter base, 16 jumps | 392,583,830 | 4,267,226,320 | 1.600435 | 3.919193 | 0 |

Matched rho totals 1,088,802,370 GAE. None of these arms is below rho in
any of the 36 pooled pricing cells. The full price grid, all 6,300
per-target joined rows, and all 35 raw JSONL files are in
[matched results](results/matched/). The separate native
[validator](results/matched/validator-report.json) recomputed 15 headline
values directly from the raw records. Each matched run also reproduced its
archived source-rho counts and native charges exactly.
The [graph](factor-base-grid.svg) plots each size separately; it shows the
8-jump arm below the 16-jump arm at 16, 20, 24 and 40 bits, and above it at
28, 32 and 36 bits. This is a fixed cohort with one algorithm seed per target;
no confidence interval or isolated wall-time comparison was measured.

The small-base policy rejects a split relation when both roots lie outside
its fixed, target-blind base. It rejected 35,385,164 such relations for the
half base and 110,402,348 for the quarter base; all candidate additions and
split tests remained charged. The experiment compares this specific
one-large-prime restriction with the incumbent all-node graph. It does not
test every possible factor-base policy or graph reduction.

The first candidate-only run was made after the protocol commit, with a
one-line failure-row preservation change still uncommitted. The executable
source was then committed as `f6a440f0e` and all 35 files were rerun;
[`initial-run.sha256`](results/initial-run.sha256) records byte identity.
The matched runner then added same-process source rho for every target. Its
archived-input replay compares all 35 matched JSONL files byte-for-byte.
The first run's wall diagnostics are retained in
`results/initial-times.tar.gz`.

## Kummer model and ordinary-addition gates

The exact quintic of each of the 21 transferred models has five distinct
rational finite Weierstrass roots. Exhaustive evaluation over each $\mathbb F_p$,
derivative checks at the roots and reconstruction of the quintic from its
linear factors agreed on **21/21** models. The [raw root table](results/kummer-model-gate.jsonl)
replayed byte-for-byte; the [root-location graph](kummer-model-roots.svg) and
[model-gate PDF](KUMMER_MODEL.pdf) show the per-model values. This meets the
rational two-torsion field-of-definition condition for the fast-Kummer
construction in [Gaudry's paper](https://eprint.iacr.org/2005/314/).

For each of the 21 transferred order-`r` subgroups, set `P = tau(G)` and
`Q = 2P`. The quotient map `K(D) = {D,-D}` gives the same unsigned input for
`Q` and `-Q`, but the outputs are `K(3P)` and `K(P)` respectively. Since
`r > 4`, these outputs differ. The native gate checked source order,
Jacobian arithmetic and the distinct outputs on **21/21** models; the
[21 raw witnesses](results/kummer-ambiguity.jsonl) and
[diagram](kummer-ambiguity.svg) retain coordinates. This establishes that
the present arbitrary-jump walk cannot replace its ordinary additions with a
binary Kummer addition on unsigned inputs alone. Differential addition with
known differences and a sign/lift recovery rule is a separate, unimplemented
collector path. This is a mathematical applicability gate, not a timing result.

## Point-driven pencil pilot

The point-driven sieve and a new sparse modular row collector solved and
source-verified 60/60 targets on each of nine original models: p = 739,
1,151, 1,187, 3,371, 3,527, 3,931, 11,731, 11,887 and 12,143. Same-process
source-negation rho solved the same 540 targets. An independent native
validator has rechecked both recovered scalars on all 540 targets,
matched the rho counters to frozen controls, recomputed 27 representative
price quantities and checked all 324 price-grid cells. A separate rerun matched all non-wall fields of the nine
61-row files. The direct F739 port
also reproduced the prior 61-record gate after removing wall-clock fields.

At `M+0.5S+10I` and 0.1 GAE per native logical operation, the complete cold
one-target pencil/rho ratios are:

| p | 60 target pairs source-verified | Modular row updates | Pencil / rho |
| ---: | ---: | ---: | ---: |
| 739 | 60 | 6,288,519 | 12.931178 |
| 1,151 | 60 | 20,344,734 | 28.666756 |
| 1,187 | 60 | 23,112,414 | 31.080921 |
| 3,371 | 60 | 442,470,426 | 246.117110 |
| 3,527 | 60 | 499,795,794 | 267.374941 |
| 3,931 | 60 | 690,243,338 | 332.187349 |
| 11,731 | 60 | 17,235,887,587 | 4,331.361006 |
| 11,887 | 60 | 17,655,108,982 | 3,856.238613 |
| 12,143 | 60 | 19,113,208,311 | 4,250.094923 |

Across the nine completed models and 36 pricing cells each, 0/324 pencil
cells were below matched rho; the cell ratios range from 7.870218 to
5,104.745253. These are exact prices for this fixed cohort and declared
operation model.

Full setup is charged to every target even
though it was physically reused for 60 solves per model. The [pilot PDF](PENCIL_PILOT.pdf),
[graph](pencil-pilot-cost.svg), [raw and price grid](results/pencil-pilot/)
show the disaggregated values. A separate 28-bit single-target probe at
$p=48{,}383$ source-verified both scalars after 20,814,919,424 modular row
updates; its cold-priced ratio is 96,957.132823 at the representative cell.
The [single-target PDF](PENCIL_ONE_TARGET.pdf), [36-cell graph](pencil-one-target-grid.svg)
and [raw row](results/pencil-pilot/f48383-one-target.jsonl) retain the full
counters. An independent archive-input replay matched every non-wall field,
and the native validator checked both source scalars, five summary values and
all 36 pricing cells. The cell ratios range from 26.422722 to
113,636.290374; 0/36 are below rho. This one target is not a 28-bit
curve estimate or a full-cohort pencil result.

## Complete-law arithmetic gate

An implementation directly from equations (13) and (14) of the
[Ozbay Gürler–Hisil paper](https://link.springer.com/article/10.1007/s13389-024-00351-7)
checked degree-one plus degree-one projective addition on every normalized
model. **13,608/13,608** cases matched Cantor across two projective scales:
760 inverse pairs, 752 doublings and 12,096 distinct-point additions. The
[1+2 formulas](https://link.springer.com/article/10.1007/s13389-024-00351-7)
matched Cantor on all 12,096 sampled cases: 9,492 generic disjoint,
1,304 inverse-support cancellations, 1,258 repeated-support additions
with a distinct remaining point, and 42 triplings. The 6,551 checked 2+2
additions comprise 5,040 generic, 336 ordinary doublings, 483 degree-drop
additions, 336 inverses, 294 shared-support additions, 42 same-polynomial
mixed-sign additions, 13 special-point doublings and 7 degree-drop
doublings. Every evaluated output matched Cantor. The [raw per-model counts](results/complete-law-branches.jsonl)
replayed byte-for-byte; the [1+1 and 1+2 graph](complete-law-gate.svg) and
[2+2 graph](complete-law-2plus2.svg) show branch counts. This finite sample
does not establish exhaustive exceptional-case coverage. Full integration
and 1,260-target DLP cohort remain unexecuted. This is an arithmetic branch
check, not a complete-law cost.

## Requirement status

| Requested lead | Status in this report | Remaining evidence |
| --- | --- | --- |
| Factor base / large-prime graph | Verified complete for the five frozen parameter arms, 1,260 targets each | Other policies are new experiments; qualified wall timing and external comparison before speed claims |
| Complete inversion-free law | 1+1 checked on 13,608 cases, 1+2 on 12,096 and 2+2 on 6,551 sampled cases across 21 models | Establish exhaustive exceptional 2+2 coverage, integrate the law, then run 1,260 full solves and priced rho comparison |
| Point-driven pencil collector | 540 pairs in nine complete curves plus one separately replayed 28-bit target: 541/1,260 distinct pairs | Run the other 719 targets, including scalable elimination at upper sizes, then full cold accounting |
| Kummer walk arithmetic | Partial: 21/21 models have full rational two-torsion; ordinary-addition substitution rejected by exact gate | Design differential walk with sign/lift-safe labels, then arithmetic checks and full cohort |

The remaining obligations are tracked in the [projective-law](../../problems/genus2-projective-transfer-backend.md),
[pencil elimination](../../problems/genus2-pencil-upper-size-elimination.md)
and [Kummer relation-label](../../problems/genus2-kummer-relation-labels.md)
problem records. Each record gives an explicit completion test; none of these
partial gates is a full end-to-end result.

The [PDF](REPORT.pdf) includes the result tables and figures. The
structured [manifest](results/manifest.json), [per-target table](results/matched/targets.jsonl),
[price grid](results/matched/price-grid.csv), [summary](results/matched/summary.json) and
[`SHA256SUMS`](SHA256SUMS) and [data dictionary](DATA_DICTIONARY.md) support independent review. The plotting script
reads the native analyzer's output and performs layout only.
The [verification log](VALIDATION_NOTES.md) records the release checks and
the initial temporary-filesystem failure with its successful rerun.
The immutable run notes for this update are the [expanded law gate](reports/run-20261021T1715Z/README.md),
[Kummer model gate](reports/run-20261021T1718Z/README.md) and
[two additional pencil cohorts](reports/run-20261021T1721Z/README.md).
The [28-bit single-target replay](reports/run-20261021T1706Z/README.md)
has its own run report because it is a separate workload slice.
