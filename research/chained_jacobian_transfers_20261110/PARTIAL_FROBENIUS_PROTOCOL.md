# Partial Frobenius on the actual gluing graph

Frozen before new code or execution, 2026-11-12. Parent model checkpoint is
fdfe8c832bf586403537677466e3a5815db63717. All new graph class, model and
whole-route predictions below are unexecuted. Local Rust/Lean development
checks only, with a 15-minute bound per invocation; no paid experiment or
benchmark without actual live-budget/S3 admission.

## Sources checked before choosing this mechanism

Galbraith Chapter 9, Example 9.6.14 and Corollary 9.6.15 explicitly distinguish
q-power Frobenius as an inseparable isogeny of degree q from its identity
action on E(Fq). Example 9.6.24 identifies the dual Verschiebung with
V*pi=[q]. Chapter 26, Theorem 26.2.3(4), gives Galois covariance of the Weil
pairing. For n=4, p=101 acts trivially on mu4, so Frobenius on one factor
preserves its Weil form. Milne, Abelian Varieties I section 7, defines the
scheme-theoretic kernel and warns that its degree cannot be counted by
geometric points in inseparable characteristic; II section 1 identifies
rational points with Frobenius fixed points and supplies functoriality.
These primary cached chapters were read, not merely their search snippets.

Bruin–Doerksen 0902.3480 Lemmas 2.4/2.6 and Theorem 3.2 supply the existing
stable anti-graph quotient and principal-polarization bridges. The accepted
ISOGENY_CYCLE_PROOF_V2.md and actual 36-state native census identify the
fixed graph classes. Exact external CT43 at cc5d5541, original blob
a3081f47ce6b7e4c89ac1b0ae31eef7ecbde4bee, already gives the abstract six
anti-isometries U0*F^i and their three relative-sign classes; that abstract
orbit is credited. CT44's one-factor degree7 failure remains preserved.
This task tests actual graph coordinates and rational evaluation, with no
novelty claim for partial Frobenius or the abstract orbit.

Sources checked (KB): MCP tools absent; wrapper exact-mechanism query
“partial Frobenius single elliptic factor gluing rational points”, application
query “partial Frobenius Jacobian graph quotient rational transfer”, negative
query “inseparable isogeny one factor anti isometry no rational point
advantage”, and theorem query “FrobeniusPairings” have no executable in this
runtime. Scratch receipts ct43-kb-partial-frobenius-01/02/03 and -theorems-01
retain the failures. The encrypted atlas at 8edec422 cannot substitute for
readable primary evidence. Bounded exact repository notes, external CT43 and
cached primary PDFs are the fallback; an unsuccessful search establishes no
absence of prior work.

## Competing hypotheses and predictions

Use E0=(1,46), F0=(44,21) over F101, with orders 109 and 113. Extract Q0
from all actual elliptic four-torsion Cantor pullbacks into the old T0,
independently of abstract companion matrices. Let B=E0*F0 and
phi_k=(pi_E^k,id_F), k=0,1,2,3. Set Q_k=phi_k(Q0), Gamma_k=[2]Q_k.

H1: Q1/Q2 remain Galois-stable anti-isometry graphs and lie in the two other
relative-sign classes on the fixed factor pair. The actual balanced-walk
classes are predicted to be A24/A12 respectively, and their two-reductions
Gamma24/Gamma12. Q3 is predicted to be exactly the first-factor negative of
Q0, so sign normalization closes at three. Predict Q0/Q1/Q2 are distinct
both as four-graph sign classes and as actual two-graphs.

H0 alternatives: transport is not an actual anti-graph; the orientation
prediction swaps A12/A24; a graph or coefficient field requires a twist;
the two-glue model or rational route fails an exceptional fibre. Retain every
observed disagreement and fix the failed assumption through a new version;
do not relabel a prediction after seeing the output.

Frobenius has degree 101, but is identity on every E0(F101) point. Predict
phi1/phi2 act as identity on all 109*113 rational product pairs, while moving
the geometric gluing. The quotient property predicts maps between the
corresponding Jacobians of global degree 101^k, purely inseparable, with
connected first-factor kernel group scheme and only the identity geometric
kernel point. Never call the map separable or enumerate 101 kernel points.
Its adjoint norm descends diag(101^k,1), not a uniform scalar.

For canonical four-pullbacks K_i and norms H_i, the global cleared relation
is 4*F_k=K_k*phi_k*H0. Thus on rational points only, the factor operation may
be omitted and the relation evaluated by public inverse4 modulo109/113.
The analogous two-glue relation uses inverse2. Model construction and the
new target pullbacks still cost work. This predicts a zero-operation
Frobenius factor stage on already validated F101 elliptic inputs, not an
end-to-end transfer speedup or an ECDLP solver improvement. [pcc:unverified]

After three partial steps and the first-factor negative identification, the
rational product action is (-1,1). Its CRT scalar on order12317 is predicted
to be 6103 (residues108/1). The alternative factor-sign choice gives6214.
The global morphism remains a degree101^3 inseparable map, not integer
multiplication; rational equality alone must not be used as a global proof.

## Smallest informative native check

Reuse the existing Fp12, polynomial, elliptic, Weil pairing, Cantor, HLP and
Kohel engines. Add a child module with adapters only; preserve old native
bodies except the two-line registration, with snapshots and exact comparison.

1. Extract all16 actual Q0 points and their four-torsion groups. Apply the
   existing coordinate Frobenius to the E component only for k=1,2,3.
   Check projections, all256 group sums and product pairings, Galois stability
   and actual orbit/sign closure. Compare with independently regenerated
   balanced states12/24 and all four-point two-graphs.
2. Check Weil Frobenius covariance on every E[4] pair and on all graph pairs.
   A mutant treating Frobenius as identity on geometric E[4] must fail graph
   equality for k1/k2. Keep the earlier degree7 non-anti control unchanged.
3. Check Frobenius fixes all109 E0 and all113 F0 rational points, and all
   rational product pairs for k1/k2. Check the public CRT residues and the
   sign-normalized three-step product route. These are product-factor checks;
   no new whole Mumford map is accepted by this step.
4. Construct the two additional two-glue models from actual Gamma1/Gamma2
   using the accepted raw HLP adapter. Record sextics, factor destinations,
   scales, square status and complete curve counts. Check exact cover
   coefficients, every rational point, extension infinities/x0, all6 branch
   roots and exact graph. Do not silently retwist or assume monic leading
   coefficient. New primitive-four-glue models and whole Jacobian maps remain
   separate follow-up obligations.

If step4 exposes an adapter precondition, preserve the failed source/output
and revise a generic adapter faithfully. Do not drop model checks to promote
an incomplete run. Selected test is diagnostic; complete changed suite35+
with zero ignores/filters/failures, oldguards/fmt/strictClippy and every
before/after source hash is required for acceptance.

## Proofs, accounting and checkpoint

Write the group-scheme quotient/separability/degree, rational cleared-map,
nonuniform norm and actual class-identification proof. Include feasible
faithful Lean statements for the finite orbit, CRT and rational normalization;
expose missing actual AV/Picard bridges rather than assume the conclusion.
Existing two cover identities and their exact seven-certificate algebra
replay remain unchanged unless a genuinely new identity needs its own record.

Retain every raw invocation, source snapshot, hashes, start/finish/status,
failures and controls as whole standard XZ with raw/archive SHA256 and exact
roundtrip. Register claims before citing results; scientific standing remains
unsigned unless the existing independent signing workflow supplies a verdict.
Ship source-linked report, editable diagram/count data/SVG and inspected PDF,
then commit/publish through PR3877 and persistent issue3904. Continue to the
next unblocked task: generic whole Jacobian norm/chart implementation serves
both these partial-Frobenius targets and the still-open unbalanced7/49 phaseB;
T20 phaseC and admitted full-cost comparisons remain in the queue.
