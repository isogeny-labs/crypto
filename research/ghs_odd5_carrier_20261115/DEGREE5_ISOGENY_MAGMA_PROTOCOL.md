# Independent Magma quotient check for the thirteen alternate targets

Frozen after two source-bound Sage screens agreed on all exact `(A,B)` pairs.
Generate `degree5_isogeny_survivors.magma` with the committed deterministic
`make_degree5_isogeny_magma.py`, using only the committed direct-linear replay
JSON receipts. Commit the resulting Magma source before calculator submission.
The generated source SHA-256 is
`a7085464e17f89309fca5200c15862d4ec0f95dba1161856d6b0a37a422ed97c`;
the generator SHA-256 is
`3b048dafcfe498edc5822df83f31ebae398d9c47b6b1189b51eaa5347948fac8`.
For each of the thirteen pairs, Magma must check degree, coprimality, exact
divisibility by f, and the nonsquare factorization of the degree-eight quotient
in the full elliptic identity. Retain HTTP status, response success, exit code,
truncation flag if supplied, input SHA-256 and complete stdout. A successful
check corroborates the Sage full-equation obstruction but does not independently
replay the finite branch search, which is already checked by the two Sage
constructions. No result about higher-degree maps follows.
