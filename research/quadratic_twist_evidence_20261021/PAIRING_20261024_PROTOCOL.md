# Twist-assisted pairing transfer — frozen protocol

2026-10-24, run id `pairing-20261024T0442Z`.
Starting head: `ad66884e6be033ec8264ccb52500ef5e98f2c44b`.

## User request and boundary

Pursue the positive connection: twist points used in a pairing reduction from
an elliptic-curve logarithm to a finite-field logarithm where index calculus
applies. Preserve the completed direct mixed-relation obstruction. These are
different mechanisms. Exclude invalid-curve attacks and real secret targets.
Use public synthetic/known-answer inputs only. This iteration tests transfer
correctness, degeneracy and field prerequisites, not a speedup or hardness claim.

## Native finite experiment

Use the already registered 942 nonsingular short models over
p in {5,7,11,13,17,19}. For each E:y²=x³+ax+b, use the least positive
nonsquare d and the MONIC twist E_d:Y²=X³+d²aX+d³b, with the same exact model
identities as the existing registry. Work in K=F_p[u]/(u²−d), mapping source
points by (X,Y)->(X/d,Y/(d*u)). Do not confuse this with the historical
nonmonic twist coordinate convention; record the conversion explicitly.

Factor the actual enumerated orders. Record every odd prime r dividing #E(k),
including cases r does not divide #E_d(k). For each shared r, choose the first
nonzero cofactor projection of the lexicographic rational point list as P.
For EVERY point T in E_d(k), set S=psi([#E_d/r]T), verify rS=O and its
Frobenius sign, and evaluate the Weil/Miller transfer. Record the zero-image
case separately. Check each nonzero target has exact order r and norm one;
record the q≡−1 mod r / embedding-degree-two prerequisite rather than promote
it into generic curve coverage. Retain complete per-model/per-prime coverage.

Controls: O and cofactor-kernel auxiliary points; a nonzero same-source
cyclic-subgroup auxiliary point (expected degenerate pairing); wrong subgroup
membership; wrong declared count, missing model and false pairing value.
For same-subgroup controls use shifted disjoint-divisor evaluation, with every
zero denominator/support collision rejected explicitly. The naive direct
formula's undefined cases are not evidence that a pairing is zero or one.
Enumerate candidate auxiliary offsets lexicographically in the full finite
E(K) list; retain failure if no legal offset is found.

C++ reuses the existing compiled curve backend and binary Miller chain.
Independent Rust uses coefficient-pair field arithmetic, exponent inversion
and a linear Miller addition chain, independently recomputing all points,
torsion/cofactor counts, pairing values and control decisions. No shared field
or group arithmetic and no Python research execution. Existing sources stay
unchanged; write new sources and dated evidence. Cap each native full replay
at 600 seconds. Keep timeouts/failures as evidence, not mathematical rejection.

Also replay the existing E/F_43:y²=x³+x, d=2, r=11, P=(31,18), Q=(13,24),
monic twist point (35,12) mapping to S=(39,3u). Recompute all eleven scalar
transfers and native finite-field IC certificates, including scalar7, with
independent matrix/field/actual-point checks. Retain the old archive unchanged.

## Lean targets and claim gates

Add new modules rather than rewriting old source-bound proof files.
1. Prove the exact cyclic-character logarithm-transport implication from
   explicit additive-to-multiplicative homomorphism and nondegeneracy/order
   hypotheses. Do not call an assumed pairing homomorphism a general Weil proof.
2. Instantiate a computable quadratic field over ZMod43. Attempt kernel checks
   of the explicit Miller function on actual Mathlib elliptic points, its
   scalar table, nontrivial order11 target and scalar7 recovery. Attempt the
   full 11×11 generator-pair table; preserve resource or elaboration failures
   and state any remaining finite/formal gap. No sorry/custom axioms or trusted
   native evaluation. Do not define the transfer by a discrete-log lookup.
3. Check norm-one target algebra, with a source-bound IDC1 polynomial record
   for the norm multiplication identity and false-identity/source controls.
4. Run the unchanged proof gate on all study modules and audit transitive
   axioms. Keep every failed source as .lean.txt and its compiler log/exit.

A finite checked Miller table is not a general proof of Weil-pairing
nondegeneracy or equality with the geometric divisor definition. The native
finite-field IC replay is not automatically a formalized IC algorithm.
General applicability, constructive transfer, subgroup injection, destination
solver costs and practical advantage remain separate requirements.

## Delivery and validation

Keep source-bound raw results, independent audit, controls, commands, hashes
and failed attempts in the dated native/validation directories. Produce a
source-linked report, graph/diagram, editable sources and inspected PDF in
reports/pairing-20261024T0442Z. Run mandatory release library tests and touched
native/proof/metadata checks before pushing. Extend PR #2381 normally with an
accurate body/proof map/problem record; keep all existing merge holds. Do not
touch frozen evidence, the registry, benchmark history, ledger rows or the three
conflicted browser/traits/leaderboard files. Actual shared Conductor access is
still unavailable; no reservation or replacement coordinator is claimed.
