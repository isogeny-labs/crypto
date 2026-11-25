# Pair-trace 2+1 split: completed negative screen

Study `EXP-C25519-PAIRTRACE-001` tested an exact swap quotient for two of three
factor-base roots, combined with a two-S3 split and producer-local auxiliary
variables that make every input equation quadratic.

The primary P arm used `2371.7222843` times D's counted F4 field operations at
the first frozen `d=7` screen; the descriptive paired 95% interval is
`[2371.7202857, 2371.7238337]`, versus a frozen upper limit below `1.00`.
P used `2.0732899` times the raw quadratized split R and reached a maximum
per-pair P/D peak-RSS ratio of `14.4512397`. Although P lowered maximum
productive degree from 5 to 4 on all four random targets, it therefore stops
before `d=9`, `d=19`, scaling, and `m=4`.

All 24 qualification rows completed without censoring. Every planted row
evaluated exactly and group-verified. Native self-tests exhaustively checked
the pair-trace bijection, trace roots, circuit identities, and small truth
panels; D reproduced the predecessor binary exactly on all eight fresh-seed
targets. Amendment 1 was filed before solver data to retain a cross product
required by the already-frozen quadratic-input rule.

This `[pcc:unverified]` result is a relation-system stage diagnostic on a small
prime-field Curve25519-family instance. It does not price a complete ECDLP
pipeline and makes no exponent or subexponential claim. No JavaScript is used.
