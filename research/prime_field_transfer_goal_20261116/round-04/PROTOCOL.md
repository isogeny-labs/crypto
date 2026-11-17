# Round04 — certified Weber-root recognition from a source residue

Original GOAL.md remains active and unchanged. This is a bounded native
development qualification of a concrete root recognizer, not a performance
experiment or a proposed generic ECDLP break. Preserve all37 frozen inputs
from rounds02/03. Only new round04 sources/receipts and necessary publication
metadata may change. Reuse the open study-lane PR4399 if it remains open.

## Literature / mechanism review before implementation

KB unavailable: no loaded KB MCP; the repository wrapper exits127. Conductor,
Cargo, FLINT, fplll and PARI are unavailable. No freshness/novelty claim.
Read the repository CT84 selective Padé experiment, CT115 sign-relaxed
transfer, CT116 orbit-index equivalence, CT119 invariant-only obstruction
and CT124 two-prime CRT chimeras. CT84 already implements classical lattice
recognition for one order-seven input; neither LLL nor that idea is new.
The present qualification will try all order19 inputs and separately bounded
larger small fixtures, rather than treating a labelled orbit scan as selection.

Primary sources read:

- A. K. Lenstra, H. W. Lenstra Jr., L. Lovász, *Factoring polynomials with
  rational coefficients*,1982 preprint, §§1–2:
  https://ir.cwi.nl/pub/9304/9304D.pdf. Use exact rational Gram–Schmidt,
  size reduction at1/2 and Lovász delta3/4. A short numerical relation is
  a candidate; exact global algebra is the acceptance gate.
- Bourdon–Clark, *Torsion points and Galois representations on CM elliptic
  curves*, arXiv1612.03229v4, Theorems2.8/2.10 and Lemma7.6:
  https://arxiv.org/pdf/1612.03229. A Weber function quotients by the CM
  unit group. For j=0 use w=x³/B; the cyclic prime-ideal kernel's Weber
  field has degree d=(r−1)/6 over K=Q(omega). This is classical CM.
- Lauter–Stange, *The elliptic curve discrete logarithm problem and
  equivalent hard problems for elliptic divisibility sequences*,2008,
  Theorems1,6,8–10: https://arxiv.org/pdf/0803.0728. A normalized EDS
  value is efficiently coordinate-computable, but the unnormalized phase
  needed for its finite-field-DLP reduction is a separate hard problem.
  Do not substitute normalized terms into that reduction.
- The elliptic-Gauss-sum paper was located, but its primary PDF returns403.
  Do not attribute an unread fast resolvent algorithm to it. An explicit
  full-orbit Gauss sum would conflict with the no-enumeration objective;
  a compressed weighted-trace method remains an unimplemented alternative.

## Exact proposed field and cover

Use u in {3,5,7}, b=2u−1, r=3u(u−1)+1, so r=19,61,127. All are
small development fixtures. In K, omega²+omega+1=0. The kernel of
[(u−1)+b*omega] equals ker(1−pi), pi=u+b*omega, and has order r.

On y²=x³+B write x([n]T)=x*f_n(w), y([n]T)=y*g_n(w), w=x³/B.
Construct these rational functions by exact addition/doubling, independent
of B. Their common kernel equation is the monic gcd

    C(W)=gcd(num(f_(u−1)−omega*f_b), num(g_(u−1)+g_b)).

Require deg(C)=d and coprimality with every relevant denominator. Retain
the exact quotients certifying divisibility, not just local root checks.
Find an auxiliary split prime ell≤257 and a cube-root embedding where
C mod ell is irreducible (Rabin finite-field criterion). If missing, keep
the missing certificate/failure; do not change the fixture.

For the source, choose the first c in2..32 for which p=c²r+c+1 is
prime, with p≠r. Use exact small-prime qualification. Select the first
sextic twist of cardinality c²r, using direct small-field counts, and a
hash-derived nonzero cofactored generator. For u3 reuse the round03 pinned
generators (42,31) at p79 and (4,1) at r19 on the same first twists.
At the target choose the first
anomalous sextic twist over Fr of cardinality r. These counts admit
fixtures; they are not source DLP tables. CRT produces B.

Pin omega at p to (1+c(1−u))/(cb), at r to −u/b. Require generator
Weber values to be roots of C with nonzero derivatives. The fixed maps
K[W]/C→Fp and →Fr send W to the respective generator's Weber value;
the same formal W is used at both places for every input.

## Evaluator and exact certificate

The core receives model, encoded Q and a source precision N; no scalar,
group table or orbit label. For the field basis1,omega,W,omega*W,…,
W^(d−1),omega*W^(d−1), Hensel-lift the fixed generator embedding and
the input-selected C-root to p^N. Construct the (2d+1)-dimensional integer
congruence lattice

    sum(a_i*s_i) − D*t_Q = 0 mod p^N.

Run exact LLL with delta3/4, a10000-step cap, and inspect only its reduced
basis rows. There is no scan of combinations or subgroup points. Retain
failed/short/spurious candidates. Recompute Gram–Schmidt, check size and
Lovász conditions, and check the unimodular basis-change certificate.

For a proposed beta=sum(a_i*basis_i)/D require D≠0, its source evaluation
equals w(Q), and **C(beta)=0 in the exact global quotient algebra**.
Irreducibility makes this a field; distinct source residues distinguish
the global roots. These exact checks certify accepted candidates without
assuming that LLL succeeds at a prescribed precision. No general recovery
probability or coefficient-height bound is claimed.

Evaluate this same beta at the fixed target place. Handle denominators
divisible by r by lifting omega and the fixed target C-root to v_r(D)+1
digits, cancelling only the certified local valuation. Do not reduce an
undefined rational denominator modulo r or treat integer representatives as
a field map. Output the target Weber class before scalar recovery.

Decode its six possible affine target points by native cube/square roots.
Use the classical Smart solver, then at most six CM-unit source comparisons
to resolve the scalar. Target roots use ternary digit lifting, not a scan
of Fr. Independent OpenSSL source/target multiplication checks the answer.
This is a scalar reduction with a sixfold ambiguity, not a group law on
the Weber coordinate itself.

## Fixed correctness calls, controls and limits

Setup each u once, save its exact model as a separate input, then freeze it
before dependent calls. Each setup or qualification call has a60-second
limit. One original call per declared bin, up to two focused diagnostic
calls before a protocol amendment. No wall-time comparison or paid run.

- u3: all18 nonidentity source multiples, scalar labels confined to the
  auditor. Try N=1,2,4,8,16,32, without changing the protocol after seeing
  outputs. A failed N is retained, not called a successful map.
- u5 and u7: four source points from fixed hash labels per fixture; per
  point try N=4,8,16,32. No generating scalar is supplied to the evaluator.
  The input API and source verification must be inspectable. Split calls
  by public-point index, each still capped60seconds.
- A wrong genuine global root must pass a global/local-target root check
  but fail the input's source-residue gate. A CT124-style element satisfying
  both local equations must fail the exact global-root gate. A mutated
  numerator and wrong target cube-root orientation must be rejected.
- Verify all accepted r19 Weber images against the auditor's target
  relation; larger accepted answers against OpenSSL source multiplication.
  Reject k+1. No claim that every precision or larger bin succeeds.

Native C++/GMP/OpenSSL only; Python may run existing governance or plot
recorded values, never crypto algorithms/harnesses/tests. Keep source and
0555 executable qualifiers immutable, separate build/pre-call/post-call
manifests, refusal to replace pre-call records, errors and timeouts. This
is local development under the isolab-eks unit-test exception, not an EKS
performance experiment or independent preregistration/execution proof.

## Interpretation and next gate

An accepted beta would supply a concrete input-selected same-field class
for this small model, exceeding two unrelated local root fits. The dense
field degree d and lattice dimension2d+1 grow with r; this implementation
does not establish an efficient cryptographic-size map or arbitrary-curve
applicability. Charge field construction, precision, LLL, certification,
local cancellation, target solve and source checks. A compressed recognizer
without that dense dependence remains the research obligation. Preserve
the original goal and do not mark it complete from this small qualifier.
