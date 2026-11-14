# Next obligation: shifted mode weights and orientation refinement

Prospective analysis, 2026-11-14. This is not a rank-factorization theorem.
Current source freezes only the66 component declarations in the proof map.
Primary source reread: pinned OpenAI Oct25 text D.4.2-D.4.7, as cited in the
protocol, especially D.12-D.15. KB unavailable; no novelty claim.

At a retained chart level i, refining columns to
`y=y0+2^u*k` fixes every retained column bit. For each fixed row x the new
anchor mask then stays constant. The candidate entry becomes

```
C(T)/kappa * 1/(T-delta0-2^u*k)
 * product_(j in mask)
   (2^j-T)^m_j / (2^j-delta0-2^u*k)^m_j,
delta0=y0-x.
```

The actual crossing denominator has valuation at most i<u. Selected anchor
denominators have valuation at most j<u. Hence formal expansions in k have
integral ratios with at least one bit per order. This suggests column
moments `sum k^b X_y`, at weights at least `g+1+b` if the actual correction
schedule gives g. Exact coefficient formulas, integral units and truncation
still need to be constructed and bound for the real local coefficient ring;
this note does not supply them [pcc:unverified].

A naive refinement has up to2^u tiles per packet, instead of2^(i+1).
The source's per-weight variable count and precision are changed jointly;
one cannot retain the old D=L/2^i mode count after this refinement. Even
conditional on the suggested expansion, `g+1+b` is an additive weight shift,
not `(g+1)(b+1)`. The latter unsupported substitution would artificially
divide the weighted partition exponent by a square root of g.

For K variables at every integer weight g+1+b, the exact formal generating
product for unrestricted nonnegative mode powers is

```
product_(b>=0) (1-z^(g+1+b))^(-K).
```

A small exact lower-bound control: take only orders0,...,B and only binary
powers0/1. If `K*(B+1)*(g+1+B)<P`, all `2^(K*(B+1))` such exponent tuples
meet the source's strict weight budget <P. This concerns that explicit
all-mode enumeration, not the actual matrix rank, nonzero coefficients,
a universal lower bound, or every possible algorithm [pcc:unverified].

Using source-scale quantities `P` proportional to `M*L/(2^i*u)` and naive
K proportional to `M*2^u/L`, the old unshifted square-root counting bound
would scale as `M*sqrt(2^(u-i)/u)`. The retained Fibonacci mass constrains g.
Increasing a and L changes both P and K, so a claimed benefit from increasing
g must be checked in the full product, rather than only the lowest mode.
These are conditional symbolic accounting expressions, not measured costs
or an established impossibility result [pcc:unverified].

Next discriminating construction: either an actual factorization whose
weighted moment count survives the refinement and shift, or a hierarchical
orientation construction that avoids enumerating all higher-bit patterns.
Keep the older child-entry, degree, precision and recursion charges explicit.
No paid/native/search campaign is authorized by this note.

## Isolated strict-margin draft

The successor draft at `evidence/hierarchy-draft-022.lean.txt` locally
elaborates a stricter content implication: if all retained m_j>0 and the
actual first differing bit s is below u, a retained crossing has content
at least G_i+1. For s<i the extra term is (i-s)m_i. For i<s<u the omitted
anchor s removes a loss (s-i)m_s. This may localize the cheapest crossing
layer to matching retained bits. No integral low-layer factorization or
all-layer rank statement is supplied. The draft has not yet been promoted
into the independently audited/fresh project; receipt/data claim
PCC-20261114-6f9128 records isolated elaboration only [pcc:unverified].
