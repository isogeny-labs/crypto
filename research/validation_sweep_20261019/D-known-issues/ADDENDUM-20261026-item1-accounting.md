# Item 1 addendum (2026-10-26): what the cross-ISA result means on `main`

Qualifies [`item1_cross_isa_20261020/RESULT.md`](item1_cross_isa_20261020/RESULT.md),
its [rerun](item1_cross_isa_rerun_20261020/RESULT.md) and item 1 of
[`REPORT.md`](REPORT.md) / `verdicts.jsonl`. Nothing there is edited; every
measurement stands. What changes is how the cause is named and what is
recommended.

## 1. The cause, stated precisely

Both runs built the recorded commit `2245b75`. There, `eager_rewrite()` was
`KIC_F4_EAGER != 0 && wordwise_rewrite()`, so BMI2 availability decided
whether specialisation was **eager** (all kept rows rewritten at once, charged
per row) or **lazy** (each row rewritten when first read, from wherever it was
last current). The Apple M4 Pro (no BMI2) recorded the lazy charge; a BMI2
x86-64 host charges the eager one. The reports' wording, "the BMI2 word-wise
rewrite charges a different count than the portable bit-scatter path", names
the coupling, not the quantity that differs: it is **eager against lazy
accounting**. The one-line patch worked at `2245b75` because switching BMI2
off also switched eager to lazy there.

## 2. The recommendation was already done on `main`

"Make the charge path-independent" was written against `2245b75` and was not
checked against `main`. Since #2043 (2026-10-17) every host is eager (`pext` /
`pdep`, or shift runs without BMI2) and the two eager paths charge identical
counts. `rewrite_paths_charge_identical_work_and_build_identical_rows`
(commit `77f20cd94`) asserts every `InheritCost` field and every row. It was
run again on 2026-10-26 on `main` (`71dd7db5c`): 1 passed. No production path
on `main` depends on the host.

## 3. What remains, and what was and was not done

- **Archived sessions recorded at `2245b75` on a host without BMI2** carry the
  lazy charge and cannot replay on `main`, on any host. That is a definition
  change, not a host dependence; see
  `problems/ecbench-claim-n13-replay-accounting-version-drift.md`.
- **Done (this change):** sessions now record the version of the inherited-F4
  accounting they were made under (`Session.accounting_versions`,
  `INHERITED_F4_ACCOUNTING_VERSION = 2`), the audit names that version when an
  F4 record fails to replay, and a test pins the charge so it cannot change
  without a bump. A session without the field is reported as predating
  versions, not assigned one. Method identities are unchanged: they hash the
  configuration, and adding a parameter would change the id of every
  committed F4 method.
- **Not done: making the lazy mode charge the eager amount.** The eager charge
  of a row is its parent-layout words read plus the words from the first
  nonzero word of its child-layout image to the end. Lazy mode never builds
  the child-layout image of a row nobody reads, so that figure is not defined
  for it without running the eager rewrite as a shadow, which removes the only
  reason the lazy mode exists (a same-binary performance control). It would
  also not make `claim-n13` replayable: its record holds the lazy charge.
  The problems record reaches the same conclusion ("no single charge
  reproduces both").
- **Superseded:** the item 1 action "add `target_arch` to the method
  fingerprint" in `verdicts.jsonl`. With no host-dependent path on `main`, the
  accounting version is the field that matters.
