# tm1514 recheck: AMENDMENT-1 (counted-only; C3 deferred)

**Status: DRAFT.** It is frozen by the commit that re-freezes `PROTOCOL.json`
with this amendment recorded, before any record of this study is read.
Approved by the owner on 2026-11-07 ("Approved"). Filed before any data of this
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
   variants are dropped with C3.
4. **C3 is deferred** to a separate protocol with an instrument whose timed
   samples each last at least 5 ms (for example, thousands of negation
   canonicalisations or walk-table lookups per sample). It is out of this
   study's scope. The repository's host-free work model (`ecbench work`,
   bitop/v1) is reported beside the band as a model, labelled as such, never as
   a measured price.
5. **New replicates.** Two fresh replicates, A2 and B2, with new measurement seeds
   derived as before: `sha256("crypto:research/tm1514_recheck_20261106:<suffix>")`,
   first 16 hex digits, mod 2^53, suffixes `measurement-A2` and `measurement-B2`.
   Targets, curves, arms, rounds, warmup and `target_seed` are unchanged.
6. **Execution route:** _[to be fixed before freeze; see §4]_.
7. **Analysis.** `examples/tm1514_analyze.rs` runs with C3/C4-measured outputs
   disabled and without the wall-admission inputs. The change is limited to
   that; its new sha256 is recorded at re-freeze.

## 3. Unchanged

Question, hypotheses H1 and C1b, curves, arms (rho-neg, rho-neg-aa, inc, inc-aa,
rho-plain), 96 targets per curve, 6 rounds plus 1 warmup, `target_seed`
1406057239440944, bootstrap seed 244596609222896, B = 10,000, the C1 rule
(98.33 % interval of D alone decides), C1b, the C2 detection rule, the verdict
vocabulary, and the USD 5 ceiling (one attempt spent).

## 4. Open before freeze

The execution route. Counts are exact and host-independent, so either route
gives the same numbers; they differ in who can read the records.

- **EKS (repository default).** The isolab job will fail the L2 gate again
  (exit 23) after writing complete records. This amendment would accept those
  records for counts if `audit.json` is OK and every replay reproduced. Someone
  with lab S3 access must copy `runs/<job>/` into `sessions/A2` and `sessions/B2`.
- **Local counted sessions.** Run in the session container with
  `isolation_required` set to L0 (development host, stated as such), then audit
  with `ecbench verify --replay-all`. No S3 access needed. This departs from the
  owner's EKS default and needs the owner's explicit approval.
