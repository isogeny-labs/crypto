# Maximal-order twist seeds: executed finite construction test

[Results, editable graphs and inspected PDFs](../reports/generators-20261027/RESULTS.md).
All **120** complete base sets pass native and independent PARI replay;
all **60** archived baseline sets match, and all **40** zero-shift sets stay
empty. Nonzero-shift fills change from **35/40** to **39/40**. This is an exact
construction change, not a measured solving or IC speedup.

The p827 fixture `icv1-fp10-t5-192cb216` has source group order823. Its twist
has invariant factors **[7,119]**, cardinality833. The original seed has order7;
the new maximal seed has order119. Native exhaustion and GP ellgroup agree.
Finding/certifying that maximum in this prototype scans827 x values and
performs416 point-order tests; those activities are retained and unpriced.

The p10111 first seed has order2029 versus maximal10145. The remaining
underfilled case, `icv1-fp6-tm7-fc20da94`, p59, shift3,B16, retains14 coordinates
after all52 nonidentity terms. No refill or sample pruning. All ten original
source/twist rational cardinality gcds are1; no source-generator scalar transfer
is supplied. New bases differ, so their costs require a separate full comparison.

## Evidence and boundaries

* [Pre-data successor protocol](PROTOCOL.md), [structured freeze](protocol.json),
  [source/binary/input pins](FROZEN.json), scientific source
  `f98f2935d5b7ccc41b10c4da5a7839b9225910a0`.
* [Strict native/CAS admission](ADMISSION.json) and actual
  [GP receipt](pari-replay-v1/RECEIPT.json). GP independently verifies both
  cardinalities, source generator order, canonical seeds, maximal exponent,
  invariant factors, every base and both fill aggregates.
* [Unified actual two-execution export](experiment.export.json): full
  protocol/schema/file binding passes with zero findings. Two executions
  check the same panel; their cases are not pooled as independent targets.
* Original [full-order candidate failure](../execution-v6/README.md) remains
  separate. The new maximum-order rule applies to **every** original fixture,
  including noncyclic groups, and was frozen before successor data.
* Native/CAS p7 prime, cyclic and noncyclic controls, source-generator and
  zero-shift controls pass. Full native release package tests pass serially.
  Six unchanged incoming root regressions remain in
  [ROOT_CHECKS.md](../execution-v4/ROOT_CHECKS.md); no full-root green claim.

No time, GAE, external speed ranking, useful-row rank, full scalar recovery,
novelty or NIST claim. L0 existing managed host and authorized local fallback;
native600s/4GiB/64MiB per file, GP300s/4GiB with64MB initial/4GiB maximum stack.
All E28 final labels32–63 remain unopened. Earlier full campaigns stay partial.

## Reproduction and artifacts

Raw `runs.tar.xz` is **7,364 bytes**, SHA256
`d4f6b99b131b0e9c82646b0aff3ff014b5bd2cde22e51ebc761d621c918039d0`.
The maintained Setec manifest and [fresh download](RAW_VERIFICATION.json)
verify all **5** file members and identical deterministic repacking.
Source/figures/PDF remain reviewable; large PNG copies use their manifests.

Hydrate `execution-v7` with `tools/artifacts/artifacts.py pull`, check the
archive checksum and extract. Also hydrate/extract execution-v4's raw archive
for the original ROWS.json input. Build the frozen native source with Rust1.90.0,
pinned Cargo.lock and `--release --offline --locked`; verify FROZEN.json hashes.
Use fresh output directories for `produce INPUT RAW`, the recorded GP command,
`admit RAW RECEIPT OUT.json` and `export V OUT.json`. Actual command/environment
and limits are in the producer/GP receipts. Do not replace a pinned source or
modify old raw data when comparing the separately frozen experiments.

Next cold-cost and explicit-kernel experiments have concrete
[blocked, unrun cards](../../../backlog/XP-20261027-twist-maximal-seed-cold-cost/README.md)
and [map criteria](../../../backlog/XP-20261027-twist-kernel-isogeny-orbit/README.md).
Their existence does not report them as running or allocate funding.
