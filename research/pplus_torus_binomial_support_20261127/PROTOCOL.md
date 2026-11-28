# Protocol: norm-one-torus binomial support gate for the rank-15 S5 polar

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-torus-binomial-support-20261127`. Draft PR: #5304.
Standing: prospective deterministic local algebra experiment,
`[pcc:unverified]`.

This protocol is frozen before candidate source, local execution, or an outcome
exists. It authorizes one bounded dependency-free Rust experiment. It does not
authorize an m=7 relation census, a production ECDLP run, a speedup claim, or
amortization across targets. [pcc:unverified]

## 1. Question and stopping boundary

Let `n=2^t`, `D_n(X)=D_n(X,1)`, and let `F_r` be the actual fifth Semaev
polynomial with the public target abscissa in its fifth slot:

```text
F_r(z1,z2,z3,z4)
  = S_5(z1+z1^-1, z2+z2^-1, z3+z3^-1, z4+z4^-1, r).
```

The experiment asks:

> After passing to norm-one-torus coordinates, does the exact Laurent support
> of the target-specialized rank-15 S5 polar admit a binomial elimination or
> lattice quotient whose one-target time and live state are `o(n^2)` without
> materializing the pair grid?

A pass requires an exposed field-arithmetic recurrence

```text
T_t, S_t <= C * rho^t * t^k,  with fixed C,k and rho < 4,
```

covering the zero query and exact witness or miss recovery. A compact name for
a norm, resultant, quotient algebra, FFT, or root grid is not a pass. Every
target is a separate process and is charged for rebuilding every evaluated
object. Reusing an evaluated transform, table, polynomial, or transcript across
`G` and `[17]G` is forbidden.

This is a gate on the specified torus/binomial elimination family. A quadratic
stop does not assert an arithmetic-circuit lower bound for every possible S5
algorithm.

## 2. Sources checked before design

The shared research-KB MCP is absent and `scripts/research-kb status` exits 127
with `no executable found`. The fallback below was used. No literature-wide
novelty claim is made.

Primary sources:

- Abhyankar--Cohen--Zieve, *Bivariate factorizations connecting Dickson
  polynomials and Galois theory*, equation (1.6), gives
  `D_n(z+z^-1,1)=z^n+z^-n`,
  <https://sites.lsa.umich.edu/zieve/wp-content/uploads/sites/822/2026/03/acz.pdf>.
- D'Andrea--Sombra, *A Poisson formula for the sparse resultant*, Proposition
  3.4 gives the mixed-volume partial degrees for arbitrary Laurent supports;
  Theorem 1.1 gives the root-product formula with its directional factors,
  <https://www.maia.ub.edu/~sombra/publications/SparseResultants/SparseResultants.pdf>.
- Emiris--Canny, *Efficient incremental algorithms for the sparse resultant and
  the mixed volume*, Theorem 3.10 states the corresponding multihomogeneous
  degree formula,
  <https://people.eecs.berkeley.edu/~jfc/papers/95/cejsc95.pdf>.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic
  curves*, supplies the zero criterion and resultant recursion,
  <https://eprint.iacr.org/2004/031.pdf>.
- Petit--Kosters--Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields*, leaves its composed-selector
  solve term explicit,
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
- Amadori--Pintore--Sala, *On the discrete logarithm problem for prime-field
  elliptic curves*, analyzes the Weil-restricted systems and gives an m=7 cost
  comparison under a solver assumption; its reported experiments stop below
  m=7,
  <https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf>.

Repository controls:

- `research/pplus_dickson_s5_structure_20261127/{PROTOCOL.md,REPORT.md}` at
  `origin/codex/pplus-dickson-s5-structure-20261127`;
- `research/pplus_pair_divisor_transfer_20261127/{PROTOCOL.md,REPORT.md}` at
  commit `fb80d45be8201b95904e6213a2675f979c9e2d90`;
- `research/prime31_summation_compression_20261122/iteration10/
  quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`;
- `research/prime31_summation_compression_20261122/iteration13/
  {EXPLICIT_KSUM_FRONTIER.md,RANK_STAR_CONTRACTION.md,
  OFFSET_PARITY_QUARTIC_NORM.md}`; and
- `problems/prime31-exact-pair-spectrum-query.md`.

These sources fix two cautions. Sparse-resultant degree prices the specified
generic resultant representation, not every circuit for its specialization.
The rank-15 pair factorization is exact, but low rank alone does not locate a
zero entry.

## 3. Exact torus substitution and orbit accounting

Work over `Fp`, `p=2^31-1`, and use only depths `t<=29`, so every required
`4n`-th root lies in the norm-one torus of `F_(p^2)` because `4n | p+1`.
From the Dickson identity,

```text
D_n(z+z^-1)=0  <=>  z^(2n)+1=0,  z != 0.
```

Put `M=2n` and `B_i=z_i^M+1`. The `M` torus roots are paired freely by
`z -> z^-1`, giving exactly `n` Dickson abscissae. For an ordered second pair,

```text
torus roots                         M^2 = 4n^2,
coordinatewise inversion quotient  n^2,
inversion plus pair swap quotient   n(n+1)/2.
```

The last count includes the repeated-leaf diagonal; the off-diagonal count is
`n(n-1)/2`. These are exact orbit counts, not a root-grid implementation.

The candidate must confirm coefficient-for-coefficient that `F_r` is invariant
under inversion of each coordinate and under every leaf permutation. It must
also search for further exact compression in the eliminated pair:

1. affine rank and Smith index of the lattice generated by support differences;
2. the full coefficient-preserving stabilizer among every matrix in
   `GL(2,Z)` with entries in `[-2,2]` acting on `(z3,z4)` exponents;
3. rank of the coefficient matrix whose rows are `(z1,z2)` exponents and whose
   columns are `(z3,z4)` exponents; and
4. one-dimensional monomial substitutions and rank-one pair separation.

A finite symmetry changes a constant only. A proper exponent sublattice or
rank-one specialization is recorded but is not called an algorithm until its
root-orbit recurrence and witness recovery meet Section 7.

## 4. Construction of the actual polar

For pair invariants `s=x1+x2`, `q=x1*x2`, define

```text
c2 = s^2 - 4q,
c1 = -2(s(q+a)+2b),
c0 = (q-a)^2 - 4bs.
```

The candidate constructs

```text
H_r(A,B) = nu_4(c(A))^T C_r nu_4(c(B))
         = S_5(x1,x2,x3,x4,r)
```

through the padded-degree Semaev/Sylvester recursion and a subset-dynamic
determinant. It then substitutes both pairs by

```text
s = z1+z1^-1+z2+z2^-1,
q = z1*z2 + z1/z2 + z2/z1 + (z1*z2)^-1,
```

and analogously for `(z3,z4)`. Laurent exponents remain signed. Multiplying by
the unique coordinatewise minimal monomial clears negative exponents only for
reporting; the Laurent support and Newton widths are measured before this
translation.

The independent verifier instead builds `S_5` directly in
`(x1,x2,x3,x4,r)` with a separately written 720-permutation determinant and
substitutes each `xi=zi+zi^-1` individually. Candidate and verifier share only
the public constants and a line-oriented evidence schema, not Rust modules.

For each target, both implementations emit and compare the full canonical
Laurent coefficient map, not only a hash or term count.

## 5. Newton and resultant quantities

For each fixed target, record:

- total Laurent terms, coordinate minima/maxima and widths;
- second-pair projected support, exact two-dimensional convex hull, normalized
  area, affine rank, and support-difference lattice index;
- coefficient-matrix rank over `Fp` and the exact small-`GL(2,Z)` stabilizer;
- the minimal denominator-clearing monomial; and
- factorization/separability checks from Section 3.

For the sparse resultant

```text
R_r(z1,z2) = Res_(z3,z4)(F_r, z3^M+1, z4^M+1),
```

the candidate and verifier independently derive, without expanding `R_r`,

```text
degree in the coefficient group of F_r
  = MV([0,M e3],[0,M e4]) = M^2 = 4n^2.
```

The Poisson product has one factor for each ordered torus root pair, subject to
the directional factors in the cited theorem. Because `F_r` is invariant under
each inversion, this product is the fourth power of the ordered Dickson-pair
product. Restricting to the symmetric pair quotient leaves exactly
`n(n+1)/2` value slots. The program must emit the exact recurrences for all
three dimensions through `t=29`; it must never enumerate a root or pair.

Mixed volume and quotient dimension are reported separately. Neither is
silently promoted to a universal time lower bound.

## 6. Fixed inputs, execution, and caps

Use registered curve `icv1-fp31-tm3989-ae9db972`:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644*x + 21
G     = (1391443431, 177979044)
[17]G = (1960828035, 1167167028)
```

Only the target abscissae enter `F_r`. Run exactly two independent candidate
processes, one for `G` and one for `[17]G`; run two independent verifier
processes the same way. An analyzer may compare their text records but may not
construct or reuse a shared evaluated object. There is no random stream and
the seed manifest is exactly `null`.

The exact polynomial map is capped at 262,144 Laurent terms and 512 MiB RSS.
The `GL(2,Z)` search is capped at entries `[-2,2]` as frozen above. Symbolic
dimension rows use checked `u128` arithmetic and `t=1..29`. A cap returns
`INDETERMINATE_CAP`, never a miss or a lower-bound claim.

No root-of-unity table, root grid, pair list, FFT over pairs, elliptic `A+A`,
generic Groebner basis, or cross-target cache is allowed.

## 7. Prospectively fixed decision rule

### Exactness gate

All are required:

1. the two S5 constructions agree coefficient-for-coefficient after the
   Laurent substitution for each target;
2. direct numeric evaluations at at least 32 fixed nonzero four-tuples agree
   with the ordinary S5 recursion for each target;
3. inversion and permutation identities hold coefficient-for-coefficient;
4. candidate and verifier agree on Newton data, lattice index, matrix rank,
   stabilizer, mixed-volume degree and every symbolic orbit row; and
5. the analyzer recomputes all counts from the canonical maps and rejects any
   forbidden materialization marker.

Any failure gives `INVALID_STOP`.

### Algorithmic pass

`ALGORITHM_PASS` requires an implemented exact zero query with witness or
finite miss recovery, and a proved ordinary-field recurrence with `rho<4` for
both time and live state. It must include inversion/swap quotient handling,
diagonal pairs, rational lift/sign filtering needed by the parent m=7 path,
and all target-specific work. A fitted slope, a low matrix rank, or a compact
resultant handle cannot substitute for this recurrence. [pcc:unverified]

### Scoped stops

- `TORUS_QUADRATIC_STOP`: the exact polar has two-dimensional primitive pair
  support, no additional growing lattice quotient, and the specified
  binomial-resultant/norm route retains the exact symmetric quotient dimension
  `n(n+1)/2 = (4^t+2^t)/2`. This stops this elimination route before an
  eliminator or m=7 census. It is not a universal circuit lower bound.
- `SPECIALIZATION_LEAD`: the actual coefficient map has a verified extra
  sublattice, factor, or symmetry that changes the recurrence; freeze a new
  protocol before implementing the resulting eliminator. [pcc:unverified]
- `INDETERMINATE_CAP`: a prospective cap is reached without the exactness or
  structural decision.
- `INVALID_STOP`: an exactness or coverage check fails.

## 8. Source custody and proof model

Use a standalone Rust crate with separate binaries:

- candidate: subset-determinant S5, pair-invariant Laurent substitution and
  all canonical records;
- verifier: permutation-determinant direct-x S5, independent substitution and
  complete record comparison; and
- analyzer: recompute the frozen metrics and apply only Section 7.

Commit source hashes before the first scientific run. The local wrapper records
the exact commands, Rust version, source commit, source manifest, stdout,
stderr and SHA-256 of every retained artifact. Full maps may be losslessly
compressed, but verification always reconstructs and compares their original
bytes.

All inputs and witnesses are public. Deterministic independent replay is the
appropriate proof model; this is **not** a zero-knowledge proof, trusted
execution receipt, or signed PCC verdict. The result remains
`[pcc:unverified]` until an independent signed PCC verdict covers it.

No timing or fastest-tool claim is in scope. If `ALGORITHM_PASS` occurs, a new
protocol must add the complete same-target cold comparison and fastest external
implementation before any speedup claim. [pcc:unverified]
