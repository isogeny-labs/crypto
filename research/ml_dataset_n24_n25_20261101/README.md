# n24/n25 paired-twist native dataset panel

This pre-data functional panel adds two field sizes absent from the retained n23/n47/n61 campaigns. The frozen [protocol](protocol.json) and [plan](plan.json) specify 12 method cells: two field sizes (24, 25), m=2, Koblitz/ordinary-binary/prime, both quadratic-twist settings, one curve seed, one generator, one target key and native `enumerate`. The original default 165,150,720-cell matrix remains a separate budgeted campaign.

The claim is exact and local: all 12 planned cells must retain terminal results, pass native artifact-hash and known-answer replay, and appear in native report and strict export. Every timeout, budget stop or partial must be retained and censored according to its native status. Candidate-test counts are eligible only when the native counter is complete. No timed, comparative, statistical, model-efficacy, FFD/LFD or whole-solve-instruction conclusion is preregistered.

This panel uses a deterministic seed and fixed order because it makes no between-arm comparison. Paired twists are distinct public instances. The exporter must keep whole-field-size split components linked across families, curves, generators, keys and twists. The two new sizes can become additional eligible split components only after their actual labels and component identities are independently checked; they do not alone authorize a training run.

The bounded local invocation is one attempt per cell, 30 seconds, 2 GiB memory, 64 MiB trace and 10,000 native work units per cell. Native per-cell persistence is the checkpoint. A failed persistence or replay check stops the invocation; no automatic retry changes the denominator. The initial run has no cloud request and no S3 claim. The full campaign still requires its current budget ledger, worker grant, immutable S3 custody and remote restore.

The construction context is [Semaev's summation-polynomial paper](https://eprint.iacr.org/2004/031.pdf), whose initial prime-field construction assumes characteristic other than two or three, and [On Index Calculus Algorithms for Subfield Curves](https://sacworkshop.org/SAC20/files/preproceedings/18-IndexCalculus.pdf), which derives Frobenius-invariant factor-base conditions for Koblitz curves. This panel reuses the native bounded enumeration adapter; it does not import either paper's asymptotic claims. The earlier [ordinary-binary n23 report](../ml_dataset_binary23_m3_15_enum_20261031/report/run.md) supplies the same paired-twist custody and censoring precedent.

Pre-data file SHA-256: protocol `00b56f3bca568dfa4bdfb2a4d7d4972d7b90f255f0f001f2428fb6dc1845c6fc`; plan `aaf4f3abb59fea7b5aad3e6059504e0ab59c11d84f031f786b2d7b447be86e19`. The canonical protocol body hash is `5e1b765211297eff92cd763c9ac34023dc7a9d2b9d71d024dbaf89f37d5aecdf`.

## Protocol admission correction before data

The native `dataset preflight` accepted v1 and recorded 12 planned, zero generated cells, but the independently fetched official `experiment_protocol.py validate` checker rejected v1 with 48 schema findings (exit 1). Its full output is [retained here](official-protocol-v1-check.log). The v1 freeze and bound plan are immutable pre-data evidence; no `dataset generate` command has run. A v2 protocol in the official schema and a new bound plan are required before generation.

## Official v2 admission

The versioned [v2 protocol](protocol-v2.json) and [v2 plan](plan-v2.json) preserve the 12-cell matrix and caps. The [official checker receipt](official-protocol-v2-check.log) reports zero findings, and the [native preflight receipt](preflight-v2.json) reports 12 planned and zero generated cells with clean source and binary provenance. These exact v2 files, rather than v1, bind any later generate invocation. Protocol v2 file SHA-256 is `f740120e8b185fbe1612cf2d88f5cbbb1efc3f0676b0341dbb228ee5700dd114`; plan v2 file SHA-256 is `dfb7ec2fdbc387724bae6fdb5a0c56657b2d09944726ed4808598ed5bcfa47de`; the official frozen canonical-body SHA-256 is `4b3650e18187860993b45926900c9ed26714d6188cef0b3c6119243a70e9cc0c`. No generation preceded this amendment.
