# Rational-function tame reciprocity

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `P1TameReciprocityAudit.actual_p1_tame_reciprocity_certificate`.

The [rational-function reciprocity proof](README.md)
now derives the product formula for actual nonzero rational functions with
split numerators and denominators over any field. Any finite set containing
their roots gives finite tame product times infinity symbol equal to one.
Actual finite support is proved before finprod. Over every algebraically
closed field, all rational functions satisfy the splitting guard, so the
full rational-place product and the corrected inverse product are one.
The global product is derived from actual factorization and local values;
no reciprocity conclusion is supplied as a premise. [pcc:unverified]

Canonical proper residues at every rational finite base place are constructed
using Y²+Y=(X-a)³ and its proved nonsingular local point (a,0), with derivative
in Y equal to one and a distinct negative point. Global nonsingularity of
that auxiliary model is not asserted. The proper residue fixes K, is
surjective, has maximal-ideal kernel and agrees with every realizing actual
model and point above a. Polynomial residues equal evaluation. The actual
normalized infinity symbol is also independent of its realizing model.
Derived constant/linear orders and symbol values prove all generator-pair
products, including repeated roots. Official polynomial root factorization,
actual numerator/denominator identities and two character kernels extend
the result to actual split rational units. Known methods and inherited
proofs are credited. [pcc:unverified]

Actual F5 controls distinguish a finite product 2 from the corrected product
1 after including infinity; removing local signs gives product 4 instead of
1. Actual binary and ternary products also equal one, with infinity values
1 and 2. These refute incorrect deletions without refuting a qualified source
theorem. [pcc:unverified]

Local **PCC-20261116-22c62d PASS at 2026-11-16T04:54:24Z**, signed status unverified.
Whole-project gate88/11 modules (10 concrete, one official bridge), zero
failures or trusted native evaluations. All313 formal and17 artifact pins
match;247 custom modules are pinned and229 actually imported. All316
predecessor pins match before/after19 states and gate; eight failed states
and both default-limit failure types are retained. Final new-source warnings
zero;30 packages, no copies, hydration or lake update. Full2626-character
header SHA-256 `1d1d356599cb566e6abb9c15552e91e5ed1adad3bc6ca96c5f1fdbbe78e60f15`;
separate binding check after completed gate: one complete, zero prefix or
unresolved statements, empty suffix; full normalized and parsed headers read.
Lossless401 files/125 objects/64768-byte XZ/one physical part/round-trip and
actual restoration equal. First-valid one-page47110-byte report: all4672
characters read, full-page visual inspected, in bounds, no drafts. The62636-
byte source plan keeps the full inline statement. Same-host/prover and
procedural binding provide no independent compiler/parser/host assurance.
No holes, custom axioms, native proof evaluation, raised limits, waivers,
frozen edits, mathematical Python or paid compute. [pcc:unverified]

Sources checked: six KB calls and initial/expanded Conductor unavailable
(127), with bounded official and frozen-source fallback. Gille–Szamuely
PDF204–206/208–209 (printed190–192/194–195), text and five individual visuals,
contain complete bounded proofs7.2.1–4 and7.2.8–10. The displayed7.2.2 checks
bilinearity explicitly for n=2. Earlier Milnor relations and every norm/base-
change dependency were not all read;7.2.5's field-norm identification proof
was not displayed. Initial PDF extraction preceded the fresh KB fallback;
that order is recorded. No novelty or absence claim. [pcc:unverified]

General closed-point residue norms, elliptic global reciprocity, arbitrary
divisor representatives, isogeny comparison, general descent, canonical
Tate and general MOV/Frey–Rück remain open. All fourteen targets remain
active. Rational-function reciprocity does not close these bridges.
[pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `428c0bca48eaab58ea0f21fde389f6448d11eab9` and local receipt `ca593bcb1701d14e648feef21e2622aba859bed6` share tree `9fa3cb42f644c1a2db13f0a4ad61d486cc296afa`. Anonymous fetch completed and a fresh read matched the exact head, title, body and ready state. All 42 pinsets / 4707 entries match. Current-head CI, review and signing are pending. [pcc:unverified]
