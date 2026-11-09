Current result: [the actual prime-five Cantor cycle](STOLL_CYCLE.md) has
smooth fraction1versusambient4/5; the whole tracked project gate passes348
declarations/0failing/0trusted-native. The [complete native census](CENSUS.md)
passes all4cells, two-language arithmetic and byte-identical repeats over
all F3/F5monic quintics. [Calibration failures and controls](CALIBRATION.md)
remain retained. Scientific standing is unsigned/unverified; full ambient
Stoll-group/geometry and the original all14gaps stay explicit. [pcc:unverified]

# Mumford counts and a finite Cantor subgroup counterexample

The new [F5control](FIVE_CONTROL.md) proves a complete16-pair model,
all16smooth, before native calibration. The F5-stage project gate240passes
with0failures/0trusted-native exceptions. A separately frozen native
calibration protocol pins both built executables; no curve invocation has
run at this pre-calibration writing. [pcc:unverified]

The subsequent [Cantor subgroup proof](CANTOR_SUBGROUP.md) checks all64
actual polynomial additions on the complete model and proves an order-two
proper cyclic subgroup with smooth fraction1, versus ambient7/8. Universal
finite cyclic-proportion equality is false in this arithmetic model.
Geometric Picard identification and qualified large-prime/asymptotic regimes
remain separate. [pcc:unverified]

Lean proves that the complete reduced-Mumford polynomial-pair model for
y²=x⁵−x over F3 has eight representatives, seven with completely split u.
The fraction is 7/8, and its difference from 1/2 is 3/8. This refutes an exact
finite-field reading of the 1/g! approximation at genus 2. Gaudry's qualified
fixed-genus, q→∞ statement is preserved. [pcc:unverified]

The curve polynomial is proved monic, degree 5 and separable/squarefree.
`ReducedPair` ranges over actual polynomials u,v with u monic, degree u≤2,
degree v<degree u and u dividing v²−f. The proof classifies every such pair,
proves injectivity and constructs an explicit bijection with the canonical
coefficient candidates. It does not assume a hand-selected representative
list is complete. [pcc:unverified]

Mathlib's polynomial division is noncomputable. The proof first establishes
an exact quotient/remainder identity and a scalar coefficient test equivalent
to actual monic division. Kernel `decide` then enumerates that test over the
finite coefficient field. The full model and splitting counts follow through
proved equivalences. No native decision or custom axiom is accepted.
[pcc:unverified]

The nonsplit case is u=x²+1,v=0. Lean proves x²+1 has no root in F3 and
cannot split. The other seven cases split, including the identity u=1,v=0.
This count includes identity; MMT2002's displayed eq12 begins at degree 1
and uses a different relation-count convention. [pcc:unverified]

## Scope and checks

The Cantor-stage whole tracked gate passed170declarations,0failing and
0trusted-native exceptions; the subsequent F5control raised the then-current
project gate to240. Both include generated declarations. The initial count-only gate passed80; its
historical receipt is retained. The exact initial source/primary read
scopes and planned native scalar supplement are in [sources.json](sources.json).
All intermediate failed builds are retained in `evidence/`.
[pcc:unverified]

This is a complete finite polynomial-pair model proof. A formal geometric
identification with Pic⁰/J(C) remains a separate obligation. The subsequent
actual finite Cantor group law and cyclic fraction counterexample are now
proved in [CANTOR_SUBGROUP.md](CANTOR_SUBGROUP.md). A small
finite discrepancy also does not disprove an asymptotic or typical-subgroup
heuristic. The [original fourteen-item audit](../theorem_census_20261109/README.md)
remains partial. [pcc:unverified]

No new native curve population, statistical study, benchmark, paid compute
or separate-host validation was run for this proof. The initial scalar identity
supplement checks only the quotient/remainder rewrite and
cannot establish these counts or the geometric identification.
[pcc:unverified]

Reproduce with `tools/lean/proof-gate.sh research/jacobian_smoothness_audit_20261109/lean`.
The project pins Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679.
See [gate](evidence/mumford-count-gate.log) and
[source](lean/MumfordSmoothness.lean). [pcc:unverified]

## Proof record

Packet PCC-20261109-07eb0d passes its full local check at20:00:58Z, with
source commit6fee3178 preceding issue. Standing remains unsigned.
Supplement IDC1h7d12c937b3b1482b accepts32derived and11extra points only
for the scalar quotient/remainder rewrite; its proof pointer matches.
Exact hashes and scope are in [verification.json](verification.json).
See the [two-page report](reports/proof-20261109-mumford-count/report.pdf).
[pcc:unverified]

The current Cantor packet is PCC-20261109-4a0f82, with source/identity
plan in [cantor-sources.json](cantor-sources.json). Scalar supplement
IDC1h95637a8da70e67cd covers only cancellation/factorization identities.
The actual gcd,64reductions, group matching and subgroup counts rest on Lean.
See the [historical Cantor-stage gate](evidence/cantor-subgroup-gate.log).
[pcc:unverified]
