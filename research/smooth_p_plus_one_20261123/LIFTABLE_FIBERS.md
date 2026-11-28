# Liftable-base factors inside fine fibers

2026-11-25. Construction argument and protocol, frozen before execution.
Standing: producer-local, unsigned. No checked Lean theorem or cost result.

## Mechanism and prior sources

Keep the existing public base B, fine map phi=T_h with h=d-3 in {1,2},
capacity catalog and RR5 norm quartic. For each occupied fine image b define

\[
 B_b=\{x\in B:\phi(x)=b\},\qquad
 H_b^B(X)=\prod_{x\in B_b}(X-x).
\]

Replace H_b=phi-b in the [factorwise presentation](FACTORWISE.md) by H_b^B:

\[
 H_b^B\bmod Q_b=0\quad\text{for each occupied }b,\qquad
 Q_\theta=\prod_b Q_b,\quad \deg Q_b=m_b.
\]

The base contains unsigned abscissae with both signed lifts available. Building
these polynomials uses only the public base and map, without target relations,
base logarithms, tuple lists or sign assignments. Grouping and polynomial
construction are charged once inside each query deadline, before chart solving.
The same chart order and multiplicities are retained. Seven variables and
eight equations remain; the largest fiber degree is at most four in this
qualification, and the quartic product can still have input degree four.

The lab's [quotient-product construction](../prime31_four_summand_20261122/QUOTIENT_PRODUCT_ADDENDUM.md)
already encodes division by the whole liftable-base polynomial. This applies
that existing membership idea to individual small fibers. The previous
[factorwise output panel](FACTORWISE_EXTRACT_RESULT.md) motivates removing
nonliftable roots but does not show they caused its timeouts. PKC2016
[Algorithm 1 and sections 3.2/3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
already provide composed-map/auxiliary-isogeny bases and a cost chain including
precomputation, relation solving and linear algebra. The first-collision study
in `../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md`
requires the whole branch sum. This is a presentation refinement, with no
claim of a new base family, isogeny implementation or complexity bound.

Sources checked (KB): status, "fiber factor-base membership monic divisor
split polynomial", and "nonliftable abscissae quotient product radical fiber
exclusion" all fail with wrapper exit127; no KB MCP is loaded. Exact repository
sources, the preceding frozen arguments and cached primary paper are the
fallback. The review does not establish exhaustive novelty.

## Guarded coverage and tangent argument

Every H_b^B is monic, split and square-free because its roots are distinct
base abscissae. Different fibers have disjoint roots, hence their polynomials
are coprime. For an admissible relation, the monic factor formed from its
leaves over b divides H_b^B and the product equals Q_theta. Conversely, any
solution has four distinct roots in B with exactly the prescribed image
multiset. Its factors are uniquely determined by that root set. Signed lifts,
nonzero denominators, target-abscissa exclusion and the group sum remain the
extractor's mandatory guards. Thus the guarded relation set and unique chart
assignment are preserved. [pcc:unverified]

For H_b^B=Q_b W_b, separability gives gcd(Q_b,W_b)=1. Differentiating monic
division and reducing modulo Q_b gives delta Q_b W_b=0 modulo Q_b, so
delta Q_b=0. Consequently the remainder Jacobian has rank four on the factor
coefficients. The previous RR5 norm-differential argument applies unchanged
at every admissible binding: five distinct affine zeros of delta F in L(4O)
force its three parameters to vanish. The combined Jacobian therefore has
column rank seven on that open chart. [pcc:unverified]

This argument does not claim equality of unsaturated ideals, global radicality,
that every fixed form separates all solutions, or solver completion. Removing
nonliftable roots can reduce algebraic work; public-base construction and
per-chart solver overhead can offset that change. The experiment below tests
complete output under fixed limits, without inferring a timing ratio.

## Formalization and inheritance map

The [Lean instance gate](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/lean-instance-gate/SKILL.md)
was read at Git blob7d3ffa79b65a92cdcbe1df02043a15e426f3b5c4. It says:
"If a field cannot be filled, rewrite the task ... or record why no instance
exists. Do not start building to find out." No Lean declaration is written
or claimed checked here. The exact library bridge and assembly probe remain
open; this is distinct from the native finite audit.

| Gate field | Current instance or missing bridge |
|:--|:--|
| Structure above | Polynomials over a field; monic divisors and separability; elliptic-curve rational functions and their divisors |
| Theorem used as stated | Exact checked divisor-differential and RR5 function/divisor declarations have not been identified; KB unavailable |
| Object as instance | H_b^B is the finite product of distinct linear factors; Q_b is monic; RR5 F is unchanged |
| Assembly probe | Not type-checked; no local lean/lake or pinned compiled library environment |
| Missing components | Generic monic-remainder differential bridge, normalized RR5 norm/divisor bridge and coordinate-local-ring bridge, owned in crypto-lean |
| Inheritance | Field and polynomial operations inherited; disjoint base roots give coprimality; exact Lean links remain open |

Dependency order is finite-product separability and remainder differential,
then the RR5 function/divisor bridge, then the study instance. The prior
metadata-only library check names Lean4.35.0 and Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2; it is not a built proof environment.
Guarded coverage and rank are written arguments with no Lean declaration,
proof-gate receipt or axioms report. No weaker formal theorem is substituted.

## Frozen qualification and falsification

Add a separate `liftable-cutoff` fixed-panel method in the shared extractor.
The old full-cutoff and factorwise methods retain their constructor choices.
Reuse catalog traversal, sixteen forms A=t0+j*a1+j^2*a0 for j=0,...,15,
msolve `-c 0`, conservative eliminant-degree/quotient-dimension guard,
20,000 construction terms and 262,144 escaped-JSON capture bytes. Each query
has one absolute three-second deadline covering membership construction and
all charts, forms, extraction, replay and receipt construction. Base/reference
setup and final JSON emission remain outside it. No incomplete union is an
answer, and no failed chart receives a new allowance. Recovery/accounting
reject this method pending separate qualification.

After source controls and a published executable identity, run three
`liftable-audit` finite witness/Jacobian controls, one on each existing fixture:
F127 depth4 and F8191 depth4/depth5. Each has one worker,2GiB address space,
120 seconds plus five seconds termination grace. Attempt all three audits.
They enumerate the same tuples and signs as the prior audit, retain identity
and target-overlap exclusions, check actual equations, fiber rank4, RR5 rank3
and total rank7, and reconcile counts with that audit. Record every fiber's
public root list and polynomial, chart sizes and ranks. No solver is used.

If any audit fails, preserve the failure and do not start the solver panels.
If all pass, run one paired panel per fixture, candidate then unchanged
cofactor-msolve on each of the same thirteen targets: the four existing F127
targets; F8191 [17]G,[43]G,[89]G,[152]G at each depth; and the depth5 planted
target (4375,6159). Attempt all three panels, each with one worker,2GiB address
space and60 seconds plus five seconds termination grace. A mismatch stops
its panel with a recorded failure; missing suffixes stay explicit. Thus
three finite-audit records and26 planned query records are expected if the
qualification gate passes. No retries, target changes, cap increases, new
recovery draws or EKS/paid runs are part of this component.

Predictions: admissible witness counts and ranks should remain unchanged;
full answers must match Direct. Membership errors or mismatches falsify the
implementation. Increased degree32 completion would support bounded solver
utility; unchanged or worse completion preserves the negative result. Neither
outcome establishes admitted cost or scaling. The historical factorwise panel
is retained, not rerun as a new paired timing observation.

Source tests check exact finite-field membership of each constructed fiber,
malformed or duplicate base inputs, rejection of an extra full-map root by
the new remainder equations, and dispatch/variable order/full traversal with
a labeled transport stub. The existing completion/failure/capture tests stay.
Keep every raw exit and stderr, completed RUR and prefix, before/after identity,
checksums, claims, unsigned standings, reproduction and inspected report/PDF.
The wrapper still does not archive separate child stderr or partial timeout
files. Flow metadata uses the distinct development schema; canonical export
validation remains unavailable. The live hook captures new JSONL records;
delivery awaits the receiver URL.
