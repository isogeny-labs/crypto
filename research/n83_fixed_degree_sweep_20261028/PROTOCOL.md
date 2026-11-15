# Fixed-base n83 regularity sweep

User request: rerun degree-of-regularity for Koblitz, general binary and prime,
n=83, m=2..9, using the completed fixed ell27 factor bases. All24 cells are
retained. Every cell has one planted target and one independently lifted natural
target, and each gets top-form and affine computations:96 attempts total.

This is a separate run of the existing S3-chain presentation and Singular driver.
The mathematical algorithms, target seed20261027, complete-result checks and
per-process limits are retained. ell is now27 for every m. The generator checks
the three exact fixed-base manifest hashes and looks up every planted point's
full (x,y) record in a hash-verified stored point shard. The base is never resized.
All inputs carry manifest identity and point-membership receipts.

The polynomial encoding represents the whole candidate x-domain with27 Boolean
selectors per base coordinate. It does not expand the67 million liftable-x lookup
set into a polynomial. This is the existing algebraic candidate system for that
fixed base: any recovered relation still requires rational point lifts and sign
consistency. Planted targets are checked to use actual saved base points.

Binary field is GF(2^83), modulus z^83+z^45+z^2+z+1. Koblitz a0,b1; general
binary a1,b=2^82+0x51b in y^2+xy=x^3+ax^2+b. Prime p4835703278761981908090881,
a2,b3 in y^2=x^3+ax+b. Full affine curves, no subgroup filtering.

Measure top-form regularity only by completing the homogeneous standard basis,
checking dimension zero and reading one plus the final nonzero Hilbert degree.
Separately retain maximum final affine basis degree. Neither is substituted for
maximum intermediate solving degree or first fall. Every incomplete value is
null. Positive-dimensional top quotients have no finite degree under this
specific definition. The prior ALGEBRA.md argument predicts positive-dimensional
prime top ideals at m>=3 independently of ell, but that is not a completed solver
measurement and does not suppress those requested runs.

Four single-threaded workers, each with3GiB virtual memory. Each of96 solver
processes gets40 seconds and5 seconds kill grace, including input expansion and
witness checks. Whole container deadline1800 seconds,4 CPU limit and16GiB memory.
No automatic retries or adaptively enlarged budgets. The existing Spot workspace
is used because the EKS worker's shared-bucket authorization failure is unresolved
(recorded in ../n83_fixed_factor_base_20261028/validation/r3-storage-denied.log).
Live EKS overview found no active n83 job. No permissions or quotas are modified.
The existing digest-pinned Singular container is used with the new committed
native binary mounted read-only; distinguish runtime image from mounted source.

Same-driver eight positive/inconsistent control computations must pass. Native
field/S3 tests and a positive/negative stored-record lookup test must pass before
launch. Inputs and completed attempts are synchronized by the host every30s to
s3://setec-astronomy-artifacts; interrupted work is retained without a degree.
No process-memory resume is claimed. Timestamps are diagnostic, wall_valid=false.

Literature checked before implementation: prior PROTOCOL.md, ALGEBRA.md and the
native regularity runner in ../n83_ic_degree_sweep_20261027; Kosters and Yeo,
https://arxiv.org/html/1503.08001 (S3 and first-fall/regularity distinction).
The current Singular PDF/manual fetch failed; installed Singular4.3.1 behavior is
checked through the retained exact control fixtures. No new unseen theorem used.

Deliver all96 outcomes, exact input/base/source/binary identities, controls, logs,
raw archive and hashes, unified export, degree grids and inspected PDF. A bounded
sweep finishing is not the same as determining all requested regularity values.
