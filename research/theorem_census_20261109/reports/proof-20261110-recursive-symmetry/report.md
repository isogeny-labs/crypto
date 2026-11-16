---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Symmetry and degree of the newest Semaev input pair

Lean proves that every actual canonical recursive resultant extension is
unchanged when its two newest independent inputs are exchanged. Both have
sharp formal degree $2^{m-2}$ at every recursive arity $m\ge4$. This extends
the earlier all-arity last-input theorem to the penultimate input.
[pcc:unverified]

| Arity | Last input degree | Penultimate input degree |
|:--|--:|--:|
| 4 | 4 | 4 |
| 5 | 8 | 8 |
| $n+4$, every $n\ge0$ | $2^{n+2}$ | $2^{n+2}$ |

These are exact symbolic consequences, not measurements on selected finite
fields. The complete tracked census gate passes **320 declarations**, zero
failures and zero trusted-native exceptions. Only propext,
Classical.choice and Quot.sound are accepted. No custom axiom, hole or
native decision exception. Lean 4.35.0 and Mathlib revision prefix
`56a00585` are pinned. [pcc:unverified]

![Actual polynomial proof dependencies. The coefficient-ring automorphism swaps the two newest inputs and preserves the declared resultant sizes.](proof.pdf){width=98%}

\newpage

# Actual automorphism and actual recursive determinant

With $z=C(X)$ and $t=X$, Mathlib's bivariate algebra automorphism sends
$z$ to $t$, $t$ to $z$, and fixes old scalar coefficients. The proof checks
that the lifted old polynomial is fixed. It also checks all three
coefficients of the lifted S3 quadratic: the squared difference,
the symmetric linear coefficient and the symmetric constant term.
[pcc:unverified]

Resultant functoriality moves that automorphism through the actual
fixed-size Sylvester determinant. This gives an identity of formal
polynomials at every recursive extension. The degree read after swapping
$z,t$ is the original penultimate degree; invariance identifies it with
the last-input degree, already proved sharp at every arity.
No Sylvester size is reset after a degree-drop specialization.
[pcc:unverified]

The main binding is `newest_pair_audit_certificate` in
RecursiveSemaevSymmetryAudit. The actual exchange, lifted polynomials,
resultant and canonical-family lemmas are mapped in
`../../RECURSIVE_SYMMETRY.md`. Exact statement/source hashes and the whole
gate archive are in `../../recursive-symmetry-sources.json` and evidence/.
Failed module-mode/coercion attempts remain preserved. [pcc:unverified]

Source commit `0c3de9b1` precedes theorem packet `PCC-20261110-104604`.
Its full local check passes at 01:54:16 UTC on producer host
`762f5fc120c9`; signed standing remains unverified. The complete five-file
gate, including inherited warning-bearing diagnostics, is preserved in a
lossless archive with unpack/hash equality checks. [pcc:unverified]

This result proves symmetry and degree of the two newest inputs. Full
permutation symmetry, degrees of every older input beyond arity 4,
higher geometric zero-sum recursion and irreducibility remain separate.
It does not assert degree preservation after evaluating independent inputs
at field elements. No new native experiment, timing or paid compute.
[pcc:unverified]

Prior primary read scopes in sources.json remain unchanged, including
Kosters–Yeo 2015 section 2/proposition 2.1 and the related Semaev notes.
Original Semaev 2004 remains unread after 403. No source-status promotion
or novelty claim is made. [pcc:unverified]
