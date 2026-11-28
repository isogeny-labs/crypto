# Prime31 RR9 radical support: prospective Rust comparison
2026-11-17 UTC · tracking PR #4541 · [pcc:unverified]

## Scope and source custody
Continue #4539's existing native original-prime31 Rust RR9 example and exact-source
GitHub Actions, pinned Rust 1.100.0 and native source commit
`1df714645aa683cf975bc76fcacb9842cb4780fd`.
The ordinary curve is E:y²=x³+847514846x+2087444724 over
p=2147483647, group order n=2147446831 (cofactor one).
Reuse the first-31-rational-x and Dickson-depth-six 31-point bases.
Do not replace Rust arithmetic, use a toy weak curve, or infer ECDLP speed
from a local support-predicate gate.

## Proposed exact equivalence
Let M be **monic degree 8** in F_p[X], H monic and squarefree with distinct
rational linear roots (the public 31-point x factor base), p>8. Let
D=gcd(M,M') be monic and S=M/D be monic. Then
   M | H^8     iff     S | H.
Proof goal: because p>8, every positive multiplicity of every irreducible
factor of M is <p, so S is its squarefree radical. H is squarefree; M|H^8
iff all factors of M are linear factors of H. S|H iff the same.
No assumption on the multiplicity pattern of M. Check the input hypotheses:
wrong degree, zero H, nonmonic or noncanonical inputs must be rejected.
For a **squarefree M**, S=M and the extraction constraint is simply M|H
(no degree-248 H^8 polynomial).

## Preregistered controls, before modifying source
- Retain 64 existing distinct-leaf valid F, 64 target-preserving mutated-F
  negatives, 6 high-multiplicity F positives (3,4,8 on both bases), and 6
  corresponding negative controls on the actual prime31 curve.
- In EACH of these 140 controls, compute old `H^8 mod M` and new
  `rad(M) | H` *independently* and assert equality. Positive cases should
  pass both; target-preserving mutations should fail both.
- Add polynomial-only positive supports with multiplicity partitions
  [8], [4,4], [5,2,1], [1,1,1,1,1,1,1,1] and a negative omitted-root
  control for each. Assert radical degrees 1,2,3,8 and original power gate
  agrees. Also reject at least one noncanonical coefficient input and wrong M
  degree. Test squarefree shortcut directly on the degree-8 partition.
- Run the existing exact-source GitHub Actions RR9 workflow and inspect its
  final SHA and tests, not just workflow dispatch. A benchmark of 2^31-1
  **complete unknown** ECDLP recoveries is NOT run by these membership
  tests; do not publish a complete-solver performance improvement.
- Record historical complete frontier from #4532 as context only:
  1024-Q mixed-base generic-oracle IC 2221.474ms, cached generic BSGS with
  2^20 baby points 170.113ms. Those are one-run unisolated producer
  measurements, not repeated in this PR.

## Prior work / novelty boundary
- #4466 signed RR9 Hermite/norm; #4539 direct seven-variable signed chart.
- #4228/#4349 quotient, subresultants and native Rust rootfinding;
  #4373 and crypto-autoresearcher #3301/#3305 negative/censored attempts.
- #4513 vs #4532 establishes why a rho-only win is not the generic frontier.
- Amadori–Pintore–Sala (2018), FFA 51, 168–182,
  https://doi.org/10.1016/j.ffa.2018.01.009;
  McGuire–Mueller (2017), https://eprint.iacr.org/2017/1262.pdf.
  Algebraic squarefree reduction is elementary, **not a novel index-calculus
  algorithm**.
KB MCP / checkout-based wrapper not available in this connector runtime;
fallback is these explicit GitHub evidence paths and primary literature.

## Decision gate and next computational target
Accept this component only if all controls match and exact-source Rust CI
passes. Never count certificate membership as coefficient extraction.
Next derive B4/B8 **public-only signed-R coefficient system** with M|H on
the discriminant-nonzero stratum, explicitly separate repeated-root
branches using S=M/gcd(M,M'), and compare every complete solution fibre to
a test-only exhaustive oracle. Then measure actual full B31 ECDLP and all
costs against equal-precomputation cached BSGS, tuned rho and external
index-calculus implementations.
