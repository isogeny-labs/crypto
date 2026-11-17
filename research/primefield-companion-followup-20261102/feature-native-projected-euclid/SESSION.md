
## 2026-11-07 — Projected Euclidean resultant (T-1770)

### What was done (commands, files, features)

Implemented an alternative projected-source backend using Euclidean resultant reciprocity, leading exponents modulo 2/4 and coefficient-only root products. Nine selected native methods pass; new checks cover 1080 Sylvester determinants (69 zero) and 245 root products. Regeneration exact; PDF rendered and inspected. PCC-20261107-1cb7a8 unsigned. [pcc:unverified]

### Decisions made (and why)

Retain prior backend; ordinary Euclid and per-resultant projection are explicit costs. Source audit gives w+k>=ceil(sqrt(r+1))-1 for this geometry representation; decoder still enumerates r inputs. Sources checked: prior source/compiler/linear-constructor and FLINT polynomial resultant documentation. KB unavailable: no tools and wrapper executable absent.

### Open threads (unfinished, pick up next time)

Compose representatives before projection; investigate half-GCD transition batching and implicit orbit products with cheap evaluation. Compact nonlinear decoder construction remains separate. Historical PCC failures persist. Full research goal active.

### Errors (what went wrong, how it was handled)

No native build or test failure. No cloud or timing panel. Original evidence and cohort unchanged.
