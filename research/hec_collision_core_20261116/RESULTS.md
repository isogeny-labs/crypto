# Joint core size, full-degree sources and exact target observability

The joint source/target/LP solver now passes all12distinct source/target cases at its original H13/36/22rule, including full-degree public sources in genera2,3,4. Smaller cores recover only four additional policies, all genus3/H18. An independent full-catalogue rank replay confirms that every one of the32retained noncompletions has an undetermined target in the admitted equations. This is bounded correctness/rank evidence for established algebra, not a mathematical novelty or tenfold performance claim. [pcc:unverified]

## Frozen panel

Original source273d1c564833, pre-run checkpoint20261116T163306Z; once-only180second watchdog run exits0/empty stderr. Input curve families/targets reuse the preceding panel: g2/F251/c11/ell61667, g3/F211/c12/ell4620611, g4/F61/c2/ell16790591. Source role0isD1; role1is[37]D1. Both source roles use the SAME original public targets. Relative expected scalar k0/37 is computed outside the solver API. Role1has actual degrees2,3,4, with prime source order preserved; no target/source replacement occurred. Full identities are in INPUTS.txt. This is a reused-field, deterministic-source panel, not a fresh population holdout or arbitrary-source proof.

Four H policies per input: g+1;8/9/10;ceil(original H/2);original H13/36/22. Source/target shifts, offset equations,8additions/input,1024unique principal tuples/4096draws, prepared tails, original group checks and independent sparse replay remain unchanged. Prefix membership differs from Compact. All48candidate policies and12Compact controls are retained.

## Exact finite outcomes

| Genus / field | H | Cases | Verified | Missing input row | Other no-k | Verified prepared-output range |
|---|---:|---:|---:|---:|---:|---:|
|2 / F251|3|4|0|0|4|—|
|2 / F251|7|4|0|0|4|—|
|2 / F251|8|4|0|0|4|—|
|2 / F251|13|4|4|0|0|192–416|
|3 / F211|4|4|0|3|1|—|
|3 / F211|9|4|0|0|4|—|
|3 / F211|18|4|4|0|0|1576–2208|
|3 / F211|36|4|4|0|0|768–896|
|4 / F61|5|4|0|3|1|—|
|4 / F61|10|4|0|3|1|—|
|4 / F61|11|4|0|1|3|—|
|4 / F61|22|4|4|0|0|912–1264|

Totals:16successful policies,10missing-input-row outcomes,22other no-k outcomes. These are12distinct inputs tested at4sizes, not48independent targets. The16successes include all12original-size cases plus four genus3/H18cases; eight successes have role1full-degree sources, spanning six distinct full-degree-source inputs. All12Compact controls recover. Every candidate success passes original-source verification and independent solve_fast over pure core rows.

Every success reaches full core rank H+2. Thus this panel supplies no example where the target is recovered while leaving an anchor-core direction free. Genus3 halves H from36to18(core columns38to20), but prepares1576or2208principal outputs instead of768–896atH36. This is a generation/rank tradeoff, not a time ratio or a tenfold total-cost premise. All smaller genus2/genus4policies fail under the frozen bounds.

## Separate identical-stream rank replay

Replay sourced34e82f88f19, prior checkpoint20261116T163558Z, once-only watchdog exits0/empty stderr. It retains every admitted raw row in the FULL catalogue+source+target coefficient space before forest elimination. Native dense modular elimination computes rank(A), rank(A with target column deleted), rank([A|rhs]) over actual ell. All48systems are consistent. RankA−rankDrop=1on all16successes and0on all32noncompletions, matching solver outcomes exactly. This checks admitted equations independently of the forest's core output; rejected>2endpoint rows remain outside the matrix.

For a consistent system A_other*x+a*k=b, if a lies in the span of A_other, a null vector changes k while preserving all equations. Rank difference0 therefore proves k is not fixed by that finite admitted row stream. If a is outside that span, rank difference1 fixes k. This is an elementary exact rank criterion, not a universal lower bound on index calculus or other samplers.

All24original output fields reproduce byte-for-byte across all48policies; control and input files also match byte-for-byte. Source-role0at the old size rule reproduces the preceding shifted solver's deterministic count vectors for all6inputs. The rank replay is additional validation of the same streams, not new independent samples or a wall-time A/A test. Both freezes identify source/import/lock/binary versions. Original Cargo bytes are preserved separately because adding the replay binary changes Cargo.toml.

Several smaller-core failures exhaust the entire chosen principal family, not just an arbitrary draw cap. For example genus2H7/H8visit all35/56unsigned triples, and successful input-row genus4H11cases visit all462unsigned5tuples, each complete sign orbit. For those fixed input rows and admitted family, a different LA ordering cannot create the missing target constraint. New target rows, anchors, primitive functions or a changed endpoint policy would be additional data/algorithm changes, not a solver repair. Missing-input-row cases have no target coefficient in their collected constraints.

## Cost and decision

Core size alone does not establish complete cost: scans/shifts, unsuccessful input pencils, duplicate tuple draws, all prepared outputs/tails, roots, graph/potential updates, matrix operations, allocations, catalogue/table/inverse setup and final verification remain costs. Selected counters are not a complete arithmetic inventory. Extra multiplier certificates, independent scalar replay and full-catalogue rank replay are correctness-gate work, not a production timing comparator. Controls differ in anchors/sampling, so no count-unit speed ratio is claimed.

The original H rule extends successfully to full-degree sources on this panel. Shrinking H fails across genera2and4and trades much more generation for fewer columns in genus3. The replay makes clear that this is a finite equation-supply issue, not a missing pivot in the forest/echelon implementation. No timing campaign or mathematical tenfold premise follows from core shrinkage here. A useful successor must change which inexpensive constraints are admitted or sampled, while paying for the changed off-anchor elimination, rather than only reorder the same insufficient rows. The full mathematical10xobjective remains active.

## Artifacts and scope

[Original protocol](PROTOCOL.md), [rank replay protocol/proof](CERT-PROTOCOL.md), [all48outcomes](CONTROL.tsv), [rank replay](CERT-CONTROL.tsv), [12complete controls](BASELINE.tsv), [full input identities](INPUTS.txt), [native summary](SUMMARY.tsv), [graph](COUNTS.svg), [three-page combined report](reports/run-20261116T1635Z/REPORT.pdf). Identical reports under bothrunids explicitly describe both executions; all pages visually inspected. Native reduce.rs summarizes fixed counters and checks reproduction; no scientific Python.

These are producer checks sharing native field/group code, without independent scientific validation, a machine-checked full group proof, timing benchmark or leaderboard admission. Prior sealed evidence/canonical timing graphs unchanged. Today's shared daily log is reserved by another verifying task; SESSION.md holds scoped handoff. Prior KB/primary-PDF evidence gaps remain explicit; no priority claim. PR remains unmerged; unrelated .resume/RESUME.md preserved.
