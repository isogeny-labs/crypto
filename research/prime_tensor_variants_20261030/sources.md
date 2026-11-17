# Source and novelty scope

Literature was reviewed before choosing these follow-ups. This study builds on
the preceding pinned source audit rather than restarting or silently replacing
its sources.

1. Akshima et al., *A Subexponential Time Algorithm for Elliptic Curve Discrete
   Logarithm*, v0.2, October 29, 2026. [Pinned PDF](https://raw.githubusercontent.com/bochum-crypto/subexponential-ecdl/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf).
   SHA256 `82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11`.
   Compact Fourier arithmetic, complete point addition, independently hashed
   factor bases and complete relation/extraction cost are existing work.
2. *The Exponential Time Hypothesis is False*, October 25, 2026.
   [Pinned PDF](https://raw.githubusercontent.com/openai/math/7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf).
   SHA256 `5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`.
   Appendix D.1 and the complete D.4 proof supply the binary packet/deformation,
   tapered Hermite, weighted-mode and recursion template. The ternary work is
   an adaptation of that template, with credit retained.
3. Stange, *Elliptic Nets and Elliptic Curves*, arXiv:0710.1316v4. Exact
   source/hash, recurrence statements and limits of the division-polynomial
   proposal are in [the delegated source review](reviews/division-polynomials.md).
4. Existing repository evidence, especially the
   [norm-encoding size check](../prime_one_shot_20261029/continuation/tensor-review/norm-encoding-size.md),
   contraction cost/precision reviews, and exact Hermite unit. The literal
   norm encoding still does not inherit the hashed construction's compactness.

The October 30 public search used the literal queries `"tensor contraction"
"3-adic"`, `"tensor contraction" "ternary" Cauchy Hermite`, and
`"The Exponential Time Hypothesis is False" ternary`. The indexed material
returned the existing binary manuscript and unrelated uses of ternary
contraction; this is a narrow search, not proof of priority. Local full-text
searches of the two pinned manuscripts found no explicit ternary/three-adic
version. The atlas metadata query is retained in
[receipts/atlas-match.txt](receipts/atlas-match.txt); its sparse textual ranking
is not a complete historical or mathematical novelty audit.

The web search also identified the source project's
[Lean scope page](https://github.com/openai/math/blob/main/lean/docs/378.md).
It describes a formalized SAT/counting bound with additional iterated-log
factors and distinguishes it from the sharper manuscript bound. Reading that
scope page is not a local recheck of the source project's proof, nor does it
formalize this ternary variant or the ECDLP reduction. Our formalization status
is reported independently in the proof map. Public repository names establish
provenance, not institutional endorsement or a correctness certificate.

No general theorem is inferred from a failed search, an unrun experiment, a
tiny fixture or an agent's assertion. The reviews state exactly what each
reviewer read and what remains outside their check.

The later matched-level recovery analysis applies the same pinned D.4.4
Hermite argument to `G(T)=F(T,T)`; it is not an independent attribution for that
method. The signed binomial formalization reads the actual Mathlib Lucas,
generalized `Ring.choose`, rational-polynomial evaluation and integer-valuation
definitions at the same pinned Mathlib revision. Its exact imports and theorem
scope are in `lean-binomial/README.md` and `PROOF_MAP.md`. A new common-kernel
construction and its priority remain unresolved; these calculations do not
establish an original algorithm.
