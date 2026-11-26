# RR5 compact B8: completed versus refused traversal, and certification scope

2026-11-24 UTC. Parent #5131 head 77364b75a430f81b06548225e069ba6536f274dc. Read-only source/evidence audit; no new curve arithmetic or benchmark. [pcc:unverified]

## Actual frozen evidence

Read #4953 at c8d50db7fb0f374e30947125cb7ed4286325f3e1, its REPORT.md, MATHEMATICS.md, qualification summary and all four raw B8 outputs; #4950 at 09fb896f69470a86a2237c51a01526abc2932af6 and its frozen protocol. Both prior PRs are closed. The existing #5128 root qualification is still open; #5142, #5136 and #5120 heads are unchanged from the preceding audit. #4977/#4979 are merged and are not maintained. No duplicate runs or code changes.

The B8 fixture in #4953 is on inherited M31: p=2147483647, a=847514846, b=2087444724, r=2147446831, G=(625037715,2100627251); natural-x base abscissae 0,1,2,3,5,6,8,9. These are not the B31 Dickson fibre. The two public targets, base, and limits match across the raw expanded and compact records. Each independent oracle enumerated 1120 signed distinct-index subsets, not all repetition/cancellation cases. Only fixed historical inputs were inspected, not rerun.

| Presentation | Target | Status | F4 field_ops | max rows / columns | F4 runs | Signed rows |
|---|---|---|---:|---:|---:|---:|
| expanded | planted | Inconclusive | 633942 | 208 / 3752 | 1 | 0 |
| expanded | generator | Inconclusive | 633942 | 208 / 3752 | 1 | 0 |
| compact | planted | Complete | 54099486 | 1085 / 1121 | 7 | 1 |
| compact | generator | Empty | 51100692 | 1085 / 1121 | 1 | 0 |

The expanded runs refused traversal when no algebraic univariate was derived. Their work is not a complete-query denominator. The compact negative control takes 94.46% of the compact positive control's reported F4 field operations despite yielding zero rows. This is a deterministic ratio of stored partial counters, not a wall-time ratio, expected miss cost, or cold cost. The raw records explicitly set phase_accounting_complete=false. Reported solving degrees 9 (refused) and 4 (completed) cannot stand in for operation count or proof of asymptotic improvement. The comparison changes formulation and ordering. No new timing or speed result. [pcc:unverified]

```text
Expanded positive / negative -> refused / refused -> neither certified
Compact positive / negative  -> complete / empty  -> fixed regular chart
All four-summand cases      -> additional chart coverage still required
Natural Dickson M31 panel   -> no inference from these two fixed controls
```

## Exact output and uncertainty gate

The inspected RR5 mathematical note excludes identity target, target x in the base, repeated leaf abscissae, and two-torsion leaves. Its complete/empty conclusion is restricted to the declared chart. The #4950 oriented protocol instead distinguishes full signed chains, multisets, augmented rows, and useful independent rank. These are different output contracts. For a future reference-backed audit, distinguish a certified complete matching output, a demonstrably wrong output, and an unresolved output (including missing chart, cap, or incomplete replay). On any fixed panel of n cases with F demonstrated failures and U unresolved, the unobserved exactness-failure fraction is only identifiable within [F/n,(F+U)/n], if F and U are disjoint and the other cases are fully certified. A timeout is not an empty and cannot be removed from the denominator. For a completion SLA it is a failure, rather than an unresolved exactness result.

For an independent representative Bernoulli validation panel of a predeclared stratum, zero failures among n fully adjudicated trials has one-sided 95% exact upper failure bound 1-0.05^(1/n). To certify a bound below 5%, 1%, or 0.1% requires at least 59, 299, or 2995 trials respectively, assuming independence and zero failures. These are design calculations, not a chosen execution budget. The two fixed B8 controls cannot be treated as iid trials. Positive and negative strata have distinct conditional distributions; planted correctness cannot establish blind yield. The preceding B31 support ceiling makes a small natural-target panel particularly weak for positive correctness. NIST exact-binomial reference: https://itl.nist.gov/div898/software/dataplot/refman2/auxillar/exacbino.htm . No new seed schedule was created because no trial was authorized/executed.

## Next decision and limits

Preserve these fixed B8 positives/negatives as historical chart regression controls; do not reuse them as held-out evidence. Keep the active native qualification lane and the full existing M31/seed/worker/resource/publication instructions. Actual current-root dedicated M31 Dickson four-summand validation, complete cold accounting and independent verification remain separate. Native workers, local Rust, Conductor and research-KB are unavailable; coordinator used parallel repository reads and deterministic arithmetic on retained counters. No new claim issued or signed, native execution, curve substitution, paid work, Space mutation or merge. Source review is bounded, not an exhaustive prior-art survey. [pcc:unverified]

Impact score: 3/10 — source-linked comparison separates completion from refusal and quantifies the cost of a particular completed empty control, with a statistically explicit certification rule. No new extraction algorithm or measured speedup.
