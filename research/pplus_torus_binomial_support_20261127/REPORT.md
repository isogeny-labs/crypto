# Norm-one-torus binomial support of the rank-15 S5 polar

## Outcome

The local exact run closes the plain Laurent-support/binomial-resultant arm for
the two public targets tested. After

```text
x_i = z_i + z_i^-1,
F_r = S_5(x_1,x_2,x_3,x_4,r),
```

the target-specialized polynomial has every exponent in the complete box
`[-8,8]^4`: 83,521 nonzero Laurent coefficients for `G`, and independently
83,521 for `[17]G`. The eliminated-pair projection is the complete
`[-8,8]^2` square, with primitive rank-two exponent lattice and no extra
small unimodular symmetry beyond coordinate inversion and pair swap.
[pcc:unverified]

Those exact facts rule out the hoped-for support collapse, univariate monomial
substitution, or rank-one pair separation. They do not rule out every
arithmetic circuit for the zero query. The coefficient matrix across the two
pairs still has rank 15, exactly preserving the parent S5 polar structure.
[pcc:unverified]

For the specified two-binomial norm/resultant route, however, the surviving
state is quadratic. The torus root grid has `4n^2` ordered points, inversion
reduces it exactly to `n^2` ordered Dickson pairs, and swap reduces it to

```text
q_t = n(n+1)/2 = (4^t+2^t)/2,
q_(t+1) = 4q_t - 2^t.
```

The frozen verdict is therefore `TORUS_QUADRATIC_STOP`. This stops this route
before an eliminator or m=7 census; it is not a universal lower bound and does
not establish a speedup. [pcc:unverified]

## Exact local result

The curve and targets are the registered public Prime31 fixture:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644*x + 21
G     = (1391443431, 177979044)
[17]G = (1960828035, 1167167028)
```

Each target was built in a separate process. No evaluated polynomial, table,
transform, or transcript was reused between them.

| quantity | `G` | `[17]G` |
| --- | ---: | ---: |
| pair-invariant terms after target specialization | 2,025 | 2,025 |
| Laurent terms | 83,521 | 83,521 |
| coordinate bounds | `[-8,8]^4` | `[-8,8]^4` |
| eliminated-pair support | 289 | 289 |
| eliminated-pair hull | `[-8,8]^2` | `[-8,8]^2` |
| hull twice-area | 512 | 512 |
| affine rank / lattice index | 2 / 1 | 2 / 1 |
| pair coefficient-matrix size | `289 x 289` | `289 x 289` |
| pair coefficient-matrix rank | 15 | 15 |
| `GL(2,Z)`, entries `[-2,2]`, exact stabilizers | 8 | 8 |
| exact leaf-permutation stabilizers | 24 | 24 |
| canonical map digest | `ba510c93769795a84614b0b7b696ade2` | `8ba359d0761f2fdcbb6f2c02efae1da7` |

The eight eliminated-pair stabilizers are precisely the signed permutation
matrices. The target digests differ, while all structural quantities agree.
The support count is not merely close to the ambient box: `83,521=17^4` and
`289=17^2`, so every possible exponent in those bounds occurs with a nonzero
coefficient for both targets. [pcc:unverified]

## Why the torus quotient remains quadratic

Abhyankar--Cohen--Zieve's Dickson functional identity gives

```text
D_n(z+z^-1,1) = z^n + z^-n.
```

Thus `D_n(x)=0` is equivalent to `z^(2n)+1=0`, with `z` nonzero. On the
Prime31 norm-one torus and the frozen depths, inversion has no fixed point on
this root set. Each Dickson abscissa therefore has exactly two torus lifts,
`z` and `z^-1`.

For `M=2n`, D'Andrea--Sombra's sparse-resultant partial-degree formula gives

```text
deg_F Res_(z3,z4)(F, z3^M+1, z4^M+1)
  = MV([0,M e3],[0,M e4])
  = M^2
  = 4n^2.
```

The Poisson product has one factor per ordered torus pair. Because `F` is
invariant under inversion of either coordinate, that product is the fourth
power of the ordered Dickson-pair product. Symmetry in the two leaves then
leaves `n(n+1)/2` unordered pairs with repetition. These are exact orbit and
resultant degrees; the program derives them through `t=29` with checked
integer arithmetic without constructing any root or pair. [pcc:unverified]

Inversion and swap are worthwhile constant-factor reductions. They do not
change the exact asymptotic base four of this intermediate. The primitive
support lattice index one also excludes a further power substitution such as
`F(z3,z4)=G(z3^k,z4^k)` with `k>1` for either fixed target.

## Independent replay

The candidate and verifier do not share Rust modules.

- The candidate constructs the pair-invariant S5 polar using the padded
  Semaev/Sylvester recursion and a subset-dynamic determinant. It substitutes
  the pair invariants into Laurent coordinates by exact sparse linear algebra.
- The verifier constructs S5 directly in `(x1,x2,x3,x4,r)`, evaluates all 720
  determinant permutations, specializes the target, and applies four separate
  binomial expansions for `x_i=z_i+z_i^-1`.
- For each target the verifier compared all 83,521 coefficients, 32 fixed
  nonzero numeric tuples, the Newton/lattice/rank/stabilizer records, and all
  29 symbolic orbit rows. Both verifier records report zero errors.
- The analyzer reparses the complete canonical maps and recomputes every
  decision quantity rather than trusting producer totals.

Source tree `949063a17ca3c435516560fd0019c894d8066cc5` was hashed and committed
before execution. `verify_evidence.sh` checked the source and package hashes,
rebuilt with Rust 1.100.0, decompressed both canonical maps, reran the two
verifiers and the analyzer, and reproduced the retained outputs byte for byte.
The seed manifest is `null`. No root grid, pair list, pair FFT, elliptic
`A+A`, generic Groebner solve, or cross-target amortization appears in the
run. [pcc:unverified]

This is public deterministic replay, not a zero-knowledge proof, trusted
execution receipt, or signed PCC verdict. The result remains
`[pcc:unverified]`.

## Assessment against PKM and Amadori--Pintore--Sala

The experiment supplies a missing concrete cost boundary in the previously
reviewed prime-field index-calculus proposals. PKM's composed selector exposes
the algebraic solve as `T(E,m,L)`; it does not give an exact subquadratic
zero-entry algorithm for this rank-15 pair polar. Amadori--Pintore--Sala's m=7
comparison assumes a solver cost, while its reported experiments stop below
m=7. Neither paper licenses replacing the present `n(n+1)/2` intermediate by
a constant-size resultant handle. [pcc:unverified]

The result does not contradict either paper. It evaluates one particular
Dickson/norm-one-torus specialization of the missing solve term and finds that
plain sparse support and finite torus symmetries alone do not lower its
pair-scale exponent.

## Best successor: a low-rank special-resultant ladder

The rank-15 coefficient channel survives, so the next lead should attack the
quadratic *second elimination*, not change the factor base.

After clearing only the `z4^8` monomial, `F` has degree 16 in `z4`. The first
special resultant

```text
R_3(z1,z2,z3) = Res_z4(F, z4^(2n)+1)
```

has degree only `O(n)` in each retained variable, and the special-resultant
literature gives fast ways to compute such a binomial composition while
pricing its output dimension. The naive next elimination by `z3^(2n)+1`
expands to a quadratic coefficient tensor. The discriminating question is
whether the exact rank-15 polar makes `R_3` retain a bounded displacement or
generator rank so that the second elimination can operate on generators
without materializing that tensor.

A successor protocol should therefore:

1. construct `R_3` exactly for growing `n=2^t` without evaluating a root grid;
2. measure and independently verify Toeplitz/Hankel/displacement generators
   across the `(z1,z2)` coefficient slices;
3. stop if generator count, retained width, or witness state has exact
   recurrence base four; and
4. promote only an ordinary-field recurrence with base below four that also
   recovers an exact four-leaf witness or finite miss for one target.

This is a Dickson-substituted PKM/Semaev elimination lead. It is not another
factor-base swap, does not assume an S3 oracle, and does not permit
cross-target amortization. No claim is made yet that its generator rank stays
bounded. [pcc:unverified]

## Sources checked

The prospective review is recorded in `PROTOCOL.md`. The primary sources most
directly used here are:

- Abhyankar--Cohen--Zieve, equation (1.6),
  <https://sites.lsa.umich.edu/zieve/wp-content/uploads/sites/822/2026/03/acz.pdf>;
- D'Andrea--Sombra, Proposition 3.4 and Theorem 1.1,
  <https://www.maia.ub.edu/~sombra/publications/SparseResultants/SparseResultants.pdf>;
- Emiris--Canny, Theorem 3.10,
  <https://people.eecs.berkeley.edu/~jfc/papers/95/cejsc95.pdf>;
- Semaev, <https://eprint.iacr.org/2004/031.pdf>;
- Petit--Kosters--Messeng,
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>; and
- Amadori--Pintore--Sala,
  <https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf>.

The shared research KB was unavailable (`scripts/research-kb status` exited
127), so the review used these primary sources and the committed parent studies
listed in the protocol. No novelty conclusion is inferred from the unavailable
catalog.
