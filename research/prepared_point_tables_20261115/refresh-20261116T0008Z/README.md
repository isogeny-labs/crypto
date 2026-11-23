# Current-main native audit integration

A normal merge preserves main's admission/dimension audit/control additions and
the existing point-table algorithm byte-for-byte. New inputs differ from old
frozen manifests, so this directory carries a fresh full source manifest rather
than relabelling old evidence. The original archives/manifests remain intact.
[pcc:unverified]

Exact combined source passes 167 locked release all-target cases, formatting,
strict all-target clippy and Rust1.78 all-target compile. The manifest also
includes the already tracked Rust hash-preparation example. These are software
controls; no runtime, signed execution or general audit verdict. [pcc:unverified]

[qualification.json](qualification.json) pins actual source/merge/check identities.
[native-source.sha256](native-source.sha256) checks current native inputs from
the repo root. [development.tar.xz](development.tar.xz) retains complete raw
check output, merge state and retention tooling; extract into a fresh directory,
then run `sha256sum -c SHA256SUMS` inside `development/`. [pcc:unverified]

[journal-retention.json](journal-retention.json) pins complete original Git
journal blobs. Full normalized blocks, multiplicity and each source sequence
are retained; only conflict-marker lines and outer block newlines normalize.
Timestamp priority is used only among independent ready blocks. The helper was
applied only to the actual old-date journal conflict; today's log is separate.
The parent archive also records ordinary problems-index row conflict retention,
actual ruleset readback and the unrelated scheduled autofix provider failure.
[pcc:unverified]

The [inherited applicable records dependency](../../../problems/point-table-crypto-applicable-ci.md)
remains open. Auto-merge is off pending actual new-head checks, protection and
blocking review. Canonical PCC history is the incoming base/main, with the
previous head retained as a complete byte prefix. No producer row, rechain,
waiver, old archive rewrite, experiment, image build or scientific workflow.

All terminal Cryptanalysis PRs and the separate ECBench study remain outside
this tracker. Current source/controller checks do not establish runtime gains.
