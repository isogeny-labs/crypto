# Joint refresh: complete journal/history retention

Normal merge retains all complete normalized journal blocks, repeated-block
multiplicity and every source's block sequence. Only conflict-marker lines and
outer block newlines are normalized; no other block bytes are changed. Every
raw journal is pinned by its original Git commit/blob/digest. Stable topological
ordering preserves each parent sequence, using timestamp priority only between
independent ready blocks. The exact sources and controls are in
[journal-retention.json](journal-retention.json). [pcc:unverified]

The exact combined native source has 169 passing locked release all-target
cases, plus formatting, strict all-target clippy and Rust1.78 all-target checks.
Its complete [manifest](native-source.sha256) is byte-identical to the original
feature manifest; native files and old receipts were not edited. These are
development controls, with no runtime, signed execution or general audit claim.
[pcc:unverified]

[qualification.json](qualification.json) names commands, source/archive hashes,
checks and scope. [development.tar.xz](development.tar.xz) retains complete
original decoded CI failure logs, local source/check output, merge attempts and
retention tooling. Extract to a fresh directory, enter `development/` and run
`sha256sum -c SHA256SUMS`. Old archives/manifests stay immutable.
[pcc:unverified]

The [applicable inherited records failure](../../../problems/point-table-crypto-applicable-ci.md)
remains a merge dependency. Auto-merge stays off; no required/applicable check
or blocking review is bypassed. New uploaded-head checks still need inspection.
PCC remains the incoming base/main history, retaining the old head as a complete
byte prefix. No producer row, artifact waiver, rechain or migration.

Both Cryptanalysis PRs1574/1577 have terminal merge/source/main-reachability
receipts in the parent refresh archive and need no further tracking. Existing
profile/pilot/challenge/audit obligations and the separate ECBench study stay
open/unchanged. No scientific job, image build, experiment, credential use or
local timing substitute was launched. [pcc:unverified]
