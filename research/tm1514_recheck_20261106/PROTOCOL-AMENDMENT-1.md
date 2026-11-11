# tm1514 recheck: AMENDMENT-1 (counted-only; C3 deferred)

**Status: FROZEN** at 2026-11-10T17:17:31Z (`PROTOCOL.json` version 2, `protocol_sha256`
`d348123a53bf02b49597bbfffc9f4337a24708caf1897cdb14b5550a69c081d6`), before any record of replicates A3 or B3 existed.
Approved by the owner on 2026-11-07 ("Approved"); execution route set by the
owner's "Resume" of 2026-11-10 (local counted sessions). Filed before any data of this
study was read: replicate A's records were never downloaded or opened
(`filed_before_affected_data: true`).

**Prior freeze (recorded).** `protocol_sha256`
`8cd5f85644d8091a5809df416fdc2ea5e449c82ac11d08da99fe14adcba9d645`, base commit
`363306c646953dbbe97fd6d611e10417f5c4dc59`, `frozen_at` `2026-11-06T17:52:16Z`.

Where this file and `PROTOCOL.json` disagree, this file governs.

## 1. Reason

Replicate A (GitHub run 42269476702, job `gh-42269476702-1`, request sha256
`0be570bd4b0aade2abb6789075878c56bc1ab0d109f197c2b48cfc98abaa134d`) ended with
launcher state `failed`, exit 23, no pod UID and no results URI. The private
receipts were not read, so the cause is inferred, not verified: the L2 gate
refuses a session when any execution solves in under 1.0 ms
(`research/ic_line_layout_tournament_20261031/DECISION-V6-REDESIGN-20261106.md`
§2, "The 1 ms floor"). Development solves of this design took 0.19–0.57 ms per
arm (`devcheck/`). The frozen design required admitted L2 wall time, which this
instrument cannot give at k = 24.

## 2. Changes

1. **Replicate A is a dry run.** Its records are not evidence for any claim and
   are not read for this study. Its seeds are not reused.
2. **Counted-only.** C0, C1, C1b, C2 and C4 are decided from operation counts
   (`cost.total_gae` and the priced native counters through `band::priced_s`),
   which do not depend on the host. No wall time enters any of them. L2 admission
   and the A/A wall admission are not required; exact replay of every record
   (`ecbench verify --replay-all`) and the C0 twin identity are required.
3. **C4** is reported at w = 0, 0.1 and 1 only (the §5a band). The measured-price
   variants are dropped with C3. At each w the verdict is "R_tm1514 above 1" if
   the 95 % interval of R_tm1514(w) lies above 1, "R_tm1514 below 1" if it lies
   below 1, and "unresolved (interval contains 1)" otherwise (same bootstrap,
   same invalidation by C0 or C2).
4. **C3 is deferred** to a separate protocol with an instrument whose timed
   samples each last at least 5 ms (for example, thousands of negation
   canonicalisations or walk-table lookups per sample). It is out of this
   study's scope. The repository's host-free work model (`ecbench work`,
   bitop/v1) is reported beside the band as a model, labelled as such, never as
   a measured price.
5. **New replicates A3 and B3, with new targets.** Two fresh replicates, A3 and
   B3. Each seed is `int(sha256(D)[0:16 hex]) mod 2^53`, with D the UTF-8 string
   `crypto:research/tm1514_recheck_20261106:<suffix>`: the form that gives
   version 1's seeds (suffix `measurement.seed:A` gives 960559345608509).
   - `measurement.seed:A3` gives A3 `8671006997891351`.
   - `measurement.seed:B3` gives B3 `6232168080843969`.
   - `workloads.target_seed:3` gives the new `target_seed` `546787239895877`
     (version 1: suffix `workloads.target_seed`, 1406057239440944).

   Curves, arms, 96 targets per curve, 6 rounds and 1 warmup are unchanged.
   The specs are `spec-tm1514-A3.json` (`ecbench plan` spec id
   `ECS1h3d4c0d4813f4`) and `spec-tm1514-B3.json` (`ECS1h58d3eafbe575`), 16,800
   executions each. They differ from `spec-tm1514.json` in `label`,
   `description`, `workloads.target_seed`, `measurement.seed` and
   `measurement.isolation_required` (L0). Method ids are version 1's. A3 and B3
   share their 480 workloads, and none of them is a workload of version 1,
   replicate A or any development check.

   **Replicate A2 is withdrawn.** An earlier draft of this amendment named
   replicates A2 and B2: suffixes `measurement-A2` and `measurement-B2`, seeds
   96725836575979 and 2732357890408474, version 1's `target_seed`, spec ids
   ECS1h02235b4f381e and ECS1h6856e77660ce. Development checks of the amended
   analysis (2026-11-10) ran reduced copies of those specs with those seeds and
   targets, so they produced exact rows of the planned A2/B2 sessions (100 and
   1,200 measured rows per replicate), and the preparing session saw a summary
   of them. No data of A2's or B2's full sessions was ever produced. Those
   seeds and targets are not used. The dry-run rule for replicate A (item 1) is
   unchanged.
6. **Execution route: local L0 counted sessions** in the session container,
   on the owner's instruction of 2026-11-10, a one-off departure from the EKS
   default. Each replicate is one `ecbench run` session of a locally built
   release `ecbench` (one binary for both, its sha256 in each `host.json`),
   with `isolation_required` L0: a development host, stated as such. Each
   session is audited with `ecbench verify --replay-all`, which must report no
   problem and every replay reproduced. No wall time, cycle count or isolation
   level enters any claim. Zero automatic retries; a killed or incomplete
   session contributes nothing and is reported.
7. **Analysis.** `examples/tm1514_analyze.rs` runs with C3/C4-measured outputs
   disabled and without the wall-admission inputs (`--wall-a`/`--wall-b`
   removed), C4 at the band weights only, and the frozen spec ids and replicate
   labels set to A3 and B3. The change is limited to that. Previous sha256
   `59b83670a1fc7a7cd2c41b761fa570d0cb9fb18e6409b9f8a4c8ebc04313b6f2`; new
   sha256 `6534cfe188a2e48dd1286af37b73006c85a579687102ed0cfbd90adf1f91f197`.
   It is run as `tm1514_analyze --a sessions/A3 --b sessions/B3 --out analysis`.

## 3. Unchanged

Question, hypotheses H1 and C1b, curves, arms (rho-neg, rho-neg-aa, inc, inc-aa,
rho-plain), 96 targets per curve (now drawn from the new `target_seed`, item 5),
6 rounds plus 1 warmup, the target law (public hash-to-subgroup), bootstrap seed
244596609222896, B = 10,000, the C1 rule (98.33 % interval of D alone decides),
C1b, the C2 detection rule, the verdict vocabulary of C0, C1, C1b and C2, and the
USD 5 ceiling (one attempt spent). `target_seed` changes (item 5).
