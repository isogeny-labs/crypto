# Authoritative held-out handoff, 2026-11-15 UTC

This supersedes the prepared-screen note in .resume/RESUME.md. Do not restart the terminal
screen. The shared resume path is currently reserved by another task, so this study-local
handoff and the dated session log carry the current state.

## Terminal evidence

Protocol1b618d864a precedes runner2404e34dca. All20 attempted dense fixtures selected,
all100target gates passed, all1500timed cells completed with correct scalars. No experimental
cell was capped, invalid or unstarted. Gates check265559principal and4449projected anchor
identities. Receipt collection validates source, args and input SHA before native jq summaries.
All100A/A pairs agree on deterministic workload/target fields. [pcc:unverified]

Primary comparison is per target: fastest of12control times / compact cold total. Five-target
curve medians range0.9433..3.1684; individual targets reach4.1460. At g2p65537, the two family
medians are1.5731 and2.0521; g3p1009 has0.9433 and1.4699; g4p127 has2.6455 and3.1684.
No10x result is established. A/A spread reaches3.0890, so L0 observations do not imply
controlled statistical confidence. Retain losses and the separate-target accounting.

## Files and reproduction

- HELDOUT-RAW.tar.gz: complete native child stdout/stderr/receipts, fixture manifests and logs.
- heldout-evidence/summary.json: all cases, all15arms and phase summaries (also in archive).
- heldout_collect.rs and heldout_summary.jq: native custody and numerical result generation.
- heldout_render.rs -> heldout.svg and HELDOUT-REPORT.typ; native Typst -> HELDOUT-REPORT.pdf.
- benchmarks/records/20261115T0700Z-hec-heldout-scaling.json: closed record and raw hashes.
- Existing shared benchrec tool handles generic ledger/custody/progress metadata only. Initial
  selection's legacy wrapper provenance is retained; scientific execution and new result
  generation use Rust/native jq/Typst. Earlier Python visual reports remain historical evidence.

All12standalone tests passed, plus12library tests. Receipt positive/replay/input-rejection/
timeout controls passed. Cargo now exposes the heldout example and CI runs its12tests and
receipt controls. Scientific timing binary remains source2404e34dca with pinned library hash.

## Decision and next mathematical action

Goal stays full mathematical>=10x for genus2+. This screen does not justify equivalent
arithmetic rewrites or any novelty/exponent assertion. H-wide dense forest work grows to31.7%
of total at g2p65537; dense anchor solve reaches17.5% at g3p1009 (medians of per-solve fractions).
Relation generation remains47.8..68.5% at the largest size per genus. [pcc:unverified]

Next: derive a cost case that changes relation generation AND recombination/anchor algebra.
Screen sparse short-cycle graph representations and the underlying Riemann-Roch relation
family before prototyping. A forest-only or solve-only optimization cannot be assumed to
deliver10x; compute its per-case remaining-cost bound first. Freeze the successor's proof
obligations, complete-cost target and falsifiers. Independent controlled replay, external
comparison and mathematical novelty review remain outstanding. No leaderboard promotion.

No experiment is running. Remaining operational work: commit/package evidence, extend PR4274,
follow current-head CI/S3 custody, checkpoint this handoff. Never edit prior closed records/raw.

## Unsigned claim receipt
PCC-20261115-38951c records the held-out numerical assertions with a pinned compact raw
observation table and a separate native jq recomputation. `pcc verify` returns not_covered;
`pcc standing` returns unverified because the producer report has no independent signature.
This adds custody and an explicit claim record; it does not promote any scientific standing.

Before publication, the sparse-checkout ledger omission was corrected: all1057original ledger
entries are preserved byte-for-byte, followed by the claim and an incident note. The captured
unpublished erroneous entry is UNPUBLISHED-LEDGER-INCIDENT.jsonl. Shared ledger chain/prefix
checks pass with1059entries. No previous claim or verdict was removed or changed.
