# Common 2+2 coprime-support uniqueness

Schema: `projective-law-common22-uniqueness/v1`. The pinned Lean build proves
that a polynomial of degree below `degree (u1*u2)` is zero if it is divisible
by both coprime supports. Applied to `residual - scale*curve`, this gives the
conditional equality `residual = scale*curve`. The [kernel log](common22-uniqueness-build.log)
records the two theorem names and their foundational axioms.

The [CSV](common22-uniqueness.csv) and [JSON](common22-uniqueness.json) list
each premise and conclusion, while the [graph](common22-uniqueness.svg) makes
the current proof boundary visible. The [PDF](COMMON22_UNIQUENESS.pdf) is a
printable copy for independent review.

The premise that `B != 0` makes the supports coprime, both source-derived
residues, and the degree bound have not been mechanically connected to this
theorem. The principal-divisor and `Pic^0` bridge also remain open. No new
DLP experiment ran and the recorded cost ratios are unchanged.
