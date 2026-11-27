# Full-cutoff extraction: complete answers and retained failures

2026-11-25 · `full_cutoff_extract_v1` · producer-local, unsigned.

Full-cutoff completes **7 of 13** fixed full-domain answers; the unchanged
cofactor-msolve comparator completes **8 of 13**. Every accepted signed set
equals the full Direct reference. Full-cutoff adds an F127 refusal at the
two-image chart `[7,7,82,82]`; both methods remain incomplete on all five
degree-32 targets. This is an output-completion result under fixed limits,
with no recovery or cost ratio. Claim PCC-20261125-1df51d remains unverified.
[pcc:unverified]

## Paired output controls

| Fixture / selector | Targets | Full-cutoff exact | Cofactor exact | Full-cutoff failure |
|:--|--:|--:|--:|:--|
| F127 / degree 16 |4|3|4|16 fixed forms refused on one chart|
| F8191 / degree 16 |4|4|4|None|
| F8191 / degree 32 |5|0|0|Common query deadline on every target|

The three panels emit all 26 planned records and exit 0 with empty panel stderr.
There are 15 exact and 11 undetermined query records, no mismatches and no
panel retries. A process exit 0 does not mean all its queries completed.
All required input-prefix and successful-RUR captures fit the frozen allowance.
[pcc:unverified]

![Complete answers and the retained degree-32 chart prefixes. Grey areas show incomplete planned queries.](full-cutoff-extract-counts.pdf){width=100%}

The full-cutoff failure at F127 target (116,69) occurs after 7 of 36 charts.
Chart index 7, with images [7,7,82,82], returns the exact nonseparating-staircase
refusal for all 16 prescribed forms. Its seven earlier charts produced three
rows, which remain in the record but are not accepted as an answer. The
cofactor comparator returns all ten reference rows for this target.
The shared parser's conservative completeness guard is unchanged.
[pcc:unverified]

On F8191 at degree 32, full-cutoff finishes 26/27/27/27/28 of 119 charts before
the shared deadline. Two prefixes contain one row each; neither is accepted.
The comparator completes zero of its one required system on every target.
The last attempted full-cutoff chart receives only the deadline's remaining
time, so these records do not measure its isolated cost. [pcc:unverified]

\newpage

## Target-by-target record

| Target | Reference rows | Full-cutoff result | Completed charts | Cofactor result |
|:--|--:|:--|--:|:--|
| F127 (20,91) |9|9 exact|36/36|9 exact|
| F127 (116,69) |10|Nonseparation|7/36|10 exact|
| F127 (41,95) |13|13 exact|36/36|13 exact|
| F127 (41,32) |13|13 exact|36/36|13 exact|
| F8191 d4, 17G |0|0 exact|18/18|0 exact|
| F8191 d4, 43G |1|1 exact|18/18|1 exact|
| F8191 d4, 89G |0|0 exact|18/18|0 exact|
| F8191 d4, 152G |0|0 exact|18/18|0 exact|
| F8191 d5, 17G |1|Timeout|26/119|Budget exhausted|
| F8191 d5, 43G |5|Timeout|27/119|Budget exhausted|
| F8191 d5, 89G |2|Timeout|27/119|Budget exhausted|
| F8191 d5, 152G |3|Timeout|27/119|Budget exhausted|
| F8191 d5, planted |4|Timeout|28/119|Budget exhausted|

These are the frozen existing targets. F8191 uses G=(0,789); the planted
target is (4375,6159). The curves and exact schedules are in the
[protocol](FULL_CUTOFF_EXTRACT.md). Reference counts are not supplied to either
algebraic extractor. [pcc:unverified]

![Acceptance requires completion of the entire catalog.](full-cutoff-extract-flow.pdf){width=86%}

## Interpretation, provenance and next step

The [prior finite audit](FULL_CUTOFF_RESULT.md) established point-set coverage
and located rank-two bindings in two-image charts. The new refusal occurs on
that type of chart, and its algebraic cause remains unresolved from the solver message alone. Increasing the number of fixed forms or discarding failed branches
would change this protocol. The next construction question is factorwise
divisibility within each fine-image fiber: first establish its coverage and
local algebra, then test it with the same complete-output and cost requirements.

Source **42728dba83**, before-call identity **d181477253**;
[raw records](evidence/full_cutoff_extract_v1/), [summary](evidence/full_cutoff_extract_v1/summary.json),
[reproduction](FULL_CUTOFF_EXTRACT_REPRODUCE.md). All before-call hashes remain
unchanged. The driver passes 39 feature tests and 12 default tests and builds;
claim PCC-20261125-b0dd5b is unsigned. The three new stub tests check completion,
prefix rejection and refusal/capture controls; the actual panels supply the
full-set comparisons. [pcc:unverified]

All completed RURs and required prefixes are retained. Failed child calls
retain their reasons; the shared runner does not archive separate child stderr
or partial timeout files. Base/reference setup and final JSON emission lie
outside the query deadline. No timing comparison, scalar recovery, EKS run,
independent signature or canonical export validation is represented.

The frozen catalog argument, PKC 2016 and first-collision study cited in the
protocol remain the prior-art context; KB was unavailable. Earlier figures
and results stay unchanged. No performance leaderboard graph changes. Three
new raw files join the two earlier files awaiting the repository's required
artifact migration; [development checks](DEVELOPMENT_CHECKS.md#artifact-migration-pending)
records its actual access failure.
