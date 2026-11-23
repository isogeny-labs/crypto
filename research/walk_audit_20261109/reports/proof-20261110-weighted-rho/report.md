---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Exact rho moments with unequal label probabilities

For every real $0\le a\le1$, Lean constructs a genuine PMF on all eight
actual hash functions from Z/3 to two labels. Each vertex independently
chooses the distinct nonzero increment $+1$ with probability $a$ and $+2$
with probability $1-a$. Actual in-degree at vertex 0 has mean 1 and variance
$1-(a^2+(1-a)^2)$. For probabilities $3/4$ and $1/4$, the variance is exactly
$3/8$. [pcc:unverified]

| Exact control | Mean | Variance |
|:--|--:|--:|
| Uniform labels, 1/2 and 1/2 | 1 | 1/2 |
| Unequal labels, 3/4 and 1/4 | 1 | 3/8 |

![Recorded exact kernel controls for the complete eight-hash ensemble. No sampled data, timing or uncertainty interval.](moments.pdf){width=95%}

The expanded whole walk project gate passes **43 declarations**, zero
failures and zero trusted-native exceptions. Only propext,
Classical.choice and Quot.sound are accepted. Lean 4.35.0 and Mathlib
revision prefix `56a00585` are pinned. [pcc:unverified]

\newpage

# Construction and scope

The proof enumerates every actual hash through a proved bijection with
three-bit tables. Kernel finite decision checks actual graph edges and
in-degree counts. The product of independent vertex-label weights is
proved nonnegative and normalized. PMF.ofFintype constructs the actual
law, and ordinary real polynomial algebra establishes the moments for
arbitrary $a$. The result is not fitted to observations. [pcc:unverified]

Source commit `a88c6fa0` precedes theorem packet `PCC-20261110-785828`.
The combined binding is `weighted_variance_audit_certificate` in
WalkWeightedAudit. Source/declaration hashes and all gate receipts are in
`../../weighted-sources.json`; the local packet verdict and unsigned
standing are recorded separately. Failed proof and registration attempts
remain retained. [pcc:unverified]

This verifies the $1-\sum p_i^2$ variance component in this exact ensemble.
It does not derive the $1/\sqrt{1-\sum p_i^2}$ collision-time penalty,
a limiting constant, general support-size formulas, Hildebrand's averaged
mixing theorem or practical hash-family behavior. No new native experiment
or paid compute was run. Prior frozen source, protocol and data remain
unchanged. [pcc:unverified]

Primary Galbraith chapter 14, version 2, section 14.2.5, pages 296–297,
was re-read, including its ideal random-function assumption and separate
conjectural collision estimates. The full PDF hash is in the source plan.
A true weighted moment is not presented as a disproof of those heuristics.
Scientific PCC standing remains unsigned until an independent signed
verdict. [pcc:unverified]
