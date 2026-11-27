# Direct Dickson/S5 structural gate

## Outcome

The bounded direct four-leaf construction is algebraically exact on the frozen
Prime31 cells: the independently expressed verifier matched every emitted
Dickson multiplication-channel entry, every `S_5` kernel entry, all 45
target/partition charts, every zero multiset and every signed curve witness.
The complete v2 packet therefore reaches the frozen structural pass.
[pcc:unverified]

It does **not** reach the algorithmic pass. The pair-flattened kernels have the
predicted constant ranks, but matrix singularity gives the wrong zero-entry
answer on all six unplanted controls. The compact divisor circuit still indexes
`binom(n+3,4)` four-multisets, and no exact target-dependent zero oracle with
fewer than `n^2` leaf-scale operations or slots was constructed. The frozen
verdict is `STRUCTURE_ONLY_STOP`; no m=7 or production census is authorized.
[pcc:unverified]

## Reproducible evidence

- Active source custody: `SOURCE_MANIFEST_2.json` and
  `SOURCE_SHA256SUMS_2`, qualified source commit
  `18dca898e2b895a31f7651e230498baf583c2eb8`.
- Raw v2 packet: lossless gzip stream `evidence-v2/candidate.txt.gzip`, `verifier.txt`,
  `analysis.txt`, `RUN.txt`, the original `SHA256SUMS`, and the stored-byte
  `PACKAGE_SHA256SUMS`. `verify_evidence_package.sh` checks both the package and
  the original uncompressed candidate digest without an object store.
- Candidate SHA-256:
  `036663ad01fe17fc52a2d784857f8f74939f093a03e46428bfb392852044388f`.
- Independent-verifier result:
  `candidate_match=1 exactness_ok=1 coverage_complete=1 errors=0`.
- Analyzer result:
  `exactness_ok=1 coverage_complete=1 predicate_disagreements=6
  exact_zero_oracle=0 verdict=STRUCTURE_ONLY_STOP`.

The first packet is retained in `evidence/` with
`COVERAGE_INVALID_STOP`. `AMENDMENT_1.md` records the missing v1 emissions and
was committed before the repaired sources or v2 run. It is not cited as the
completed result.

No randomness is used: `seed=none` means there is no hidden or omitted seed
stream. All inputs and witnesses are public. `PROOF_MODEL.md` explains why the
evidence is exhaustive deterministic replay rather than a zero-knowledge proof,
a trusted-execution receipt or a signed PCC verdict. [pcc:unverified]

## Exact bounded results

Let `N=n^2` be the ordered leaf-pair dimension. The three targets at every size
are the deterministic planted target, `G`, and `[17]G`.

| `n` | unordered pairs | four-multisets | `N^2` kernel entries | `rank K` (all targets) | `rank Delta_s`, `rank Delta_p` | zeros: planted / `G` / `[17]G` |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 10 | 35 | 256 | 10 | 10, 10 | 1 / 0 / 0 |
| 8 | 36 | 330 | 4,096 | 15 | 30, 30 | 1 / 0 / 0 |
| 16 | 136 | 3,876 | 65,536 | 15 | 30, 30 | 1 / 0 / 0 |

Every planted algebraic zero recovered exactly one signed rational witness.
Both public controls recovered none. Target infinity was replayed separately;
the rational fibre had 3, 10 and 36 signed infinity witnesses at `n=4,8,16`.
Only 2, 4 and 8 of the corresponding full-fibre abscissae lifted to this curve,
which confirms why full-fibre singularity cannot replace rational recovery.
[pcc:unverified]

The monomial verifier matched the complete Dickson product table and the full
difference, sum/reflection and total multiplication matrices for
`n=2,4,8,16,32,64`. All divisor-remainder recurrences passed for distinct
divisor degrees one through four. For every target, the five multiplicity-chart
counts summed to 35, 330 and 3,876 and their canonical zero sets equalled the
ordered-kernel zero set after multiset reduction. [pcc:unverified]

## Why the ranks stop short of a query algorithm

For one pair with elementary invariants `s=x_1+x_2` and `p=x_1*x_2`, write

```text
S_3(x_1,x_2,z) = c_2 z^2 + c_1 z + c_0,
c_2 = s^2 - 4p,
c_1 = -2(s(p+a)+2b),
c_0 = (p-a)^2 - 4bs.
```

Use the symmetric resultant split

```text
S_5 = Res_z(S_3(x_1,x_2,z), S_4(x_3,x_4,r,z)).
```

This resultant is homogeneous of degree four in `(c_0,c_1,c_2)`. Its row
features therefore lie in the 15-dimensional fourth Veronese vector
`nu_4(c_0,c_1,c_2)`. Symmetry gives a factorization of the pair-flattened
kernel through at most 15 coordinates. Consequently

```text
rank(K_r) <= 15,
rank(diag(s)K_r - K_r diag(s)) <= 30,
rank(diag(p)K_r - K_r diag(p)) <= 30.
```

The `n=8,16` cells saturate all three bounds. Thus the measured `15/30/30`
pattern is explained by the resultant's fixed feature space; it is not by
itself an index for a zero entry. [pcc:unverified]

The controls make that distinction executable. Every one of the nine kernels
is rank deficient. Only the three planted kernels contain a zero entry, so
`det(K_r)=0` and `rank(K_r)<N` produce six false positives out of six unplanted
controls. The literal exact predicate remains the diagonal operator on
`vec(K_r)`, of dimension `N^2=n^4`. [pcc:unverified]

## Best next lead: degree-two Dickson remainder transfer

The strongest remaining lead combines the rank-15 polar form with an implicit
pair divisor; it does not construct elliptic `A+A` or exchange the factor base.
For

```text
q_(s,p)(T) = T^2 - sT + p,
D_(2^j)(T) mod q = u_j T + v_j,
```

the Dickson doubling identity gives the fixed quadratic transfer

```text
u_0 = 1,                         v_0 = 0,
u_(j+1) = u_j^2 s + 2u_j v_j,
v_(j+1) = v_j^2 - u_j^2 p - 2.
```

For a distinct unordered pair of roots of `D_(2^t)`, pair membership is exactly
`u_t=v_t=0`. Repeated-root pairs form a separate diagonal chart. The direct
four-leaf query can therefore be written as two copies of this `O(log n)`
pair-membership circuit plus the target-specific polar equation

```text
nu_4(c(s,p))^T C_r nu_4(c(s',p')) = 0.
```

This is the concrete Dickson-substituted PKM/Semaev elimination to pursue. The
missing step is a transfer/subresultant algorithm that eliminates one pair
circuit without expanding its `Theta(n^2)` pair algebra, then returns either a
witness or an exact finite miss certificate. Merely handing this quadratic
circuit to a generic solver would not meet the gate. [pcc:unverified]

The next discriminating proof experiment is to propagate the polar equation
backwards through one Dickson-doubling level and bound the number and degrees of
the resulting coefficient states. Promotion requires a recurrence whose live
state and arithmetic are `o(n^2)` on at least four growing sizes, with exact
diagonal/off-diagonal charts and independent miss replay. If the symbolic state
already grows as `Theta(n^2)` before the second-pair elimination, this arm stops
without another kernel census. [pcc:unverified]

Two lower-priority backups remain:

1. express the same pair circuit in norm-one-torus exponent coordinates and
   test whether the polar form has a sparse binomial resultant below its mixed
   volume; and
2. use fixed-dimensional orthogonal-vector incidence algorithms on the
   15-coordinate features only if they operate on the implicit Dickson pair
   scheme. An algorithm that first lists all `N=n^2` pairs cannot satisfy this
   protocol's stricter leaf-scale online gate.

The canonical open obstruction remains
[`problems/prime31-exact-pair-spectrum-query.md`](../../problems/prime31-exact-pair-spectrum-query.md).

## Sources checked

The literature and repository review was frozen before source in `PROTOCOL.md`.
It includes Semaev's summation-polynomial recursion, Petit--Kosters--Messeng's
symmetry/descent method, Amadori--Pintore--Sala's Weil-restriction analysis,
Faugere--Gaudry--Huot--Renault's invariant formulation, the special-resultant
and structured-linear-algebra work of Bostan and coauthors, and the Dickson
identities of Abhyankar--Cohen--Zieve. The most relevant internal controls are
the factorized `S_4` resultant, rank-three `S_3` pencil, pair-norm counterexample,
kSUM frontier and rank-star contraction named there.

The shared KB was unavailable (`scripts/research-kb status`: no executable), so
the review used the committed sources and primary papers listed in the protocol.
No novelty claim is made from the unavailable search service.
