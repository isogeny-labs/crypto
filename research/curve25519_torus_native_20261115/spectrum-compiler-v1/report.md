# Native compact spectra for Curve25519

The public-input Rust compiler and its exact SMT export are implemented. The
original q213156431/k20633 case has 15231 variables,15239 quadratic equations and
188 semantic inputs: five normalized section coefficients, three inverse
guards and 180 Boolean label/slack bits. Source and fixed native evidence are
bound by unsigned statement PCC-20261123-989b50. [pcc:PCC-20261123-989b50]

All 12 existing supplied sections extend through the compact equations and pass
the unchanged numeric support and original indexed/group replay. The full
original case rejects211 one-wire corruptions; independent determinant checks
and recomputed guard/bit/range refusals are retained. This qualifies a fixed
correctness interface; it discovers no unknown section coefficient.
[pcc:PCC-20261123-989b50]

The compact equations use the normalized sextic quotient, guarded Chebyshev
operator and characteristic-polynomial identity with six unknown bounded coset
labels. Repeated coset labels remain allowed. Squarefreeness, map/torsion and
range guards remain explicit. Quotient rank six does not bound the dimension
of a coefficient solution ideal or the degree of a Gröbner computation.
[pcc:unverified]

The native cvc5 GPL/CoCoA release 1.4.2 passed two scalar model/replay checks and
one 15-bit contradiction. Its established BitSplit backend is a comparator for
the bounded-label bitsums. CAV 2024's verification gains do not give a
decomposition exponent; its worst case still computes a full Gröbner basis.
No section system was submitted to this backend. [pcc:PCC-20261123-989b50]

The same-target original point replay already has 1059 values, so the compact
section representation must be compared against it in an actual unknown-root
solver. The next operation is a strict all-semantic-input model parser, full
native gate checking and unchanged original extraction on matching public
targets. Complete costs/yield/family/rank/indexing/recovery remain open; a cap
or unverified UNSAT is not a route disproof. [pcc:unverified]

See README.md for the counts table; PROTOCOL.md for primary source review;
REPRODUCE.md/FROZEN.md for versioned native scope; NEXT.md for the solver gate;
PUBLISHING.md for the actual S3 access failure and pending receipt. The two-page
PDF and editable diagram were inspected. No paid job or search campaign runs.
