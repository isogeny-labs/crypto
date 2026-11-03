# Concurrent public hashed-ECDL tensor lead: narrow source triage

2026-10-30 UTC. Reviewer `/root/continuation_integrity`. Frozen repository
revision inspected: `31f31e0f5b40b45b0a5d475111790d79f6c6acfe` of public
`isogeny-labs/crypto`. This is concurrently landed material absent from the
original frozen source audit. No source, raw evidence, problem-index row or
session-log entry was edited. No implementation, test, contraction, benchmark or
challenge computation was run. Only public documents/source were read; the two
public primary PDFs were retrieved and text-extracted for inspection.

## Classification and effect on the research decision

This is an **existing, explicit claimed classical sub-exponential prime-field
ECDLP method**, with a reduction conditional on an external claimed theorem.
It is materially stronger prior art than a speculative factor-base suggestion.
The external theorem is stated with a proof in its own manuscript; it is not
merely assumed or posed as a conjecture there. The Bochum paper explicitly makes
its ECDLP conclusion conditional on the correctness of that external result and
says that result is under external review.

Here “external” identifies the actual separately retrieved public preprints in
the pinned `bochum-crypto/subexponential-ecdl` and `openai/math` repositories,
as distinct from Isogeny Labs' implementation report. It does not mean a
peer-reviewed publication, an authenticated institutional endorsement, or a
correctness certification by the organization named in a repository/author
label. This audit verifies the retrieved documents' exact content and revision;
it does not infer validity from the name “OpenAI.”

This narrow triage does not independently establish or refute that proof. The
repository correctly leaves its general mathematical and implementation audit
open. Accordingly it does **not** give this task a newly verified breakthrough
or an algorithm invented in this session. Conversely, absent benchmarks and
astronomical finite-size constants do **not** refute the claimed asymptotic
method. If the complete external proof and reduction are valid, the displayed
runtime is sub-exponential in the ordinary classical bit model.

The lead must be included in the current prior-art landscape and any renewed
selection rationale. Merely transcribing or rediscovering it would fail the
user's requirement to produce a novel method beyond existing algorithms.
Nothing in this limited audit verifies it as a replacement for the selected
norm/divisor research bet. A full proof audit could change that assessment; the
current result is not a mathematical rejection of the tensor direction.

## Primary theorem chain actually checked

1. **Akshima et al., _A Subexponential Time Algorithm for Elliptic Curve Discrete
   Logarithm_, version0.2, October29,2026.**
   Pinned public revision:
   <https://github.com/bochum-crypto/subexponential-ecdl/tree/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f>.
   That tree contains the PDF, not executable code. Read the abstract,
   Introduction1.1-1.3, Theorem2/Remark1, integer/modular encodings in
   sections2.3-2.4, and the prime-field reduction in sections3.1-3.5,
   including Theorem4 and Lemmas9-13.

   The main input is a valid nonsingular E/Fp, p>3, with known odd prime
   whole order r, a generator P and target Q. Put b=ceil(log2 p). There is
   no p-1/p+1 congruence restriction. A separate paragraph extends the
   reduction to a **fixed** cofactor c, with known #E=cr, prime r>c, by
   encoding A=[c]S and checking enumerated points lie in the subgroup.

   Theorem4 first gives an explicit reduction to Boolean tensor contraction.
   With its Theorem2, the claimed expected bit runtime and total space are

       2^{O(b/(log b)^{1/4})}.

   This is 2^{o(b)}. It is not an L_p[1/2] claim, and no useful concrete
   bound for P-256 is asserted. The paper charges inverse-hash base
   construction, expected-polynomial finite-field roots, exact prime
   testing/prime selection, failed target attempts, seed restarts, N+1
   relations and dense linear algebra. It does not assume supplied base
   logarithms, a full-rank factor-base matrix or a target-log oracle.

2. **OpenAI, _The Exponential Time Hypothesis is False_, October25,2026.**
   Pinned public revision:
   <https://github.com/openai/math/tree/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026>.
   Read the release README, AppendixD.1 theorem statement and explanation,
   and the relevant precision/parameter/model and final proof passages in
   D.4.1, D.4.2, D.4.6-D.4.7. This was not a full independent proof audit
   of the packing, modes, interpolation and precision invariants.

   TheoremD.1 explicitly claims deterministic exact contraction of bipartite
   Boolean-edge tensor networks in

       (s+L)^{O(1)} 2^{O(s/sqrt(log s))}

   **bit operations**, with polynomial space, under bounded size, description
   length and vertex degree (at most ceil(s^{9/10})). Ring entries have
   precision P0 and truncation length ell at most Cs. Its allowed local
   evaluation cost is (s+L)^b 2^{A s^{9/10}} with polynomial space, so the
   Bochum encodings' polynomial-time predicates are within that hypothesis.
   This is a claimed theorem with an explicit D.4 proof, not a conjectural
   black-box assumption in the OpenAI manuscript itself.

   Bochum's Remark1 subdivides edges to make the graph bipartite, takes
   ell=1 and precision P0=E+1 for E Boolean edges, and recovers the integer
   count uniquely because it lies between0 and2^E. Constant-factor size
   adjustment accommodates the precision and subdivision. D.4 describes
   exact ring arithmetic, canonical creation-precision lifts and recovery;
   the claim is not based on free real arithmetic or unspecified infinite
   precision.

## Why the prime-field inputs and public interfaces fit the stated model

The crucial encoding claim is not a generic circuit of O(b log b) edges.
Bochum Theorem3 constructs integer multiplication with O(b) edges and degree
O(b^{3/4}) using a two-stage Fourier construction. Each moderately large
vertex checks an explicit polynomial-time arithmetic predicate. Canonical
quotient/remainder identities give modular arithmetic. Section3.1 then covers
point validity, identity, inverses, ordinary addition and doubling. Section3.3
adds the hash tests. Thus a relation network has O(mb) edges, polynomial
description/evaluation cost and degree O(b^{3/4}), within the external degree
cap asymptotically. No elliptic logarithm computation is hidden in a vertex
predicate in the inspected construction.

The bases are public coordinate predicates. For independent public affine
hashes H_i over a prime in [2p,4p],

    enc(A)=2x+(y mod2),
    F_i={A!=O : H_i(enc(A))<tau}.

Listing a base inverts H_i for each t<tau and extracts a curve root. This
charged cold listing indexes matrix columns; relation membership does not
consult that list. Decomposition receives R and the fixed hash seed only.
Mixed labels u,v are retained separately for recovery, and the dependency is
chosen from the relation matrix only. This is explicit in the paper's
section3.4/Lemma12, not an assumed known logarithm.

The native source at the frozen crypto revision follows that public interface
in `crates/hashed-ecdl-tensor/src/reduction.rs`: `decomposition_network` takes
curve, bases and target point, uses their public hash parameters, and is not
passed the mixed scalar labels or enumerated column maps. The outer solver
retains those separately.

The native **Functional** leaf also does not require target logarithms or
precomputed decompositions. `Encoder::finish` supplies private `Program`
metadata made from its ordinary arithmetic definitions/checks.
`Program::materialize` evaluates a guessed free-bit assignment and verifies
the resulting full graph assignment. `Counter::functional_witness` enumerates
those free assignments. The metadata is not user-supplied solved-instance
advice; arbitrary caller predicates cannot attach it. This path may be
exponential, but its inputs are obtainable from the public encoding. It is
distinct from the claimed general recursive contraction backend.

This source-level interface check does not certify all arithmetic, packing,
mode, precision or reconstruction invariants for arbitrary instances.

## Actual implementation and empirical evidence

The public problem record `problems/hashed-ecdl-tensor-backend.md` accurately
classifies an implemented backend with independent general audit open. Its
native report `docs/hashed-ecdl.md`, crate README, tensor validation receipt,
and the later `research/hashed_ecdl_speed_20261029/README.md` preserve the
following boundaries:

- Bounded exact arithmetic/graph/recursive controls and a tiny complete
  seeded relation certificate exist. Later optimization controls report34
  native tests and equal complete certificates. These are development
  correctness evidence, not proof of asymptotic behavior.
- The original tiny native receipt reports backend Functional and zero
  recursive calls. Forced small-recursion controls bypass size admission
  and compare to exact direct counts. They exercise implementation pieces
  without testing the whole source theorem at its admitted asymptotic sizes.
- Literal D.4 parameters give root u=floor(ceil(log2 s)/131072). Practical
  graphs select the source's exact finite-size fallback. This is faithful
  finite-size behavior and **does not refute an asymptotic theorem**.
- The later `comparison.json` says `performance_panel_status=not_run`;
  runtime ratio, common-operation ratio, S/rho and confidence interval are
  null. Both published challenge scalars remain unresolved. Certified
  ECCp-131 parameter validation is not relation finding or scalar recovery.

No verified scaling or timing evidence in these inspected artifacts establishes
the source bound. Such evidence is also not logically necessary for a correct
asymptotic proof: the open question is validity of that proof chain and the
general implementation's compliance, not whether today's finite fallback
beats rho.

## Stable repository references

All paths below were read through `git show` at the frozen public revision;
off-disk sparse files were not treated as absent and no checkout was changed.

- [Problem record](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/problems/hashed-ecdl-tensor-backend.md).
- [Native report](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/docs/hashed-ecdl.md).
- [Native validation](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/docs/hashed-ecdl-tensor-validation.md).
- [Optimization receipt](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/research/hashed_ecdl_speed_20261029/README.md)
  and [machine status](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/research/hashed_ecdl_speed_20261029/comparison.json).
- [Reduction source](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/crates/hashed-ecdl-tensor/src/reduction.rs),
  [encoding source](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/crates/hashed-ecdl-tensor/src/encoding.rs),
  [functional metadata](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/crates/hashed-ecdl-tensor/src/network.rs),
  [contraction dispatcher](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/crates/hashed-ecdl-tensor/src/contraction.rs)
  and [recursive parameters](https://github.com/isogeny-labs/crypto/blob/31f31e0f5b40b45b0a5d475111790d79f6c6acfe/crates/hashed-ecdl-tensor/src/contraction/recursive.rs).

Retrieved primary PDF SHA-256 values:

    82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11  bochum-v02.pdf
    5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123  openai-oct25.pdf

They and extracted text are retained outside Git under
`/workspace/audit/tensor-primary/`. No paired/private repository source was
needed for this triage. Preserve the concurrent problem-index and session-log
entries during the parent's merge.
