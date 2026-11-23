# Native elimination and high-genus cover audit

The separate [high-genus cover review](COVER-REVIEW.md) constructs an algebraic
subgroup-transfer family meeting EGT's geometric conditions, but bounds its
standard full projected base below by p/(4delta) columns. This is a scoped
cost obstruction for that explicit method, with an
[inspected visual report](cover-review/report.pdf); smaller implicit bases
remain open algorithms. PCC-20261115-c4aaf8 is unverified. [pcc:unverified]

The first fixed auxiliary panel certifies all eight empty ideals; all four
positive cases retain UNKNOWN at the reduction cap. Adding standard chain
filtering and leading-degree-first reducers jointly leaves all twelve cases
UNKNOWN at storage limits. Six small exact ideals and the preserved natural
Kummer norm complete in both rounds. No positive auxiliary source certificate
or new cold recovery is obtained here. [pcc:unverified]

| Frozen round | Six small ideals | Natural Kummer, D4 | Auxiliary quartics, D6 |
|---|---|---|---|
| qualification001;7f7ce5112e |6 complete/match|Complete, quotient1|8 certified empty,4 unknown|
| qualification002;96f1cc3b7a |6 complete/match|Complete, quotient1|0 complete,12 unknown|

OpenSSL replays each twelve-target status table and compares completed model
sets where present. In001 the completed auxiliary sets are all empty. In002
none completes. Both controls retain all four positive targets; their models
are never counted as recovered. Unknowns establish neither ideal
non-membership nor a degree lower bound. [pcc:unverified]

The native producer schedules standard Buchberger critical pairs by increasing
leading-monomial LCM degree. It retains coprime-leading-term skips and defers
pairs above D. Source, variable products and scalar combinations produce the
existing PC-DAG-CERT-1 format. The separately compiled unchanged checker
reconstructs every accepted polynomial, checks bounded node degree, all final
S-pairs and original-generator inclusion before extraction. A candidate with
deferred pairs is accepted only if that complete check passes.
[pcc:unverified]

Round002 records represented pairs for the standard chain criterion and
changes reducer preference at the same time. Both alterations are tested as
one implementation; no single-change attribution or speed result is made.
Current produce.cpp is this A1 version. Use source7f7ce5112e to reproduce the
earlier version. Historical source, qualifier and binary hashes stay bound to
their own archives, rather than the current template. [pcc:unverified]

The positive001 failures occur at1,000,001 recorded reduction-loop steps
(including irreducible-term visits). The002 auxiliary failures are stored-entry
or active S-row limits near2,000,000 charged slots. These outcomes motivate
closing this particular schedule refinement without another cap increase.
The [conditional envelope](COST.md) still needs a sufficient degree and useful
positive yield on an admissible growing family. [pcc:unverified]

Reproduce the fixed software panel at the selected source in fresh paths:

    research/critical_pair_norm_20261115/build.sh PREFIX NEW_BUILD
    cp research/critical_pair_norm_20261115/qualify.sh NEW_FROZEN_WRAPPER
    timeout --signal=KILL 1200 bash NEW_FROZEN_WRAPPER NEW_BUILD NEW_OUTPUT

PREFIX is the parent's pinned FLINT3.1.3 native prefix. All five C++ binaries
build with strict warnings; the verifier, comparator, auxiliary equations and
OpenSSL reference sources are unchanged. The qualifier uses fixed six-control,
Kummer D4 and auxiliary D6 inputs. All original-equation lifts and complete-set
comparisons remain. No timing, fitted exponent, security dataset or general
prime-width extension is included. [pcc:unverified]

Scoped result statements PCC-20261115-555122 and PCC-20261115-c012b0 remain
unverified; the prospective statement is PCC-20261115-df00a3.

Every round retains nineteen build bindings and nine separate pre-call
wrapper/source/binary bindings; all match at its completion. Its whole raw
archive includes inputs, output, proofs, exits, unknown diagnostics and replay.
Normalized unpack/repack is byte-identical. [pcc:unverified]

| Whole archive | Bytes | SHA256 |
|---|---:|---|
|evidence/qualification001/runs.tar.xz|314744|bedb93ed6cdfd9c259d1a85e804d5454228289ef2920439ad773b917855021fa|
|evidence/qualification002/runs.tar.xz|82016|1ff6979f864247b4aceb3d0f15e712ad073eb4e3ed48343bc8246d8f7f4cce80|

Artifact-autofix run43720392567 uploads both whole objects and replays both
remote objects in full. Bot0683ddd111 retains their SHA256/length manifests
and ignore rules. Both exact local hydrated copies verify with zero problems;
hydrate either directory with tools/artifacts/artifacts.py pull before reading
runs.tar.xz. Extract it in a fresh directory to read runs/. No shards or
evidence replacement. Both
[round001](reports/qualification001/report.pdf) and
[round002](reports/qualification002/report.pdf) reports have two inspected pages,
recorded-count graphs and the [editable pipeline](pipeline.dot).
[pcc:unverified]

Sources checked (KB unavailable): the wrapper has no executable; read the
existing indexed-PC/source checker, initial-elimination and positive-degree
reviews, then Berthomieu--Eder--Safey El Din,
[msolve](https://arxiv.org/abs/2104.03572), Theorem3 and section3.1. The author
F99a PDF/PostScript return503; no unread full text or novelty claim is made.
The [protocol](PROTOCOL.md) and [publication record](PUBLICATION.md) retain
prospective source freezes, actual failures and the native development scope.
Canonical leaderboard/performance records remain unchanged.
