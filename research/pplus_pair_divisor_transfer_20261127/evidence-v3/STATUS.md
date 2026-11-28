# Per-target transfer packet status

The independently charged G and `[17]G` cells completed under the unchanged
caps. The frozen verdict remains `INDETERMINATE_CAP`.
[pcc:unverified]

For each target separately:

- specializing before transfer reduces the formal 16,752-coefficient H polar
  to 2,025 coefficients;
- the first canonical pair norm has 85,833 coefficients, after 160,473,912
  field additions and 160,660,740 field multiplications in this sparse
  construction; the second norm stops before exceeding 262,144 slots;
- the ordinary quadratic-tower DAG has 209, 3,843 and 44,838 live nodes at
  depths zero, one and two. The depth-one-to-two factor is 11.667447307; depth
  three stops prospectively at an attempted peak of 306,982 nodes.

The two targets have equal state and operation counts but distinct exact
coefficient-map digests. No evaluated state was reused between them. The
verifier independently matched five complete polynomial cells, checked all
nine DAG encodings, performed 45 polynomial/DAG evaluations and 54 direct
predecessor-norm/successor-DAG evaluations, and reported zero errors.
[pcc:unverified]

These cells reject accidental formal-target inflation as the explanation for
the v1 cap. They do not prove an asymptotic lower bound, handle the exceptional
pair charts or implement the first-pair witness/miss query. Consequently they
do not establish `rho<4`, an algorithmic pass or an m=7 integration.

`seed_manifest=null`; `verify_evidence_v3.sh` hash-checks source custody and
reproduces verifier/analyzer output byte-for-byte. `PACKAGE_SHA256SUMS` hashes
all stored bytes, while `SHA256SUMS` retains the uncompressed candidate digest.
