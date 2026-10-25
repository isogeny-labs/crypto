# Common 2+2 coprime-support uniqueness

Schema: `projective-law-common22-uniqueness/v1`. The pinned Lean build proves
an explicit Bézout identity for the two source-style quadratics, then proves
that nonzero `B` makes them coprime. Given interpolation and valid-input curve
congruences, it proves both residual residues. It also proves that a polynomial
below `degree (u1*u2)` is zero if divisible by both coprime supports. Applied
to the cleared residual difference, this gives the conditional curve equation.
A separate Lean lemma proves the strict degree bound from degree-at-most-five,
matching head coefficients, and quadratic-support hypotheses.
The [coprime](common22-coprime-build.log), [residue](common22-residues-build.log),
[degree](common22-degree-build.log), and
[uniqueness](common22-uniqueness-build.log) kernel logs record all nine
theorem names and their foundational axioms.

The [CSV](common22-uniqueness.csv) and [JSON](common22-uniqueness.json) list
each premise and conclusion, while the [graph](common22-uniqueness.svg) makes
the current proof boundary visible. The [PDF](COMMON22_UNIQUENESS.pdf) is a
printable copy for independent review. It is rendered from
`COMMON22_UNIQUENESS.tex` with `pdflatex` and optimized with Ghostscript
`pdfwrite`; the checksum manifest binds the final bytes.

The source interpolation premises and source polynomial head have not been
mechanically connected to the runtime formulas for all valid inputs. The
principal-divisor and `Pic^0` bridge remain open. No new DLP experiment ran;
the recorded cost ratios are unchanged.
