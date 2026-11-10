---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Every input degree in the canonical Semaev family

Lean proves sharp individual degree $2^{m-2}$ for every formal input,
every arity $m\ge3$, and every field in the actual canonical recursive
resultant family. Degrees are read through explicitly constructed
polynomial-ring variable permutations. The combined theorem checks actual
selected-coordinate images and formal coverage of every input position.
[pcc:unverified]

No abstract degree-growing sequence is assumed. The proof fixes curve
scalars, transports actual canonical polynomials, proves their actual
resultant recurrence, and applies the checked sharp surviving-variable
step with its nonzero leading coefficient. [pcc:unverified]

![Actual proof families and their formal finite-position coverage. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# Coverage, checked bases and exact scope

Stage $s$ has arity $m=s+3$ and positions $0,\ldots,s+2$.
The Lean position lemma covers first, second, last, or an introduced input
with $i=\mathrm{start}+2$ and $s=\mathrm{start}+n+1$.
[pcc:unverified]

| Family | Sharp formal degree |
|:--|:--|
| Arity 3, first/second via actual cycles | 2 |
| First/second at stage $n+1$ | $2^{n+2}$ |
| Introduced, stage $\mathrm{start}+n+1$ | $2^{\mathrm{start}+n+2}$ |
| Last at stage $s$ | $2^{s+1}$ |

All equal $2^{m-2}$. The initial S4 polynomials are actually relabeled to
previously checked symbolicAt 0/1. Their degree 4 is proved before future
transport. No S4 cross-permutation invariance is assumed. The arity 3 base
uses the actual symmetric S3 formula and cyclic ring permutation.
[pcc:unverified]

The main binding is `all_input_degree_audit_certificate` in
RecursiveSemaevAllDegreesAudit. Four new modules prove transport, seeded
transport, actual initial degrees, and complete input-family coverage.
The proof map is `../../ALL_INPUT_DEGREES.md`; exact statement and source
hashes are in `../../recursive-all-input-sources.json`. Whole census gate423passes, zero failures/zero trusted-native exceptions.
Local source commit `fd9ee4c6` precedes packet `PCC-20261110-007c40`. Actual
local verdict and unsigned standing are recorded separately. [pcc:unverified]

Lean 4.35.0 / Mathlib `56a00585` are pinned. Final modules compile without
warnings and use only propext, Classical.choice and Quot.sound. All 16
attempts retain exact sources, raw diagnostics and exit status, including
the initial expansion timeout after 180 seconds. Lossless archives have unpack/hash
equality checks. No hole, custom axiom, native decision exception or
linter waiver. [pcc:unverified]

Primary read scopes remain Kosters–Yeo (2015), §2/Proposition 2.1and related
Semaev notes. Mathlib swap/mapEquiv/resultant sources and old S4 degrees
were reviewed. Original Semaev (2004) remains unread after 403; no novelty or
source-status promotion. No new native experiment, timing or paid compute.
[pcc:unverified]

Full permutation invariance, higher geometric zero-sum recursion and
irreducibility remain open. Degree can fall after field evaluation: the
older actual smooth S4 specialization over F5 is constant of degree 0. This proof
keeps all inputs formal and does not change that negative control.
The fourteen-item audit remains partially complete. [pcc:unverified]
