---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# An older input through the actual resultant

Lean proves that a surviving old variable's degree $d$ becomes exactly $2d$
in the actual fixed-size Semaev resultant. The coefficient at $2d$ is the
same resultant on the previous old leading coefficient. This supplies a
canonical third-newest-input degree theorem at every arity at least five.
[pcc:unverified]

Let $f\in S[\mathrm{old}][E]$ be nonzero, with elimination degree at most
the declared size $D$, and let the four curve coefficients belong to the
domain $S$. The actual variable permutation reads
$S[\mathrm{old}][Z][T]$ as $S[Z][T][\mathrm{old}]$.
The Sylvester sizes remain $D,2$ throughout. [pcc:unverified]

![Actual determinant proof: the coefficient at the degree bound is another actual resultant, whose nonvanishing was proved in the earlier general step. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# The canonical consequence and its limits

At actual recursive stage $n+2$, arity $m=n+5$, the surviving input was
the preceding polynomial's penultimate input. Its degree $2^{n+2}$ doubles
to $2^{n+3}=2^{m-2}$. The earlier newest-pair proof supplies the other two
newest input degrees. [pcc:unverified]

| Canonical arity | Checked individual input degrees |
|:--|:--|
| 4 | All four equal 4, from the earlier S4 proof |
| 5 | Three newest equal 8 |
| Every $m\ge5$ | Three newest equal $2^{m-2}$ |

The matrix has $D$ quadratic columns of old degree zero and two $f$
columns of old degree at most $d$. The determinant degree is at most
$2d$. Its coefficient at $2d$ is the fixed-size resultant of the old
leading polynomial with the lifted S3 quadratic. That old leading
polynomial is nonzero and still has elimination degree at most $D$.
The earlier exact resultant coefficient theorem proves this coefficient
cannot vanish. No generic leading-term assumption is needed.
[pcc:unverified]

Main binding: `focus_degree_audit_certificate` in
RecursiveSemaevFocusAudit. Generic sharp degree and exact top coefficient
are fully quantified alongside the actual canonical corollary. Proof map:
`../../RECURSIVE_FOCUS.md`. Exact source/statement hashes and gate receipt:
`../../recursive-focus-sources.json`. Whole census gate357passes, zero failures/zero trusted-native exceptions.
Source commit `a55402b8` precedes packet `PCC-20261110-2c396d`. Actual local
verdict and unsigned standing are in the separate verification record. [pcc:unverified]

Lean4.35.0 and Mathlib `56a00585` are pinned. Source15 compiles without
warnings; only propext, Classical.choice and Quot.sound are used.
Fourteen earlier attempts are preserved losslessly, including accepted
intermediate components. Explicit polynomial hom domains/codomains
resolve nested ring inference. No hole, custom axiom, native decision
exception or linter waiver. [pcc:unverified]

Primary read scopes remain Kosters–Yeo2015 section2/Proposition2.1and
related Semaev notes. Mathlib Bivariate.swap, Resultant.Basic and
Matrix.Polynomial were reviewed. Original Semaev2004remains unread after
403; no novelty or source-status promotion. [pcc:unverified]

Every remaining older canonical input still needs transport through the
successive coefficient-ring permutations. Full permutation symmetry,
higher geometric zero-sum recursion and irreducibility remain open.
No degree preservation after finite-field evaluation, new native
experiment, timing or paid compute is claimed. [pcc:unverified]
