#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.2pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Five cryptographic assumption boundaries

#text(fill: rgb("2f7d4a"))[Adversarial Lean audit · local proof gate PASS · PCC unsigned · 2026-11-17 · PR #4597]

== Scope

This report pairs five omitted-hypothesis countermodels with sufficient
positive or exact boundary endpoints: finite transcript simulation versus
soundness/extraction, Schnorr scalar-map injectivity, Fiat--Shamir transcript
framing, KZG trapdoor-enabled openings, and survival-tail convergence versus
finite cumulative tail.

The negative models target underspecified abstractions. They are not claims
that correctly parameterized Schnorr, canonically encoded Fiat--Shamir, or
hidden-trapdoor KZG is broken. At source commit `4ce098bdb0`, the project built
and the repository proof gate audited 158 theorems with zero failures and zero
trusted native-decision results. PCC verdicts remain unsigned.

#figure(
  image("attack-boundaries.svg", width: 100%),
  caption: [Each row separates an omitted premise (amber), an explicit negative
  model (red), and a sufficient premise or exact boundary (green). The bottom note records
  the refinement boundary.],
)

== Five boundaries

=== Zero knowledge

GMR define completeness, soundness, and zero knowledge separately.
Bellare--Goldreich define extraction with computational and oracle structure.
A finite always-accept model can have completeness and equality between one
deterministic honest transcript and its simulator output while failing
soundness and a global accepted-transcript extraction property. That property is stronger than
Bellare--Goldreich only in per-transcript and false-instance coverage, while
omitting their computational obligations. Its failure must not be renamed “not
a proof of knowledge.” The bespoke equality predicate is not GMR zero
knowledge, which quantifies over polynomial-time verifier views.

=== Schnorr scalar bridge

In `ZMod 6`, multiplication by `2` is noninjective: `3` and `0` collide.
Prime-order Schnorr excludes this model. Over prime `ZMod q`, a nonzero
multiplier is injective by field cancellation. The repository's separate
`JKSSTCyclicECDLP.orderScalarMap_injective` theorem supplies the abstract
exact-order bridge; concrete point and parameter checks remain implementation
obligations.

=== Fiat--Shamir transcript encoding

The distinct pairs `([1],[2,3])` and `([1,2],[3])` have identical raw
concatenations. Every hash receives the same bytes, so this is an encoding
alias rather than a hash collision. Ordered domain tags, field tags, lengths,
and full-consumption decoding give a left inverse and hence injectivity. A
byte codec still needs a canonical bounded length representation. This does
not prove draft-04 prefix-freeness or protocol reachability.

=== KZG toxic waste

For the scalar-coordinate surrogate `c=q(tau-z)+y`, knowledge of `tau` gives
`q=(c-y)/(tau-z)` when `z != tau`; substitution accepts a chosen `y`. At
`z=tau`, verification reduces to `c=y` independently of `q`. The evaluated
identity certificate records only the polynomial simplification `(c-y)+y=c`;
its id is `IDC1ha9b3d803847b97b6`. It says nothing about inverses, pairings,
degree, ceremony behavior, or a security reduction, and no equivalence to a
pairing verifier is proved.

=== Survival tails

`S(n)=1/(n+1)` tends to zero while its partial sums diverge. Vanishing
survival alone therefore does not bound the cumulative tail. A uniform lower
stopping hazard with `0 < delta <= 1` gives
`S(n+1) <= (1-delta)S(n)`, a geometric upper bound, and
the finite inequality

$ delta sum_(n < N) S(n) <= 1 - (1-delta)^N. $

It follows that $sum_(n < N) S(n) <= 1/delta$ uniformly in `N`.

The committed predecessor supplies the tail-sum identity for finite truncated
waits; an infinite/nontermination bridge remains open. Under Theorem 4.1's
assumptions, JKSST prove `Pr[success] < C0/p` for one run but do not state the
adaptive joint law needed for the repeated-trial step; “at least O(p)” should
use the lower-bound direction `Omega(p)`.

== Source correspondence

- Goldwasser, Micali, Rackoff, _The Knowledge Complexity of Interactive Proof
  Systems_, SIAM J. Comput. 18(1), 1989, DOI 10.1137/0218012.
- Bellare, Goldreich, _On Defining Proofs of Knowledge_, CRYPTO 1992,
  DOI 10.1007/3-540-48071-4_28.
- Schnorr, CRYPTO 1989, DOI 10.1007/0-387-34805-0_22; Pointcheval--Stern,
  J. Cryptology 13 (2000); RFC 8235.
- CFRG _Fiat--Shamir Transformation_ draft -04 (2026-11-02); FIPS 204;
  FIPS 186-5; RFC 9497.
- Kate, Zaverucha, Goldberg, _Constant-Size Commitments to Polynomials and
  Their Applications_, ASIACRYPT 2010,
  DOI 10.1007/978-3-642-17373-8_11.
- Jacobson, Koblitz, Silverman, Stein, Teske, _Analysis of the Xedni Calculus
  Attack_, DCC 20 (2000), DOI 10.1023/A:1008312401197.

Exact retrieved-file hashes and theorem-level source locations are recorded in
`REPORT.md` and `NEXT_FIVE_PROOF_MAP.md`. The repository KB wrapper was
unavailable (exit 127); primary PDFs and committed predecessor modules were
read directly.

== Formal status and remaining bridges

`lake build` completed 8,941 jobs. The whole-project proof gate reported
`PASS 158 thm 0 trusted`; the axiom audit found only `propext`,
`Classical.choice`, and `Quot.sound`, with some endpoints using none. Five
interpretive PCC claims bind the central declarations, but main-branch verdicts
remain unsigned. Exact receipt hashes are in `VERIFICATION.md`.

Outside scope: computational proof-of-knowledge, a concrete Schnorr
refinement, byte-level codec reachability, pairing and KZG security models,
the measure-theoretic tail/expectation bridge, and the adaptive xedni law.

No performance result, curve identity, leaderboard row, or canonical
index-calculus graph changes in this round.
