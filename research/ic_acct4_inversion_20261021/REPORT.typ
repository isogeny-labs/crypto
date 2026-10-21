#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Running the priced inversion, and one self-check policy]
  #v(2pt)
  `research/ic_acct4_inversion_20261021` · 2026-10-21 · class: accounting
]

= Question

`bitop/v1` prices a field inversion as a binary extended Euclid and a
quadratic-residue test as the same gcd-type loop. Each costs about 2.1 field
multiplications at 6–16 bits.

The hyperelliptic arithmetic instead ran Fermat and Euler exponentiations
through `num-bigint`'s windowed `modpow`. That is about 93 products whatever
the size of $p$, so the work executed was some 40× what was charged.

Two questions follow:
- Does running the priced algorithms leave every committed count unchanged?
- Does it close the instruction gap the coverage diagnostic saw?

= Method

- *The change.* Word-sized binary extended Euclid (`utils::inv_binary_u64`),
  Jacobi symbol (`utils::jacobi_u64`), a word path for `sqrt_mod_p` with the
  same roots, and square-and-multiply for small public powers. Charges per
  call are unchanged; prices and frozen vectors are untouched.
- *Equivalence.* Five rows of `hec_bitop_20261020` re-run with the study's
  seeds (5 IC and 40 rho runs each), compared field by field with the
  committed rows.
- *Diagnostic.* Callgrind with $(I, R) = (1,2), (3,2), (1,6)$ trials. Each
  arm's figure is its marginal instructions over its marginal counted bitops.
- *Analysis.* `examples/ic_acct4_analyse.rs`.

= Results

All five re-run rows (genus 2 at $p = 41, 61, 101$; genus 3 at $p = 23, 31$)
are *byte-identical* to the committed study in per-run totals, phases,
validation subsets, keys and verification.

#figure(image("figures/instr_per_gate.svg", width: 88%),
  caption: [IC/rho instructions per counted gate. "Before" is the hec study's recorded whole-run figure; "after" is this study's marginal figure, a different method, so read the change as direction.])

#table(columns: 5, stroke: 0.4pt, inset: 4pt,
  [*row*], [*IC*], [*rho*], [*IC/rho now*], [*before*],
  [genus 2, $p = 41$], [1.212], [1.304], [0.930], [1.01],
  [genus 3, $p = 23$], [1.611], [1.681], [0.958], [1.16],
)

= Self-check policy

`docs/units/BITOPS.md` now has three rules:
- The domain is checked once, outside counted work.
- A method's check of its own answer is charged per target.
- Operand re-validation inside the arithmetic is counted but excluded from
  the headline.

#figure(image("figures/self_check_policy.svg", width: 88%),
  caption: [IC/rho per solve with and without the checked arithmetic's operand re-validation. Re-validation is a larger share of rho's work, so the ratio rises under the policy.])

For the genus-4, $p = 61$ row, the policy-conforming figure is the study's
existing "excluding validation" ratio, *0.132*, in place of 0.114. It is
still below rho.

= Scope

- Toy hyperelliptic Jacobians, counts only.
- Callgrind on one row per genus.
- No algorithm changed what it computes, so the class is accounting.
