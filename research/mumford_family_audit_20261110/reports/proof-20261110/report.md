---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# A growing family of exact reduced-pair cycles

For every t≥0, Lean checks f=Xⁿ+1 over F₃, g=12t+2 and n=24t+5.
The actual polynomial is monic, separable and has degree 2g+1. The
complete finite universe of valid reduced pairs contains n distinct
smooth pairs: identity and (Xᵈ,±1), 1≤d≤g. Normalized addition by
(X,1) cycles through them with exact period n. [pcc:unverified]

The valid pair (X²+X+2,X) is nonsmooth for every t and outside that
cycle. Thus the cycle smoothness proportion is 1 and the ambient
proportion is strictly smaller. Genus and period are unbounded.
[pcc:unverified]

![Actual polynomial objects, exact arithmetic and counting. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# What is proved, and what remains

The ambient set consists of all monic u with deg(u)≤g, all v with
deg(v)<deg(u), and u dividing f−v². Its finiteness is proved by an
injective bounded coefficient encoding. The n cycle pairs are proved
distinct. The quadratic X²+X+2 has no F₃ root. Explicit quotient
identities, its period dividing X⁸−1 and n=8(3t)+5 prove the uniform
nonsplit pair valid. No finite list is declared complete by assumption.
[pcc:unverified]

Every addition step has full monic gcd/Bézout, composition, bounded
remainder and zero-or-one reduction certificates. These cover identity,
positive increment, the degree-g crossing, negative decrement and
cancellation. The actual step is defined from the unique normalized
output; agreement with an indexed successor is then proved. Injective
positions modulo n prove return after j steps iff n divides j, including
the absence of every shorter positive period. [pcc:unverified]

| Module | Main obligation |
|:--|:--|
| ReducedPairs / CurveFamily | Actual finite universe, monicity/separability/degree, valid split and nonsplit pairs |
| OrbitFamily | n distinct smooth pairs, fraction 1 versus ambient fraction <1 |
| NormalizedCantor / FamilyTransitions | Unique normalized output and all five polynomial run certificates |
| Successor / Cycle | Independently defined arithmetic step; exact period n |
| FamilyCertificate | Unbounded genus/period, combined certificate and direct universal cycle-equality negation |

The whole project audit passes 91 declarations, zero failures and zero
trusted-native exceptions. Lean 4.35.0 / Mathlib 56a00585; only propext,
Classical.choice and Quot.sound. No holes, custom axioms, native_decide,
linter waivers or added elaboration settings. All 39 attempt/source files
and five gate files have byte-equal archive roundtrip checks. Source
commit `9e23de04` precedes `PCC-20261110-156a3c`; full local verification
passes at 13:09:19Z. Signed standing remains unverified; see verification.json. [pcc:unverified]

Stoll 2014, Lemmas 4.16–4.17, Theorem 4.18 and Example 4.19, printed
pages 23–26, were reread. They motivate the actual reduced-pair conditions
and addition equations. The geometric Picard/Riemann–Roch correspondence
and a full all-pair ambient group law remain separate Lean obligations.
This cycle theorem is not promoted to an unconditional geometric
Jacobian subgroup family. [pcc:unverified]

Jędrzejak 2016 (DOI 10.4064/aa8141-3-2016), relevant introduction,
theorem/model and good-reduction sections and the odd-degree torsion proof
on printed page 111 were read, not the full paper. The torsion family is
known; no novelty claim. Cantor's 1987 original remains unread after AMS
HTTP403. Exact source URLs, hashes and read scopes are in source-review.json.
No claim against Gaudry's fixed-genus q→∞ estimate or typical/large-prime
subgroup asymptotics. Periods n are not all prime. [pcc:unverified]

The existing IDC1hb7102c15384f2648 is copied byte-identically for its
base-quintic scalar identity only. New characteristic-3 and variable-genus
arithmetic rests on Lean. No fresh sampling, native experiment, timing or
paid compute. KB/Conductor unavailable; primary/repository/Mathlib fallback.
See README.md, sources.json and verification.json. [pcc:unverified]
