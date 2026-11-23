# Actual Jacobian elliptic-cover Gram: first A

Prospective question: do the actual C7 degree2/2/3 cover pullbacks give
the global Gram G=[[2,-1,0],[-1,2,0],[0,0,3]] and a degree9 isogeny?
GLOBAL_GRAM_PROOF.md is a written proof with explicit standard Jacobian,
ordinary-endomorphism and Rosati dependencies, not a compiled Mathlib proof.
The finite native experiment independently tests the actual geometric-fibre
correspondences on every E7(F7) point; it cannot prove the global result
by finite sampling. No benchmark or advantage is proposed. [pcc:unverified]

## Fixed inputs and scope

Use only the qualified actual C7/E7 and R/P/projective chart and Q→E7
conversion from source3916f563, full result62daac1e (PR5026), without
replaying its57120 frame decisions. Fix f0=deck quotient, f1=f0∘R,
f3=order3 quotient. TARGETS contains exactly the11 independently qualified
E7(F7) coordinates, no scalar/log labels. Forward/fibre routines receive
only points, field operations and cover index, never expected Gram entries.
The inherited qualified digit-field arithmetic is extracted from
cm19_f7/field_b.rs (source0b9b506b), with unused full check/triples,
alternative Horner check and unused roots counter removed, and div added
using the qualified exponentiation method. Original file is retained as
an unexecuted input. Use F49:b²=3, F343:t³=6+5t, exactly the previous
qualified extensions. Do not execute old whole-curve/field-triple/H drivers.
[pcc:unverified]

## Geometric fibre algorithm

For degree2 over F49, solve Z²=v-(6x²+3x+1) at each affine E target,
retain both roots and branch multiplicity2 at root0. O has the two actual
original infinities[1:0:±1], each multiplicity1. For f1 apply actual R^-1
to the entire f0 fibre, with original projective C equation and forward
target validation for each point. [pcc:unverified]

For degree3 over F343, invert the qualified Q→E7 isomorphism, including
O/(1,3) as the two actual x=0 eigenaxes (each multiplicity3) and finite
delta0 point(1,4)↔Q(3,3). For finite Q(u,w), recover v=(w-H(u))/4.
If v≠0 use all cube roots t³=v, normalized[1:t:u/t]. If v=0, verify
u=0 and use all roots z³=3 on[1:0:z]; never divide by zero. Transform
every point through P to original C7 coordinates. [pcc:unverified]

For each of33 fibres, check weighted degree2/2/3, unique support, original
quartic validity, actual forward restoration, Frobenius7 stability with
multiplicities, P and R inverse roundtrips. Print every fibre and all actual
coordinates/multiplicities; rational/extension support counts are descriptive
observables with no post-selection. [pcc:unverified]

For every target P and i,j in{0,1,3}, compute the actual E group sum of
f_i on the full f_j^-1(P) weighted fibre, subtracting the actual sum on
f_j^-1(O). Each extension-field addition validates both inputs and output
on y²=x³+2x+6. Check all99 corrected images descend to F7 and equal the
written G entries applied to the target point. Check121 addition laws for
each of9 computed maps (1089 total). Group labels are never supplied to
the correspondence constructor. [pcc:unverified]

## Positive and real negative controls

Pristine identity/model inverse and original forward maps must pass.
Omitting the actual i=3,j=0 basepoint correction must disagree on all11
targets (offset is nonzero); degree2 diagonal zero-offset omission is an
explicit11-case positive control and must remain unchanged. Changing the
degree3 ramified fibre multiplicity3 to1 must fail weighted-degree and
actual elliptic push-sum checks at(1,3). Swapping the actual quotient
infinity images must fail the actual image at the positive infinity.
Restore pristine values and pass. A control that unexpectedly passes
falsifies that control; preserve the full first failure and revise a new
version, never alter frozen assertions or report manufactured counts.
[pcc:unverified]

## Acceptance, limits and custody

Before ANY first rustfmt/compiler/Clippy/native call, publish all source,
runner/metadata/protocol/inheritance/runtime locks and fully read back every
locked input plus lock from that exact source commit. Seven strict phases:
rustc/clippy/rustfmt versions, rustfmt --check on3 modules, rustc -Dwarnings
with overflow/debug assertions and opt3, clippy-driver -Dwarnings, actual
native experiment. One worker,120s/2GiB per phase,256MiB free RAM and64MiB
compiler-temp floor; stop on first nonzero status. No allow/waiver, provider,
credential, runtime or limit changes to make a failed call pass. Preserve
all original stdout/stderr/argv/version/phase/build/pre/post hashes and the
whole original ELF in temporary custody pending approved durable storage.
No failed frozen file is edited; repairs use a new version and full remote
readback before calls. Python performs file/Git/hash/JSON custody only.
[pcc:unverified]

Native acceptance is seven0 exits, guards[], all33 fibres/99 images/1089
actual addition laws and real controls passing with complete raw output.
Any failed assertion, timeout, native signal, bad input guard or missing
raw evidence rejects the first qualification and is recorded verbatim.
Do not fabricate a scientific outcome after format/compile/Clippy failure.
Canonical inherited artifact config is locked; complete raw at/above its
threshold requires the established approved store workflow, with the same
parent5019 raw custody blocker still open despite its external merge.
[pcc:unverified]

After actual outputs: unsigned data and separate interpretive claims,
source-linked MD/diagram/PDF with every final page visually inspected,
full result readback, canonical ledger/artifact gates and session log.
Finite rational-point qualification is separate from the written global
degree argument. Actual genus3 class arithmetic,3-primary gluing/integral
CM19 S maps, J4 pairing/target construction, generic Lean and independent
replay remain open. The original transfer Ff0*(P)=(2P,-P,0) preserves DLP
but supplies no new solve advantage. No subfield descent over F7; any
future benchmark needs a promising quantitative mechanism and full cold
cost against strongest elliptic references. KB/Conductor unavailable,
no novelty clearance; original owners/EKS/paid STOP/vault/CI/merge rules
persist. Draft5035 remains while this scoped work is unfinished.
[pcc:unverified]
