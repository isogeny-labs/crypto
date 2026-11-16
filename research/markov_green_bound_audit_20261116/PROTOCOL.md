# General Green bounds and literal asymptotic-equality audit

Frozen before formal implementation, 2026-11-16 UTC. Item10 continuation;
all14 remain active. Zero paid compute; mathematics exclusively in Lean.

## Sources checked

KB MCP tools absent. Ten actual wrapper calls and three Conductor calls
returned127; exact scripts/logs/exits retained under scratch/markov-green-prior.
No novelty/absence claim. Canonical prior BirthdayImports/UniformChain/
GreenMoment and their pinned custom import closure reviewed and reused.
Official Matrix/Stochastic, Convex/DoublyStochasticMatrix, Matrix/Basic
transpose_pow, Finset/Lattice/Fold finite maxima, Nat/Prime/Infinite,
SpecificLimits/Normed polynomial/geometric limits, Asymptotics/Lemmas
little-o/tendsto equivalence reviewed at bounded source ranges.

Primary: Kim–Montenegro–Peres–Tetali2010, DOI10.1214/09-AAP625,
https://arxiv.org/pdf/0712.0220, actual v2 electronic reprint27Sep2010,
300644B, SHA25667bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07.
Prior complete bounded text PDF1–8/start9 and individual visuals6,7,8;
fresh Lemma3.3 proof text and complete individual page8 visual reviewed.
Reprint pagination differs from journal; no full29-page reading claimed.
Publisher download HTML/PDFparse failure and original Teske403 retained.
Bounded correction query did not establish absence of errata.

Lemma3.3 on reprintp7 states, for an ergodic finite chain with uniform
stationarity, A_T,A*_T <= 2*sum_(j=0)^T (j+1)*max_(u,v) P^j(u,v).
G_T includes identity P^0. Row proof uses row normalization; column proof
uses transpose being stochastic, supplied by uniform stationarity.
On p8 the following paragraph prints the asymptotic EQUALITY
A_T,A*_T=(2+o(1))/(1-d)^2 after assuming the pointwise upper envelope
P^j(u,v)<=o(T^-2)+d^j. Treat that literal equality separately from the
qualified finite lemma and asymptotic collision order.

## Targets / falsifiers

1. Actual finite transition PMFs constructed from official row-stochastic
real matrices, with exact entry bridge. Derive all-power normalization and
nonnegativity from official submonoid power membership, not hypotheses
encoding the desired Green moment.
2. Actual inclusive matrix-power Green sums and finite maximum row/column
square energies and power entries. Prove row-only bound, preferably the
sharper sum(2j+1)*maxEntry, then source2*sum(j+1)*maxEntry. Derive column
bound with actual column normalization/transpose and source qualified
uniform/doubly-stochastic conclusion. No ergodicity needed for this finite
inequality; do not assume uniform column sums for arbitrary kernels.
3. Candidate guard-deletion counter: positive reset chain on ZMod11,
q(0)=1/2 and q(y!=0)=1/20, actual normalized PMF/matrix, stationary law q,
positive one-step reachability/period1, nonuniform stationary law. AtT1
column energy at0 should be19/4, actual maxEntry0=1/maxEntry1=1/2,
source RHS4. Prove the strict violation with actual maxima, or retain
failure; qualified source lemma remains separately scoped.
4. Actual uniform chains on prime state counts N_t>=3^t chosen by the
unbounded-prime theorem. Derive allT Green maxima1+(T^2+2T)/N. For T=t,
error1/N_t should satisfy actual IsLittleO(1/t^2); prove the envelope for
ALL j>=0 with d=1/2, and actual row/column maxima tending to1 rather than
2/(1-1/2)^2=8. Only if all actual source hypotheses/envelope/limits close,
refute the literal printed asymptotic equality. Do not refute the upper
bound, paper's collision asymptotic order, or qualified adding-walk constant.
5. Retain zero-time/singleton/uniform controls. All conclusions faithful to
actual objects; transparent pinned proposition bundles allowed for main.

## Execution / publication

At most100 source states; deadline2026-11-16 16:00UTC. Existing13packages
plus one local path dependency=14, no copies/hydration/lake update. Preserve
all47 currently published plans6127entries plus36 static-hash dependency
entries=6163entries/48initial plans, checked before/after every state/gate/
replay and archived losslessly. Verify actual counts during setup.
Register every current root from01. One formal operation at a time; no
source edits during builds/gate/native/replay. Default limits, no holes,
custom axioms, native proof evaluation, raised limits, global flags,
waivers or frozen edits. Python only source/metadata/Git/provenance/JSON/
PDF/archive, never mathematical computation.

Stage git add --sparse before whole gate. Completed gate before complete
written-header binding1complete/0prefix/0unresolved/empty suffix; separately
read full normalized/parsed statements. Frozen main exporter unchanged.
Lossless state archive/hash/inventory/actual restore; data<=65536B,
PDF<=10MiB, multipart if needed. Full report text/every individual visual/
layout before seal. No copyrighted primary PDF/text or regenerable PNG
committed. Normal producer before once-only PCC. Issued pins immutable;
localPASS/CI/signskipped do not confer signed standing. Update all14 maps,
preserve ledger prefix/historical main fork. Exact-tree nonforce publication
with fresh leases; no resurrected branches/merge/rebase/rechain/force/
retarget/mainmerge. Next owner10:30UTC; no busy polling; $0 spend.
