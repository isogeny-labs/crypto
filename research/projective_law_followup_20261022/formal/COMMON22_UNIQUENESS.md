# Common 2+2 coprime-support uniqueness

Schema: `projective-law-common22-uniqueness/v1`. The pinned Lean build proves
an explicit Bézout identity for the two source-style quadratics, proves their
degree-two and nonzero properties, then proves that nonzero `B` makes them
coprime. Given interpolation and valid-input curve
congruences, it proves both residual residues. It also proves that a polynomial
below `degree (u1*u2)` is zero if divisible by both coprime supports. Applied
to the cleared residual difference, this gives the conditional curve equation.
A separate Lean lemma proves the strict degree bound from matching head
coefficients and quadratic-support hypotheses. It covers a degree-at-most-six
residual whose sixth coefficient vanishes.
Four additional Lean theorems expand a generic cubic numerator against two
quadratic supports and a quadratic residual support, then read its coefficients
at degrees six, five, and four. Four more substitute the proof-side
source-style coefficients and show `0`, `B²`, `0`; they also equate the
canonical numerator with its interpolation expression. The semantic link
from Rust execution to that proof-side polynomial remains open.
The [coprime](common22-coprime-build.log), [residue](common22-residues-build.log),
[degree](common22-degree-build.log),
[polynomial-head](common22-polynomial-head-build.log),
[source-head](common22-source-head-build.log), and
[uniqueness](common22-uniqueness-build.log) kernel logs record all twenty-three
theorem names and their foundational axioms.

The [CSV](common22-uniqueness.csv) and [JSON](common22-uniqueness.json) list
each premise and conclusion, while the [graph](common22-uniqueness.svg) makes
the current proof boundary visible. The [PDF](COMMON22_UNIQUENESS.pdf) is a
printable copy for independent review. It is rendered from
`COMMON22_UNIQUENESS.tex` with `pdflatex` and optimized with Ghostscript
`pdfwrite`; the checksum manifest binds the final bytes.

The proof-side head is exact, but Rust-to-Lean semantic correspondence has not
been mechanically established for all valid inputs. The
principal-divisor and `Pic^0` bridge remain open. No new DLP experiment ran;
the recorded cost ratios are unchanged.
