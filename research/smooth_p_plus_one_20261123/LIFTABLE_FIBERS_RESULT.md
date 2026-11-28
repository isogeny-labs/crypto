# Liftable fibers complete four larger queries

2026-11-25 · `liftable_fibers_v1` · producer-local, unsigned.

Restricting each fine fiber to its public liftable-base coordinates gives
**four complete answers on the five degree-32 targets**. The fifth times out
after 99 of 119 charts. Across the planned panel, the candidate completes
11 of 12 attempted queries, with 13 planned. The F127 panel stops early when
the unchanged cofactor comparator returns an incorrect empty answer. The
failed comparison and two unattempted queries remain explicit. Claim
PCC-20261125-4219a4 is unsigned; no timing ratio is claimed. [pcc:unverified]

## Complete, incomplete and missing outputs

| Fixture | Planned per arm | Liftable fibers | Cofactor |
|:--|--:|:--|:--|
| F127, degree 16 |4|3 exact; 1 unattempted|2 exact; 1 mismatch; 1 unattempted|
| F8191, degree 16 |4|4 exact|4 exact|
| F8191, degree 32 |5|4 exact; 1 timeout|5 incomplete|

All three audits pass before the solver panels start. All three panels are
attempted; exits are 1/0/0. The records contain 17 exact, 6 undetermined and
1 mismatch outcome, with2 unattempted queries, no retry and no required
capture omission. F127 stderr records the mismatch; other panel stderr is
empty. The frozen wrapper exits one. [pcc:unverified]

![The full planned population includes failures and the missing F127 suffix. Green means the complete signed set equals Direct; a partially collected set is not an answer.](liftable-fiber-outputs.pdf){width=100%}

At degree 32, accepted row counts are 1/5/3/4 for targets 17G/43G/152G/planted.
Target 89G has two rows in its incomplete prefix, but remains undetermined.
Each completed candidate chart uses the first prescribed form. The previous
full-fiber factorwise panel completed none of these five larger targets;
that historical panel was not rerun as a timing comparison. [pcc:unverified]

## A retained comparator failure

For F127 target (41,95), liftable fibers return all 13 reference rows.
Cofactor emits an empty set and the comparison rejects it, stopping before
target (41,32) in both arms. The previous successful record has identical
prepared equations, input prefix and fixed form, but different RUR bytes.
Both outputs pass the existing eliminant-degree/dimension guard. The
[byte comparison](evidence/liftable_fibers_v1/comparator_difference.json)
does not diagnose which parametrization or extraction assumption failed.
Next validate the retained coordinates against their exact equations and
form in an offline native audit, using both records. No replacement run or
relaxed acceptance is justified by this observation. [pcc:unverified]

\newpage

## Construction and finite qualification

For each fine image b, replace the full map fiber by
\(H_b^B=\prod_{x\in B,\,\phi(x)=b}(X-x)\). Keep the same monic factors,
remainder equations, RR5 norm quartic and capacity catalog. Grouping and
membership-polynomial construction occur inside each query deadline.
This reuses the comparator's established liftable-base membership idea
locally; it is not a new base family. The [frozen argument](LIFTABLE_FIBERS.md)
gives guarded coverage and the unchanged local rank argument.

| Fixture | Charts | Admissible bindings | Full rank 7 | Equation terms |
|:--|--:|--:|--:|--:|
| F127, degree 16 |36|1,038|1,038|32–41|
| F8191, degree 16 |18|560|560|32–41|
| F8191, degree 32 |119|16,008|16,008|33–48|

All 17,606 admissible signed bindings satisfy the actual equations with
fiber rank 4, RR5 rank 3 and total rank 7. Every previous per-chart tuple,
exclusion and rank count reconciles. Sixteen identity and 74 target-abscissa
overlap exclusions remain. Systems have seven variables, eight equations
and input degree at most four. The audits share catalog, interpolation and
rank code; they are not independent implementation evidence. [pcc:unverified]

![The public membership change preserves the full-answer acceptance contract.](liftable-fiber-flow.pdf){width=100%}

## Reproducibility and limits

Source **3b7cd7422d**, before-call identity **c2d6a4c318**. All 18 before-call
file hashes stay unchanged. Source controls pass 47 feature tests and 20
default tests, and the release build completes. The four added tests cover
exact fiber membership, malformed inputs, extra-root rejection and shared
dispatch controls. Claim PCC-20261125-8eaff2 is unsigned. [pcc:unverified]

[Raw records and summaries](evidence/liftable_fibers_v1/),
[reproduction](LIFTABLE_FIBERS_REPRODUCE.md),
[flow](evidence/liftable_fibers_v1/development_flow.json). The three-second
shared query limit, sixteen-form policy, construction/capture caps and
conservative RUR guard are unchanged. Base/reference setup and final JSON
emission remain outside the deadline. Separate child stderr and partial
timeout files are not archived. No recovery, EKS run or admitted timing
comparison is represented. The incomplete comparator panel prevents treating
this as a fully successful paired qualification.

The written argument (PCC-20261125-6cb640) is unsigned. The Lean instance
map identifies missing generic remainder/RR5 bridges and an unbuilt assembly
probe; no checked theorem, global radical certificate or formal solver
completeness result is supplied. Prior sources include the lab's
quotient-product and first-collision studies and PKC 2016 Algorithm 1 and
sections 3.2/3.4. KB was unavailable; the implementation remains torus/Dickson.

Three new raw panel files join eight earlier files awaiting required
artifact upload and byte replay. No upload of these newest files is claimed;
the latest workflow evidence is in [development checks](DEVELOPMENT_CHECKS.md).
The live hook retains 709
local records and has no configured receiving URL. Prior reports and
performance graphs remain unchanged. [pcc:unverified]
