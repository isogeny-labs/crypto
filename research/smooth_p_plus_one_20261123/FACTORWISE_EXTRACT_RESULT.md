# Factorwise extraction clears the degree-16 refusal

2026-11-25 · `factorwise_extract_v1` · producer-local, unsigned.

Factorwise extraction now returns the full signed answer on **all eight
degree-16 queries**, including the F127 target where the old pushforward
refused a two-image chart. It completes **8 of 13** fixed queries overall,
matching the unchanged cofactor comparator's **8 of 13**. Both methods remain
incomplete on all five degree-32 targets. Claim PCC-20261125-29f2a4 records
this bounded output result; it is unsigned and carries no cost ratio.
[pcc:unverified]

## Paired outputs and retained history

| Fixture | Queries | Historical pushforward | Factorwise | Cofactor |
|:--|--:|--:|--:|--:|
| F127, degree 16 |4|3 complete|4 complete|4 complete|
| F8191, degree 16 |4|4 complete|4 complete|4 complete|
| F8191, degree 32 |5|0 complete|0 complete|0 complete|

The historical pushforward comes from the [preceding frozen controls](FULL_CUTOFF_EXTRACT_RESULT.md)
and was not rerun here. The new paired panels emit all 26 planned records:
16 exact and 10 undetermined, with no mismatch, retry or required capture
omission. All three panels exit zero with empty panel stderr. Process success
is separate from completing every query. [pcc:unverified]

![Complete-answer counts and degree-32 factorwise chart prefixes. Grey regions represent incomplete planned queries. The historical arm is not a new paired timing observation.](factorwise-extract-counts.pdf){width=100%}

At F127 target (116,69), chart 7 with images [7,7,82,82] completes on the
first prescribed form, j=0, and contributes one signed row. The whole
36-chart union returns all ten reference rows. The old record retains sixteen
nonseparating-staircase refusals on that chart. Every degree-16 factorwise
chart completes on its first form. [pcc:unverified]

At degree 32, factorwise completes 22/24/24/24/23 of 119 charts before the
shared deadline; all five last attempted calls report `msolve timeout`.
Two prefixes contain one row each and remain unaccepted. Every cofactor
query reports `oracle budget exhausted` with no completed system. Earlier
chart work consumes the same deadline, so these failures do not measure
the isolated cost of the last chart. [pcc:unverified]

\newpage

## Target-by-target controls

| Target | Reference rows | Factorwise result | Completed charts | Cofactor result |
|:--|--:|:--|--:|:--|
| F127 (20,91) |9|9 exact|36/36|9 exact|
| F127 (116,69) |10|10 exact|36/36|10 exact|
| F127 (41,95) |13|13 exact|36/36|13 exact|
| F127 (41,32) |13|13 exact|36/36|13 exact|
| F8191 d4, 17G |0|0 exact|18/18|0 exact|
| F8191 d4, 43G |1|1 exact|18/18|1 exact|
| F8191 d4, 89G |0|0 exact|18/18|0 exact|
| F8191 d4, 152G |0|0 exact|18/18|0 exact|
| F8191 d5, 17G |1|Timeout|22/119|Budget exhausted|
| F8191 d5, 43G |5|Timeout|24/119|Budget exhausted|
| F8191 d5, 89G |2|Timeout|24/119|Budget exhausted|
| F8191 d5, 152G |3|Timeout|24/119|Budget exhausted|
| F8191 d5, planted |4|Timeout|23/119|Budget exhausted|

The [frozen protocol](FACTORWISE_EXTRACT.md) gives the exact fixtures and
limits. F8191 uses G=(0,789), with planted target (4375,6159). Reference rows
are available only to the comparison caller, never the algebraic arms.
[pcc:unverified]

![The constructor changes within the shared whole-answer acceptance contract.](factorwise-extract-flow.pdf){width=100%}

## Provenance, interpretation and next question

Source **d54d9d37bd**, before-call identity **8dd11c8e44**. All 16 before-call
file hashes stay unchanged. The source passes 43 feature tests and 16 default
tests and builds; claim PCC-20261125-1afa6b is unsigned. The new stub checks
public dispatch, factor-variable order, dimensions and traversal; actual
panels supply whole-set comparisons. [pcc:unverified]

[Raw records](evidence/factorwise_extract_v1/), [summary](evidence/factorwise_extract_v1/summary.json),
[reproduction](FACTORWISE_EXTRACT_REPRODUCE.md). Completed RURs, exact prefixes,
forms and completed-chart rows are retained. Failed-child reasons are present;
the shared wrapper does not archive separate child stderr or partial timeout
files. Base/reference setup and final JSON emission are outside the query
deadline. No recovery, EKS run or timing comparison is represented.

The [prior tangent audit](FACTORWISE_RESULT.md) explains the local rank
repair, while these outputs keep the larger completion gate open. The full
map fibers still include nonliftable abscissae. Next investigate replacing
each with its liftable-base factor, as the production comparator already does
for the whole base. Establish exact coverage and charge its construction
before another fixed-limit comparison. This is a proposed follow-up, not an
implemented or measured improvement.
[pcc:unverified]

The protocol's quotient-product and PKC 2016 sources remain the prior-art
context; KB was unavailable. Formalization and complete-cost recovery remain
open. Earlier reports and performance graphs are unchanged. These three raw
files join the five earlier files awaiting required artifact upload/replay;
the last observed store failure concerns the earlier five.
