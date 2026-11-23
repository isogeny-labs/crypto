# Coupled S3 partial-norm structure qualification

Date: 2026-11-22 Pacific. Parent: crypto PR #4921. Status: prospective native correctness checks; no timing experiment or solver-cost claim. [pcc:unverified]

## Question and representation

For E: Y^2=X^3+A X+B over an odd prime field and d|(p-1), form

H(A,B;x,t) = Res_z(z^d-1, S3(x,z,t)) mod (x^d-1).

Write H=sum c_ij x^i t^j, 0<=i<d. Test whether the first eliminated S3 link admits a small separated representation sum_(k=1)^r a_k(x)b_k(t). The exact coefficient-matrix rank is the minimal such r. This representation-specific question does not constrain arbitrary arithmetic circuits, alternative selectors, or target-specific quotient compression.

## Prediction, controls and acceptance

A written specialization argument predicts generic full row rank d when p>2d. At A=B=0, the first d t-coefficient columns are (-1)^j binom(2d,j)x^(-j); the determinant is (-1)^(d-1) product_(j=0)^(d-1) binom(2d,j). The singular specialization is solely an algebraic nonvanishing certificate, never a curve-security or performance fixture. Verify nonsingularity separately on the ordinary project fixture.

The native development qualifier will reuse the exact published `src/cryptanalysis/semaev_prime_poly.rs` field, S3, polynomial and Sylvester-resultant code at base e68f631c492c15bb54d79b96714e56ff72cf9f9d. No uncommitted sibling selector implementation is copied, edited or published.

Required checks: cyclic-x partial product agrees with direct scalar S3 products; bounded Sylvester checks; the singular coefficient identity and determinant; degree_t<=2d-1; leading coefficient -4d^2*(x+A*x^(d-1)+B*x^(d-2)) for d>=2; exact row rank and first-minor determinant on the fixed project curve. Preserve any failed check. No timing, arithmetic-operation, ECDLP-cost, independent-rank or performance comparison is inferred.

Fixtures: F97 selector d=1,2,3,4,6,8,12,16; Prime31 p=2147483647 with d=1,2,3,6,7,9,14,18,21,31,42,63. Nonsingular coefficients A=847514846, B=2087444724 are the existing project's ordinary Prime31 fixture. Reuse does not re-certify its full group order. No secret scalar or DLP-dependent input is used. Larger d=1302 is covered only by the written generic minor bound unless explicitly run later.

Local unit tests are development work under isolab-eks; no paid job or timing benchmark is authorized by this file. Bound the qualifier to one process, 2 GiB working memory and 60 seconds per invocation. If those bounds fail, retain that failure and do not increase them silently. Capture compiler version, Cargo.lock, exact source SHA-256, stdout, stderr and exit status before interpreting checks. Keep full-repository cargo validation distinct from this narrow exact-module qualifier.

## Prior work and coordination

Sources checked (KB): KB unavailable (no exposed MCP; scripts/research-kb status exit 127). Read FASS #4921 protocol and live selector report (read-only, uncommitted sibling); #4920 chain/point-lifting implementation; #4922 shared-Macaulay continuation; #4928 NEXT_IMPLICIT_INDEX.md at e1d1519aa7f2003658df56b83af75ab4eb93e824; Petit-Kosters-Messeng prime-ECDLP paper; Bernstein-De Feo-Leroux-Smith, arXiv:2003.10118. Existing dense-norm and dictionary cost obstructions are not being rediscovered as new results.

Conductor is unavailable; use this isolated checkout and only coupled/ paths plus a distinct parent ACTIVE entry and dated log. The tracking commit is published at 929d38b3937e197ddf323e2b7b137256cb388bc0. Terminal git main fetch failed with unresolved deltas and push lacks terminal credentials; authenticated GitHub connector remains available. No other worktree is modified.

All new mathematical statements are written derivations pending formal proof and signed independent validation. Existing PRs and frozen evidence remain unchanged. [pcc:unverified]
