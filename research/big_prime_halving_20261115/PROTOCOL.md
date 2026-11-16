# Arbitrary-precision auxiliary point-halving setup

Freeze before new implementation or qualification. This is a native software
port of the already qualified subgroup/trace/tower setup, with independent
complete-root replay. It addresses field width, not relation solving. No DLP
target, supplied scalar, base logarithm, norm assignment, scientific timing,
yield estimate, fitted exponent or paid experiment enters this component.
The requested complete general algorithm remains active. [pcc:unverified]

## Source review before code

KB unavailable: no loaded research-kb tools; wrapper status127/no executable.
Read the parent's generated setup, complete preimage and OpenSSL replay code,
SETUP-BOUND.md, the projective tower and its shared-law proof map. Existing
guarded-fall-degree arbitrary-prime Rust construction/replay handles a different
guarded polynomial DAG model; it does not provide this point-halving interface.
No absence or novelty claim is made. [pcc:unverified]

Read official FLINT API/header declarations and exact upstream v3.1.3 source
for fmpz_is_prime, fmpz_sqrtmod and fmpz_mod_poly_roots. Online documentation
is3.7.0-dev and partly generated; its newer ECPP/generic-ring implementation
description is not attributed to the installed3.1.3-2 package. That tagged
source uses proved primality mode with deterministic small tests and
Pocklington/Morrison/APRCL paths, and prime-modulus finite-field roots.
Package/library/header/binary hashes bind actual builds; no blanket assertion
that every Debian-patched byte equals upstream. [pcc:unverified]

Primary mathematical sources: MIT18.783 Fall2023 Lecture2 section2.2 for
doubling, Lecture3 sections3.11--3.11.2 for roots, Lecture4 definitions4.18/4.20
for isogenies and Lecture5 Theorem5.13 for the exact rational two-isogeny.
Existing parent sources and the5new primary hashes are retained under checks/.
Use the curve-algebra workflow's field/map/subgroup boundaries; canned CM/GLV
examples are not used. No pure-Python arithmetic, crypto harness or verifier.

## Shared algebra before its native use

Extend the shared Lean library first, at its pinned4.35/mathlib environment,
with generic commutative-ring identities for the cleared doubling quartic and
the degree-two curve residual. Write tau=3xi²+a, D=x-xi, N=x²-xi*x+tau,
a'=a-5tau, b'=b-7xi*tau. The proposed residual is

    (x³+a*x+b)*(D²-tau)² - (N³*D+a'*N*D³+b'*D⁴)
      = (xi³+a*xi+b)*tau*(tau-2D²).

Derive its kernel-root specialization and the affine equation over a field
with D nonzero and y²=x³+a*x+b. Run the canonical shared proof/axiom gate,
preserving failures and exact source/toolchain bindings. These statements
concern actual rings/fields and rational equations; they do not formalize
native parsing, point arithmetic, isogeny homomorphism or complete programs.
The shared projective-coprimality law retains its existing separate scope.
[pcc:unverified]

## Candidate and independent reference

Use native C++ with GMP integers and the pinned FLINT fmpz_mod polynomial
root/square-root interfaces. Keep the old word-prime source unchanged.
The new core accepts canonical prime moduli of3..4096bits, p>=5; t=3..12,
d=2^t. These are public software admission caps, not a growing-family theorem.
Proved fmpz_is_prime admission is charged/protected; no polynomial-time bound
for that exact admission implementation is inferred from the abstract sampler.
[pcc:unverified]

Finite-point preimages use the same cleared doubling quartic, complete rational
abscissa roots, quadratic lifts and actual doubling checks. Sort/deduplicate
roots and sort actual halves lexicographically by x,y. Repeated quartic roots
at order-two targets are allowed; actual halves must still be complete.
The candidate never enumerates Fp. Tiny outer census code remains a separate
control mode and is never called by setup. [pcc:unverified]

Sample a,x0 uniformly under the ideal word source, set b=-x0³-a*x0, reject
singular models, and make t-1 complete uniform rational-halving choices from
K=(x0,0). Verify [d]T=O and [d/2]T!=O. Preserve the old unbiased uint64
rejection mapping whenever the domain fits; use multiword bit rejection for
larger domains. Record every raw draw, rejection, root set, lift set, index
choice and failed path. Fixed seeds are controls, not a distribution proof.

Sample G with the uniform2p+1 encoding, retaining nonsquares, duplicate y=0
signs and infinity. Reject2G in H=<T>, using only the d-element subgroup,
not a field/group-order census. Construct all t actual Vélu steps, codomains,
kernel/point images, final trace-good point, homogeneous N/D composition and
the d distinct coset abscissas. Compare the monic N-cD with their root product.
There is no main-base exact-three filter, target query or DLP collector here.
[pcc:unverified]

The separate C++ OpenSSL replayer includes no candidate/GMP/FLINT code.
Its own BN polynomial arithmetic derives the quartic from f'(X)²-
4*(2X+xp)*f(X), and verifies every reported rational x-root set by
[pcc:unverified]

    product(X-root) = monic gcd(quartic, X^p-X).

It independently lifts roots with BN square roots and checks actual doubles,
including empty sets, using OpenSSL group arithmetic. This certifies complete
finite-point preimages without a large-field scan. It replays the public RNG
and every setup branch, exact order, H membership, coset set, each step's
cleared curve identity, domain/codomain membership, composed polynomials and
final fibre equality. Primality/API trust and native-program formalization
retain their stated scope; an evaluated identity is not substituted for a
field-independent proof. [pcc:unverified]

## Exactly one qualification001 after source freeze

Build fresh strict C++17 candidate, reference and native adverse-transcript
writer. Commit all source before calls, copy execution wrappers separately,
and retain distinct build/pre-call/post-call manifests and all exits. One
native fixture exporter serializes the declared field formulas; shell only
builds/runs/captures native executables. Never overwrite a raw output path.

1. Native admission/structural controls for malformed/composite field,
   unsupported t, singular/off-curve/noncanonical finite targets and the
   old supported small-field interfaces. A failed expected control stops.
2. The same complete14754 finite-point preimage census at primes5,7,11,13,17,19.
   Compare with independently enumerated OpenSSL doubling fibres, in addition
   to the polynomial completeness checks. All models/points stay in this
   separate tiny outer correctness mode.
3. Eighteen generated setups, all seed1234: the old eight fields
   19,23,31,43,59,83,127,251 at t3; plus each of
   2^61-1,2^127-1,2^255-19,2^256-2^32-977,2^521-1 at t3 and t4.
   Every completed case gets full independent replay. The eight old cases
   must semantically match the saved unfiltered qualification001 setup
   receipts; no old producer is rerun. Large moduli identify fields only,
   not standardized auxiliary curves or a security target.
4. Four native adversarial copies of the successful p19,t3 transcript:
   remove one claimed quartic root with an adjusted count; alter a codomain
   a' coefficient; alter a composed numerator coefficient; alter a raw RNG
   draw. The reference must reject at the corresponding completeness,
   curve-identity, polynomial-binding and RNG checks. Keep originals and
   all adverse bytes/statuses; do not implement these mutations in Python.

Limits: one thread,2GiB per native process;120s for the small census;
30s CPU/wall for each setup or reference invocation, and1500s for the entire
frozen wrapper. Sampler bounds remain4096curve trials,64accepted point slots,
128raw point proposals per slot, and4096raw attempts per bounded RNG call.
No field, seed, degree, t, cap or acceptance criterion adapts after calls.
Unknowns retain full available prefixes and are never empty-set conclusions.
Unexpected agreement/checker failures stop the panel. Protect all output paths
and manifests from replacement, and package one deterministic whole raw round
through the existing verified artifact workflow, without shards.
[pcc:unverified]

These are local functional known-answer/software integration controls under
AGENTS9a, with no scientific performance clocks, timing ratio, exponent fit,
benchmark row or paid/cloud job. The parent's abstract probability bound does
not certify this exact program, RNG stream, finite caps or a relation oracle.

## Publication and active dependency

The worktree begins at critical-pair source4b8e8797b8 so its append-only PCC
prefix remains compatible with that pending branch. Parent4302 merged as
230dfbc408, with final head2da6eb137f and current required gates passed.
PR4304 is retargeted to main with auto-merge, but its shared journal needs
the requested Copilot base merge. Keep that journal untouched while pending;
record this continuation in PUBLICATION.md until the base lands and a fresh
main merge permits the combined daily handoff. Do not race its resolver.
No task-wide autonomy completion is recorded. [pcc:unverified]
