# Why the degree-32 PKM map is cheaper — F4 shape and full-pipeline pricing — 2026-11-25

## F4 shape

The `ic bench` JSON already records the decomposition system and every F4
solver call. The two arms have the same nominal system — 10 variables, 11
equations, degree vector `[4,2,2,2,2,2,2,2,2,2,2]` — but the isogeny
membership equations produce a smaller elimination problem.

| Metric | 13-bit interval (20261185) | 13-bit isogeny t5 s5 | 14-bit interval (20261301) | 14-bit isogeny t5 s7 |
|:--|---:|---:|---:|---:|
| Variables / equations | 10 / 11 | 10 / 11 | 10 / 11 | 10 / 11 |
| Mean solving degree | 6.0 | **5.0** | 6.0 | **5.0** |
| Max F4 rows | 1,493 | **821** | 1,493 | **821** |
| Max F4 columns | 928 | **583** | 925 | **583** |
| Max F4 nonzeros | 283,474 | **47,326** | 282,099 | **31,885** |
| F4 multiply-adds (32 rows) | 209.5 B | **6.40 B** | 881.7 B | **27.95 B** |
| Solver calls (32 rows) | 1,645 | 1,610 | 6,928 | 7,056 |
| Refuted systems (32 rows) | 1,405 | 1,402 | 6,748 | 6,868 |
| Linear-algebra rank / rows (median) | 14 / 14 | 13 / 13 | 12 / 12 | 11.5 / 11.5 |

The dominant effect is the **one-degree lower solving degree**, which shrinks
the F4 matrix and its nonzero count by roughly 6x and the multiply-add total
by about **32x**. Relation probability is not the lever: the candidate finds
about the same number of relations per trial (1.02–1.07x).

The system dimensions and degree vector are identical, so the saving is in the
membership polynomial's coefficient structure — the five degree-2 Vélu steps
produce a system whose first-fall degree is 5 rather than 6. That is the
structural explanation the seed sweeps could not provide.

## Full-pipeline pricing

The same `ic bench` rows price the complete cold target at this instance size:
relation collection and failed trials (`cost.trials`, `cost.relations`), the
F4 solver (`decomposition.solver`), the relation matrix
(`linear_algebra.rank`, `rows`, `columns`, `work_native`) and the recovered
scalar (`recovered`, `verified`). `total_gae` is the charged total;
`s_over_rho` divides it by the matched same-target rho reference.

| 13-bit instance | Interval median s/rho | Isogeny median s/rho |
|:--|---:|---:|
| 20261185 | 958,469 | 529,542 |
| 20261185 (mirror) | 863,411 | 238,297 |
| 14-bit 20261301 | 1,230,355 | 333,083 |
| 14-bit 20261301 (mirror) | 1,172,974 | 313,499 |

The linear-algebra block reaches rank 12–14 on 12–14 rows, so these are
full-rank solves, not partial relation collections. The candidate remains
**2.2e5–5.3e5** times the matched rho reference; the component advantage does
not translate into a same-target crossover at these sizes.

## Limits

- The 1,000-trial cap and five-second solver budget are frozen; failed and
  censored attempts are charged in `total_gae`, but a larger trial budget could
  change the completion probability.
- The systems are 10 variables; the measured PKM chain ceiling is ~14
  variables, so this shape result does not extrapolate to NIST sizes.
- The 14-bit panel is one curve with three target replicates; the 13-bit panel
  is three selected 13-bit instances.
- No asymptotic or NIST-size claim follows.
