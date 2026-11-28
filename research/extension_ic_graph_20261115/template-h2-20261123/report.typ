#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:0.45em)
= H2: exact degree-7 template, cost comparison open
24 November 2026 · Local Rust construction controls · Producer evidence UNVERIFIED

The degree-7 template returns a complete 40-polynomial grevlex basis on both saved cases. Original-generator membership, input containment, every S-pair and mutual containment with the saved unrestricted F4 basis pass. Elimination rank is 1652. This establishes bounded construction viability. It does not establish a cheaper relation finder. [pcc:unverified]

#image("row-counts.svg",width:100%,height:73mm,fit:"contain")

#table(columns:(0.45fr,1fr,1fr,0.55fr,0.55fr),align:(left,right,right,right,right),
[Degree],[Enumerated],[Retained],[Columns],[Accepted],
[5],[504],[451],[462],[0/2],
[6],[1260],[1057],[924],[0/2],
[7],[2772],[2184],[1716],[2/2])

The initial generic-cofactor construction completed its reference computation but refused the witness shape/budget gate before an online application. It recorded 33,418,328 learning products. The precise failed sub-bound was not retained. Every failure remains in the record. [pcc:unverified]

== The required-degree correction
Read-only Rust inspection found 40 minimal leading generators in each saved reference, with histogram 2:6, 3:9, 4:10, 5:9, 6:5, 7:1. The degree-7 minimal generator is x5^7. A degree-5/6 full-basis template cannot supply it. This should have been checked before the lower-degree runs. They are inadequate-degree controls, not evidence against H2 generally. The API now refuses such a bound before enumerating rows. [pcc:unverified]

== Arithmetic retained
#table(columns:(0.45fr,1.4fr,1.4fr),align:(left,right,right),
[Degree],[Case 0 elimination products],[Case 1 elimination products],
[5],[18,243,997],[18,240,089],
[6],[157,579,955],[157,588,350],
[7],[1,004,730,078],[1,004,673,424])

Degree-7 exact checking separately records 11,991,515 and 12,005,619 products. Comparison/input-validation counters remain separate. The two eliminations total 2,009,403,502 products and use 8,517,600 coefficient/provenance cells. Counts include explicit provenance-column arithmetic and inversion exponentiation. Memory work and other machine costs are outside this unit. No runtime or F4 speed ratio follows. [pcc:unverified]

#pagebreak()
== Construction and exact acceptance
Enumerate bounded original-generator multiples and omit rows reducible by principal syzygies f_i e_j - f_j e_i. This is a restricted syzygy-pruning construction, not a minimal basis of the full module. Freeze input leading monomials. A saved development F4 basis supplies only the required leading-ideal fingerprint; its coefficients never fill an online row.

Online modular RREF retains explicit original-generator witnesses. Accept only after exact membership, original-input containment and all S-pair checks. A matching leading ideal alone is insufficient. Refusals retain their attempted arithmetic and require explicit solver fallback; no refusal is a proved empty root set. [pcc:unverified]

The new module is exported at the existing F4 polynomial representation boundary. Terms are descending grevlex and monic. The basis can feed the existing action/root stage and Nagao callback boundary; a fully benchmarked callback/fallback and whole-IC comparison remain open. The production default is unchanged. Twelve exact native controls, release build and strict all-target/no-deps Clippy pass. This is standalone-source qualification, with current-main full-root syntax/fixture problems recorded separately. [pcc:unverified]

== Protocols and custody
All three calls were local, single-threaded, bounded to 8 GiB virtual memory, with 120/180/240-second outer limits. Native and launcher exits were 0, post-worker hashes matched and stderr was empty. No old scientific call was retried. The earlier H1 launcher discrepancy remains separate. [pcc:unverified]

Published source before each call:
- Generic cofactors: b6f1c7a3ad.
- Degree 5/6: 5c306fadb0.
- Degree 7: 8867bf797e.

Every case is from the same immutable saved input, using exposed cases 0 and 1. There is no new curve, target or population-level holdout. The saved reference is an offline development oracle, whose fresh construction cost is unmeasured here. A cold comparison must charge it, all learning, failures, exact checks, roots and graph materialization.

Original summaries, phases, source/worker pins, shape analyzer, native report analyzer and raw manifests accompany this report. Degree-7 explicit witnesses occupy 6,225,569 bytes; large witnesses/templates remain LOCAL pending artifact-store publication. No splitting, S3 upload, isolation admission or independent signed verdict is claimed.

== Decision and next action
Keep the default and measure against the existing traced six-coordinate F4 backend at the same complete root/materialization boundary. The present dense elimination/provenance work is large. Row reduction and correctness do not establish a gain. Full cold graph-IC versus the strongest applicable rho remains the active goal.

All producer PCC records are unsigned: 1ccf5e (generic refusal), 0e256e (lower-degree controls), 8f3d8b (reference shape), a42327 (degree-7 result), 16712e (latest qualification). Prefix: PCC-20261124-. No canonical scoreboard promotion. Concurrent main PCC appends leave publication history divergent; preserve both histories and reconcile without rewriting inherited ledger entries.

Sources: Larsson–Åström–Oskarsson 2017 §§2–3; Larsson et al. 2018 §§1–2; existing certified-relations checker and captured F4 reference. Primary sources and hashes are linked in the earlier hypotheses packet. KB/Conductor unavailable; no novelty or optimality claim. #link("https://github.com/isogeny-labs/crypto/pull/5094")[Lane PR #5094].
