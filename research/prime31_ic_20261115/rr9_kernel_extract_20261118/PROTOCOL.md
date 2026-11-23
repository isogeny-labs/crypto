# Protocol: public-only RR9 D9 kernel extraction

Status: frozen before candidate-source edits or measured runs. Experiment id
`EXP-PRIME31-RR9-D9-KERNEL-001`. This is a B8 algebraic relation-extraction
test. It is not a complete ECDLP experiment and cannot support a rho or
cryptographic-scale claim `[pcc:unverified]`.

## Question

The existing public-residual RR9 degree-nine Macaulay operator distinguishes
two registered negative B8 residuals by exact Bezout unit certificates. For
the registered positive residual, can scalar Wiedemann recover a right-kernel
vector from the same public matrix, and is that vector the rank-one monomial
evaluation vector of a valid RR9 coefficient solution?

This replaces the proposed sparse F4/normal-form positive step with the
smaller test `M v = 0`. It does not enumerate signs or factor-base supports.

## Frozen mathematical construction

- Field and curve: `p = 2147483647`,
  `E: y^2 = x^3 + 847514846 x + 2087444724`, prime group order
  `n = 2147446831`.
- Factor-base abscissas: `(0,1,2,3,5,6,8,9)`, with public factor polynomial
  `H_B` built from those eight values.
- Positive public residual: `(1911579686,548820096)`.
- Negative public residuals: `(625037715,2100627251)` and
  `(1923055775,307731458)`.
- RR9 chart variables:
  `(a0,a1,a2,a3,v0,v1,v2)`, with `V` monic of degree three, and the eight
  public quadrics already defined in
  `prime31_rr9_public_macaulay.rs`.
- Degree-nine affine Macaulay matrix `M`: 27,456 rows, 11,440 columns and
  240,240 stored nonzeros. Columns are all monomials of total degree at most
  nine, in the existing deterministic order.
- For each frozen seed, form only `A = M^T D M`, with seeded nonzero diagonal
  `D`. Generate exactly `2N = 22,880` scalar Krylov moments from seeded
  nonzero `u,v`, then run the existing Berlekamp--Massey implementation.
- If the returned monic connection polynomial is
  `P(X) = X^L + c1 X^(L-1) + ... + cL` with `cL = 0`, evaluate
  `W = (P(A)/A) v` by Horner. Accept it only after direct sparse replay
  confirms `M W = 0` and `W != 0`. No BM flag alone is an extraction.
- If the constant monomial coordinate of `W` is nonzero, normalize it to one.
  Read the seven degree-one coordinates as the candidate variables. Require
  every degree-at-most-nine coordinate to equal the corresponding monomial in
  those seven variables. Then require the eight RR9 quadrics, direct
  reconstruction of `H_B`, and a separately invoked original-curve signed
  group-sum verifier all to pass.

The division by `A` is only a notation for dropping one trailing factor `X`
from `P`; no inverse of singular `A` is computed. Failure of a scalar
projection to yield a vector annihilated by `M` is retained as an
inconclusive seed, not repaired by consulting the known witness.

## Frozen arms and seeds

Run all arms; do not stop after the first favorable seed.

| arm | residual | seeds | required role |
|---|---|---|---|
| positive | registered positive above | `0x0d991001`, `0x0d991002`, `0x0d991003` | candidate extraction |
| negative-0 | generator | `0x0d990001` | full-rank/no-fabrication control |
| negative-1 | second registered negative | `0x0d990002` | full-rank/no-fabrication control |

The candidate receives only `(curve, H_B, public residual, seed)`. The signed
B8 fixture enumerator and the known positive evaluation vector are test
oracles and are not callable from the candidate path.

## Predictions and falsifiers

- **P1, sparse correctness:** the D3 sparse/dense operator control, 2x2
  singular/full-rank controls, and the existing known-positive evaluation
  check pass unchanged. Any failure invalidates the run.
- **P2, positive scalar kernel:** at least one of the three positive seeds has
  terminal BM coefficient zero and yields a nonzero `W` with `M W = 0`.
  Zero successful seeds falsifies this scalar-Wiedemann extractor under the
  frozen projections; it does not establish that the public ideal has no solution.
- **P3, rank-one moment vector:** at least one directly checked positive
  kernel vector has nonzero constant coordinate and passes every monomial
  consistency check. If directly checked kernels exist but none is rank one, record
  the observed inconsistency and move to block-Wiedemann/flat-extension
  extraction; do not choose coordinates from the known witness.
- **P4, exact public relation:** a rank-one candidate satisfies all public
  RR9 equations, reconstructs `H_B`, selects one rational sign at every B8
  abscissa, and those eight original-curve points sum to the public residual.
  Any failed independent check rejects the extracted relation.
- **P5, negative controls:** both negative arms reproduce linear complexity
  11,440 with nonzero terminal coefficient and produce no kernel candidate.
  A directly checked negative kernel or fabricated RR9 solution invalidates the
  candidate. A different inconclusive preconditioner is retained but does not
  erase the existing exact unit certificates.

## Counted work and limits

Record, per arm: rows, columns, nonzeros, Krylov moments, Gram applications,
field multiplications in the moment loop, BM complexity and terminal
coefficient, kernel-polynomial Gram applications, direct `M W` work,
monomial checks, output-vector digest, wall time, peak RSS, and every verifier
boolean. Wall time is diagnostic; exact field-operation counts are primary.

The intended IsoLab request is x86 CPU Spot, one pod, 4 vCPU, 8 GiB RAM,
20-minute deadline, one infrastructure retry, and a total ceiling of USD 1.
No automatic quota/cap increase and no alternate provider after a budget
stop. Development tests may use an unmeasured local build, but the recorded
run uses the committed source and launch request.

## Decision rule

- P1--P5 pass: classify this as a successful B8 public-only positive
  extraction component. Next, measure corank/multiplicity on the registered
  B31 22-positive/2-negative panel before integrating any relation into the
  31-column/1,024-target pipeline.
- P2 fails: retire scalar-Wiedemann kernel extraction for this fixed operator
  and test block Wiedemann or sparse F4; preserve the positive fixture and
  exact negatives.
- P2 passes and P3 fails: the kernel is not directly a point evaluation;
  construct multiplication matrices from a block moment basis. Do not label
  the scalar vector a relation.
- P3 passes and P4 fails: implementation/candidate invalid; fix only under a
  dated amendment and rerun all five arms.
- Any outcome remains a component result. Rho parity still requires B31
  relation rank, cold one-target recovery, complete phase accounting, matched
  rho, growing sizes, and independent replay.

## Frozen source boundary

- Base branch commit before this protocol: `61264f2b2561209585d61b837bd3879d59ec8244`.
- Pinned original native arithmetic: `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- Existing public RR9 source SHA-256:
  `2717152b3746f6881a7ac0dc016a7b6f900ebbaf21447ba6ddbed0879f0d1529`.
- Existing sparse D9 core SHA-256:
  `083a4820408ab3a9608da55981fc0b52a7ca072ab7d669f6e6e403a83f206233`.

Candidate source, verifier source, launch request, binary digest, host
manifest and raw-output digests will be appended without changing the inputs,
seeds or gates above.
