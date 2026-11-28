---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# All-pair arithmetic on the growing smooth family

For every t, Lean gives the actual reduced pairs identity and (Xᵈ,±1),
1≤d≤g, a cyclic group law over F₃ for f=Xⁿ+1, g=12t+2, n=24t+5.
Every sum equals an independently defined unique normalized Cantor
output. The actual generator (X,1) has exact order n and generates every
member. The earlier polynomial-defined cycle step equals addition by
that generator. Genus and order grow without bound. [pcc:unverified]

All group members are smooth, fraction 1. The full finite set of valid
reduced pairs has smooth fraction strictly smaller than 1, witnessed by
(X²+X+2,X). This verifies the group law on the family. It leaves the full
ambient group and geometric Picard correspondence open. [pcc:unverified]

![Actual arithmetic precedes and agrees with group transport. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# Certificates, source scope and remaining bridge

Same-sign inputs (Xᵃ,s),(Xᵇ,s), s=±1, have monic gcd 1. Their composition
has degree a+b. If it exceeds g, one normalized reduction gives
(Xⁿ⁻ᵃ⁻ᵇ,−s). Opposite-sign inputs have gcd Xᵐⁱⁿ⁽ᵃ,ᵇ⁾: equal degrees
cancel to identity, and unequal degrees leave the larger sign and degree
|a−b|. Full polynomial gcd/Bézout, composition, bounded remainder and
reduction equations are certified for every ordered pair; identity and
swap cases are included. [pcc:unverified]

The signed residue map is a bijection to ZMod n. Every candidate output
respects residue addition. Output uniqueness holds for explicitly
selected Bézout coefficients, so pairwiseOutput and cantorAdd are defined
from the actual normalized equations before group transport. The
transported sum is then proved equal to cantorAdd. The actual generator
has order n; every member is j times it for some j<n. A coefficient and
actual-point equality proof binds the frozen parent's actualStep to this
same generator addition. [pcc:unverified]

| Module | Exact checked obligation |
|:--|:--|
| AllPairRuns | General same/opposite-sign, cancellation, identity and swap polynomial runs |
| IndexArithmetic | Signed-residue bijection; every candidate respects addition |
| PairwiseOutput | All pairwise runs, unique output and closure before group structure |
| VerifiedFamilyGroup | Sum equals actual normalized arithmetic; generator order and cyclicity |
| GeneratorAgreement | Earlier actualStep equals generator addition; combined unbounded-family certificate |

The whole project audit passes 64 declarations, zero failures and zero
trusted-native exceptions. Lean 4.35.0 / Mathlib 56a00585, standard axioms
only: propext, Classical.choice and Quot.sound. No holes, custom axioms,
native_decide, linter waivers or added elaboration settings. All 22
attempt/source files and five gate files have byte-equal archive checks.
Issued parent formal sources remain unchanged. Exact source commit,
packet, local verdict and signed standing appear in verification.json;
signed standing remains unverified. [pcc:unverified]

The exact primary read receipts are reused: Stoll 2014, Lemmas 4.16–4.17,
Theorem 4.18 and Example 4.19, printed pages 23–26; Jędrzejak 2016,
relevant introduction/theorem, model, good-reduction and printed-page-111
torsion proof. No new full-paper read is claimed. Cantor's original
remains unread after ordinary AMS HTTP403. Source-review.json pins
the prior URLs, hashes and read scopes. No novelty claim. [pcc:unverified]

The full ambient reduced-pair group law and its geometric
Picard/Riemann–Roch identification remain separate formal obligations.
No unproved ambient embedding is assumed. The orders 24t+5 are not all
prime. No refutation of fixed-genus q→∞ or typical/large-prime subgroup
asymptotics is claimed. [pcc:unverified]

The existing IDC1hb7102c15384f2648 is copied byte-identically for its
base-quintic scalar identity only. It does not certify this field,
variable exponent, group law, period, counting or geometry; the new
arithmetic rests on Lean. No new native experiment, sampling, timing or
paid compute. KB/Conductor unavailable; bounded primary/repository/Mathlib
fallback. Proof map and pins: README.md, sources.json, verification.json.
[pcc:unverified]
