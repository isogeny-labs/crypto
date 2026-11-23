# CT341 frozen protocol — exhaustive F103 bielliptic dual-map census

Registered 2026-11-23 UTC before writing or running any CT341 science program. Owner: one coordinator, no exposed native child-agent slots. Tracked on draft crypto#5045, following merged #4997 CT339/CT340. [pcc:unverified]

## Frozen sources and literature

- Original CT339 standalone Python source SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`; raw canonical JSON SHA-256 `eb986754a030062db0aa629989edf1ac2705695f5d8b1863fc19dab28c085492`.
- Original CT340 pullback protocol/report under `research/chained_2power_descent_20261123/{CT340_PROTOCOL,CT340_RESULT}.md`; source SHA-256 `d81cc062589682b7b8454e40cc63027949a314bed3e7e2aa564c6b0603`.
- Prior elliptic subcover and Richelot context: Diem, https://arxiv.org/abs/math/0312413 and Magma handbook https://magma.maths.usyd.edu.au/magma/handbook/text/1615 ; no novelty assertion.
- KB MCP, Conductor and native Rust agent/compiler not exposed in this runtime; use exact repository evidence and local public fixture only.

## Finite, falsifiable experiment

Use unchanged CT339 curve `C/F103: y²=∏_{j=1}^6(x-j)`, its odd `z=1/(x-1)` model and source `F(z)=∏_{j=1}^5(1-jz)`, product quotients `Eplus`, `Eminus` with independently enumerated 92 and 100 rational points. Implement a separately authored `Psi:Eplus×Eminus→J(C)` using CT340's generic Mumford pullback formulas and its W1/O/X=0 exceptions; evaluate `Phi` only from unchanged imported CT339 source.

Before accepting an output, require the imported CT339 code and frozen raw stdout hash to match, and the announced curve/point counts to match.

For **all 92×100=9200** rational product pairs `(P,Q)` (including identities, branch points and ramification): compute the actual reduced Mumford `D=Psi(P,Q)`, require `Phi(D)=([2]P,[2]Q)`, collect every distinct canonical `D` and exact fiber multiplicities. From the degree-4 separable split isogeny with rational 2-torsion kernel, predict **2300** distinct `D` and four product preimages per class. A different result is an experiment failure, not success by relabeling.

For **every** resulting Jacobian class `D`, require `Psi(Phi(D))=[2]D` using unchanged Cantor arithmetic. Require (i) all three nonzero kernel classes of `Phi`, plus the identity, (ii) full `Phi` kernel size 4, (iii) `Psi` kernel size 4, (iv) generic and exceptional pullbacks, and (v) distinct odd/even scalar behavior. Independently enumerate `J(C)(F103)` as the image of this proven surjective-on-rational-points `Psi`; do not assert exhaustive rational-Jacobian coverage before checking cardinality, fiber uniformity and group identities.

Fail closed: corrupt a generic pullback ordinate sign, wrong special W1 handling, and/or a frozen source fingerprint; test that each actually fails in normal and optimized Python. Source and canonical JSON are immutable once committed; report commands, hash identities, failures and counts. No timed performance result or complete ordinary prime-field ECDLP advantage is authorized by this experiment. No PCC/Lean proof from a same-coordinator replay.

## Reservations, budgets, and next gate

The 695/701-record norm-point reuse fixture, physical `C169/C13a/C13b/N2_actual/D` and strongest matched cold rho/BSGS/IC comparisons are separately owned. Do not access or claim them, alter #5007 native F3/E7 results, run paid EKS, compete for benchmark slots, or touch locked crypto-autoresearcher vault. No extra automation/worker claim or PR merge. Following a positive result, the next gate is **independently assigned source-pinned Rust/Cantor and signed PCC** reproduction plus real all-exception/source-curve validation, not a broad speed claim.
