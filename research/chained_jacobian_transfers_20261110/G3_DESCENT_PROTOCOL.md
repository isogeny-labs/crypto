# G3: complete encoded-point descent into a plane-quartic Jacobian

Frozen before new implementation or arithmetic runs, 2026-11-13. This is a
correctness and proof protocol, not a timing or solver comparison. Root source
base is `d393a223fb2f5af5e60d308193cbdfbf7adc768d`; the settled consumer is
`1644d47db7b8ff8de8a0318dc2a6067d6e854328`. Scientific standing is unsigned.

## Question and preserved scope

Can the existing GHS/Ciani construction and canonical plane-quartic Picard
engine evaluate the trace pullback for **every encoded source point**, including
nonsplit, ramified and coinciding conjugate fibres, using the actual origin
fibre rather than a constant inferred from scalar samples? Preserve the user's
nontrivial DLP transfer, prime-field descent, independent validation, continuous
checkpointing and eventual computational comparison requirements.

The initial acceptance panel is the first split case at each of q=5,7,11 in
`research/ghs_cubic_genus3_build_20261012/results/run-20261013T0320Z.jsonl`.
Freeze those complete producer records in `evidence/genus3-primary-01/cases.json`.
They report (source order, odd prime)=(148,37),(356,89),(1324,331); these are
expected values to recompute independently, not accepted fresh counts.
The exact cubic moduli in those records win over a constructor's default
field choice. Coefficient indices are field encodings, never DLP labels.
The new short model uses the core's quartic-infinity origin; an older
root-fitted Weierstrass model is retained as provenance and is not an input
coordinate convention. Quartic equations may differ by a nonzero scalar.
[pcc:unverified]

At this stage `candidate_id` and finalized EC1 identities remain null. The
generator rule is frozen: enumerate short-Weierstrass points in lexicographic
coefficient order, choose the first nonzero [4]P killed by the independently
checked odd prime. Record the resulting exact field/model/subgroup/generator
identity before any later performance workload. No selection or speed claim
will use these public, historical fixtures.

## Proposed construction

Let K=Fq³, u=t²+bt+c, and u_i its three coefficient conjugates. The existing
core gives a smooth descended plane quartic C/Fq and the degree-two cover
pi0:C_K→E0, with E0 the smooth model of y²=u_0 u_2. Put
D6(P)=sum(i=0..2) sigma^i(pi0*P). Compute
phi(P)=[D6(P)-D6(O)] in the existing canonical Pic^0(C)(Fq) engine.

Represent each completed degree-two fibre by a line L and conic Q over K.
Multiply the three conjugate ideal sheaves (L,Q), homogenize the eight
products to degree six, and descend their span by coefficient traces of
the products multiplied by 1,g,g². Products preserve local multiplicity at
overlaps. Feed the descended generating space to the **existing** reducer.
Its rank/completeness requirements remain in force. No support root,
extension point, scalar label or fitted origin constant enters the runtime.
The fixed origin space must actually be constructed, not short-circuited.
This is a proposed implementation; the written derivation is separate.
[pcc:unverified]

The prior W1 transfer handled only a subset of split fibres and calibrated
an origin shift from samples. Its reports and failures remain immutable.
Do not reuse its source-point scalar table or noncanonical degree-three keys.
Do not run `gen_instance` at the tiny sizes: its growing-size order filter is
a different contract. Do not call Kf.sqrt at q=5; its documented shortcut is
for q³=3 modulo4. Runtime fibre ideals require no square roots.

## Reuse and permitted source changes

Reuse `src/cryptanalysis/diem_quartic.rs` for its original cubic field,
homogeneous polynomials and checked core; reuse
`src/cryptanalysis/plane_quartic_pic.rs` for normal forms, ideal products,
canonical reduction and group operations. The only planned existing-source
changes are visibility for the original `Core`, its fields, `build_core`, and
the homogeneous polynomial types/operations needed by the adapter, plus
thin public accessors for the original Picard linear kernel and degree-six
cubic-space extraction. Check that removing these exact interface additions
recovers both frozen source files byte for byte.
No existing arithmetic body, filter, validation criterion, ignore, policy or
verifier changes are authorized by this protocol.

Use `isogeny_algos::fpr::FpR` as the separate polynomial-basis oracle in
degree3 and degree6. It already supplies irreducibility, arithmetic and
square roots. Do not write another finite-field or Jacobian engine. A thin
dependency wrapper may import original modules without compiling their old
tests as new adapter tests. Preserve all original tests in their old jobs.

The primary baseline records all794 current inputs of the previous full
replay, the old50 consumer inputs, and the **whole six-job** consumer workflow.
No one of those794 inputs includes the Diem or plane-quartic modules; their
separate exact source hashes are recorded. Prior C0/T20 science remains pinned
to its own immutable source. No focused CI job is removed or weakened.
[pcc:unverified]

## Required acceptance and independent controls

1. Validate exact field basis, canonical coefficients, curve equations,
   nonsingularity and the frozen fixture identity. Independently enumerate
   every short-Weierstrass point using FpR, verify its order and primality,
   and check both directions of the quartic/Weierstrass coordinate change,
   including O and the two x_unshifted=0 branches. Charge all setup if costs
   are measured later. Reject malformed, foreign-model and oversized inputs.
2. For **every** source point in all three fixtures, build the completed
   trace-pullback generating space and canonical Jacobian class. A separate
   Fp6 point-and-local-jet oracle must identify the same divisor scheme and
   cubic space, including nilpotents, repeated support and Frobenius overlap.
   Construct the Fp3→Fp6 embedding explicitly and verify its modulus and
   Frobenius compatibility; never reinterpret coefficient arrays.
3. Check phi(O)=0, negation, doubling, every odd-subgroup scalar, nonidentity
   and exact image order. On the complete source group, verify every edge
   under a deterministically found generating set, and independently verify
   that it generates the complete enumerated group. Report the observed
   even-part kernel; do not assume injectivity on the full even-order group.
   Check auxiliary-cubic reduction independence on representative exceptional
   cases. The written pullback/norm argument must establish odd-subgroup
   injectivity without using enumeration as its mathematical definition.
4. Preserve explicit controls: replacing the sum divisor by a reduced union
   must lose multiplicity at the origin overlap; a wrong infinity sign and
   swapping the finite and infinite x_unshifted=0 branches must be detected;
   omitting or changing the origin shift must be distinguished whenever that
   shift is nonzero. Record any null control on its actual fixture. A wrong
   cubic basis or coefficient interpretation must reject or give a documented
   different valid output. Include coherent coordinate changes as positive
   controls. No silently discarded exceptional points or newly ignored tests.

If a required assertion fails, retain that original execution and diagnose
the first failure. A materially different corrective attempt gets new source
and a new run record; do not rewrite the protocol or old result to pass it.
If ideal generators fail the reducer's completeness limit, record that exact
gap and implement a justified completion rather than lowering its rank test.

## Proof obligations and limits

Write the actual completed covers, the special coordinate branches, the
Cartier-ideal product/descent argument and the origin correction. The three
involutions give pullback/norm composites [2]; use character separation,
not coprime orders of conjugate elliptic factors. Prove that the descended
trace-pullback is injective on an odd-order cyclic subgroup. Distinguish
the global (2,2,2) isogeny of degree8 from a finite subgroup inverse. [pcc:unverified]

Formalize feasible central polynomial and group implications with pinned
Lean4.35 and the existing proof gate, exact statement bindings, and an axiom
audit. Expose the actual-curve/Picard/sheaf/descent/reducer bridges if they
remain unfinished; conditional algebra is not full geometric formalization.
An arbitrary-class Jacobian norm and a destination IC solver are separate
future tasks. This protocol neither measures nor establishes an advantage.

## Execution and preservation

Native Rust/Lean only. Python may package source/command/log receipts, manage
GitHub, lint metadata or render the report; it must not implement arithmetic,
cryptographic tests, replay verification or a benchmark driver. Use a fresh
native build directory outside each raw run boundary. Exact commands, source
snapshots/hashes, environment, stdout, stderr, original exits and all final
guards are retained. First strict static build ≤15min; each initial native
unit or bounded CLI attempt ≤15min; each proof attempt ≤10min. No background
process may be represented as accepted before its original terminal receipt.

These are local correctness builds/tests. No timing panel, IC run, paid
allocation or speed comparison is admitted. Respect the repository checkpoint
gate for experiments; unavailable S3/budget admission remains an explicit
blocker for those experiments. Commit/publish this protocol and current GitHub
resume state before implementation; checkpoint unit/proof work continuously.
Preserve an entire original run tree as ONE standard tar/XZ archive with
every member, raw/archive hashes and exact roundtrip. Use the existing genuine
artifact publication route when policy requires it; no split, prune or fake
pointer. Each result gets a claim with its actual standing and a source-linked
visual report/PDF with every final page inspected. Advance after each rung.
