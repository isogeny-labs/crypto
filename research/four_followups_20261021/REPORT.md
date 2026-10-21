# Four follow-ups: evidence status after the first complete sweep

**Scope:** four authorized leads on the 21 exact-order Scholten models and the
original 60 one-target workloads per model. The [frozen protocol](PROTOCOL.md)
defines the comparisons. This report records the complete factor-base/jump
sweep and an exact Kummer ordinary-addition gate. The complete law and pencil
collector remain open requirements; their previous results are not counted as
new executions here. Values below are observed operation counts or values
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

## Kummer ordinary-addition gate

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

## Requirement status

| Requested lead | Status in this report | Remaining evidence |
| --- | --- | --- |
| Factor base / large-prime graph | Verified complete for the five frozen parameter arms, 1,260 targets each | Other policies are new experiments; qualified wall timing and external comparison before speed claims |
| Complete inversion-free law | Not implemented | Independent implementation of the published complete formulas, exceptional-case cross-checks, 1,260 full solves and priced rho comparison |
| Point-driven pencil collector | Not attempted on full cohort | Adapt the existing 20-target gate to all 21 models and 1,260 targets; a sparse rank solver is needed at the upper sizes, then full cold accounting and replay |
| Kummer walk arithmetic | Partial: ordinary-addition substitution rejected by exact gate | Design differential walk with sign/lift-safe labels, then arithmetic checks and full cohort |

The [PDF](REPORT.pdf) includes the two figures and this result table. The
structured [manifest](results/manifest.json), [per-target table](results/matched/targets.jsonl),
[price grid](results/matched/price-grid.csv), [summary](results/matched/summary.json) and
[`SHA256SUMS`](SHA256SUMS) and [data dictionary](DATA_DICTIONARY.md) support independent review. The plotting script
reads the native analyzer's output and performs layout only.
