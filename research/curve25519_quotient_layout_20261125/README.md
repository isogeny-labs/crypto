# Quotient-first grevlex layout: completed negative qualification

Study `EXP-C25519-QLAYOUT-001` tested whether moving the four selective
quotient variables from the trailing grevlex positions to a leading block
could keep the degree-4 benefit while consuming auxiliary state earlier.

The primary F arm used `1.2934390725` times D's counted F4 field operations;
the descriptive paired 95% interval is
`[1.2934369447, 1.2934413625]`, versus a frozen upper limit below `0.75`.
F improved on the canonical trailing T layout by only 2.69%, far short of the
required F/T upper endpoint below `0.60`, and its maximum per-pair F/D peak-RSS
ratio was `2.3765566672`. F therefore stops before scaling and `m=4`.

All 60 qualification rows and 12 canonical-B reproduction rows completed.
Every planted row evaluated exactly and re-added to the target. Inverse
permutation proves T/F/E/I are identical ideals with renamed variables; their
input counts and verdicts match. T reproduced the predecessor B instrument on
all 12 fresh-seed targets.

This `[pcc:unverified]` study is an ordinary-grevlex variable-permutation
diagnostic, not a true block-order solver. It does not price the complete ECDLP
pipeline and makes no subexponential claim. No JavaScript is used.
