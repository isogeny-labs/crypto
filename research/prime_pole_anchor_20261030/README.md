# A degree-one pole correction for the common kernel

2026-10-30. A concrete analytic completion inside the preceding
[jet-completion proposal](../prime_jet_completion_20261030/README.md).
**No new sub-exponential ECDLP algorithm, improved exponent or measured
end-to-end speedup is established.** Novelty of this application is unverified;
the underlying two-pole resolvent identity is elementary prior mathematics.

## Construction

Use the source grid S={-L,...,L-1}, L=2^a, alpha=2L, nodal polynomial C and
normalizer C'(0). For real row/column slots let delta=y-x. Define

    B_delta(T)=C(T)/(C'(0)(T-delta)),
    W_delta(T)=B_delta(T)-1[delta even]*C(T)/(C'(0)(1-delta)).

The second denominator is always odd on the entries where it is used.
The correction is nonscalar and has the exact form C(T)H with H constant.
Thus it belongs to the proposed completion search space without requiring
a numerical search to construct it. Every old node value and W(0)=I are
preserved. Maximum matrix degree rises from2L-1 to2L, rather than roughly
doubling as in ordinary derivative flattening.

The two selected ideas are combined here through one fixed polynomial:
the origin remains the identity, while its off-node continuation changes
across matrix entries. This does not implement the earlier two-register
syndrome kernel or its full-rank selector. It is a new candidate within the
original one-axis Cauchy backend; the two-axis density objective stays open.

## What changes in crossing modes

At an odd interpolation node t, crossing entries have delta even. The exact
identity is

    W_delta(T)=C(T)(1-T)/(C'(0)(T-delta)(1-delta)).

The source already supplies alpha divisibility of C(t+alpha*z)/C'(0).
Both displayed crossing denominators have odd constant terms, and1-T is
even throughout this chart. Thus the correction gains one additional bit
uniformly at odd nodes. At t=1 it also supplies an extra alpha*z factor.
This compares with the source's sharper content bound, not merely its
coarse weight-one convention.

For column tile y=y0+2j put d=T-y0+x and e=1-y0+x. Its j^b coefficient is

    C(T)(1-T)/C'(0) * 2^b *
       sum_(r=0)^b d^(-r-1) e^(-(b-r)-1).

Both d,e are units. Hence the original moment source
`sum_y j^b X_y` can be reused, without a second index or extra tile family.
The coarse guaranteed weight increases from b+1 to b+2; the sharper odd
content increases from a+1+b to a+2+b. Actual cancellations can be stronger.

At even retained levels, odd-delta entries are unchanged. For even-delta
entries use tile stride s=2^(i+1), v=v2(T-y0+x)<=i. Cancel2^v first.
The rth product-denominator term has valuation at least

    a+1-v + r*(i+1-v) + (b-r)*(i+1) >= b+1.

The factor1-T is integral, so the old guaranteed coarse weight is retained.
Only odd unit inverses remain after cancellation. A fixed tile has constant
column parity; the choice of formula depends on its row and tile, so it
does not require new moment source variables or new targets.

These are written coefficient arguments. Complete recursive integration,
formalization of their valuation hypotheses and a proof-to-code audit
remain open. A finite shift in weights does not itself change the source's
square-root counting exponent or the inherited ECDLP exponent.

## Degree and precision obligations

The parent contraction now has degree at most2ML. The source's odd-node
Hermite multiplicity alone is L*h*u>=3ML, so it still exceeds this degree.
Its denominator and local-jet precision estimates depend on the same nodes,
multiplicities and chart scale. This explains why the extra degree need not
force a doubled recovery budget; it is a written extension of the source
argument, not a formally certified global algorithm. Packet arities and
entry construction costs must still be audited in a full implementation.

Global chart integrality follows by subtracting an integral correction:
C(t+alpha*z)/C'(0) is coefficientwise divisible by alpha, and1-delta is odd.
No inverse of an even residue is used. The original lifted vertex tensors
are unchanged; a different scalar lift or a different polynomial by level
is not being substituted.

## Exact development verification

[Protocol](native/PROTOCOL.md), [source](native/check.cpp), frozen output and
replay helper are retained. Version1 checked odd levels. Version2 extends
the same fixed construction to every nonzero active level v2(t)<log2(L),
before executing that extension. These are correctness units, not a search
campaign, EKS timing result or ECDLP execution.

| L | Degree old / new | Minimum odd crossing chart content old / new |
| --- | --- | --- |
| 4 | 7 / 8 | 3 / 4 |
| 8 | 15 / 16 | 4 / 5 |
| 16 | 31 / 32 | 5 / 6 |

All1288 complete node/difference charts pass nodal preservation and37320
coefficient integrality checks. Exact modular reconstruction passes52032
entries at precisions8,12,16 and zeta=-1,0,1,3, with624384 weighted
coefficient checks. There are37392 nonzero reconstructed entries and30024
entries changed from the original, so the certificate is not vacuous.
The even-inverse negative control is rejected. Integer zeta samples do not
by themselves certify every extension-ring substitution.

The first strict compile failed on missing `<cstdint>`; its exact source
and reproduced diagnostic are retained. The corrected construction passes;
this implementation error was not a mathematical failure. No host timing
is used to explain or interpret exact arithmetic.

Run `bash native/check.sh /tmp/pole-anchor-replay`.

## Sources, formal scope and remaining work

Read before construction: pinned Oct25 manuscript D.4.3-D.4.6, especially
(D.7),(D.12), and the existing jet/selector/packing notes. Source provenance
is in [the preceding bibliography](../prime_common_kernel_20261030/sources.md).
The additional atlas query is metadata-only and recorded in atlas-match.log;
it is not a complete priority review or evidence of absence of earlier work.

The pinned Lean project checks the rational two-pole identity and polynomial
node-preservation identity. It does not prove 2-adic integrality, the mode
valuation bounds, Hermite extension, recursion, or improved ECDLP complexity.
See its exact proof map and actual verification receipts.

Next: integrate and independently verify the corrected child-mode identity,
then price its exact tuple counts against the sharper original baseline.
The promised broader bounded rank-completion search has not run: an analytic
candidate was found first and tested as a development correctness unit.
Scientific search/timing work uses the repository's EKS workflow; its skill
was read, but no EKS submission or background search is claimed here.
