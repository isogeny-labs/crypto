# Protocol: four-leaf-first Dickson-substituted S5 structural gate

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-dickson-s5-structure-20261127`. Draft PR: #5287.
Standing: prospective deterministic algebra/structure experiment,
`[pcc:unverified]`.

This protocol is frozen before candidate source, direct-S5 kernel ranks,
quartic-divisor measurements, or a decision outcome exist. It authorizes only
the bounded structural cells below. It does not authorize a relation census,
an m=7 projection, or sharing work across unrelated targets.

## 1. Question and success boundary

Let `n=2^t` and

```text
F_n(X) = D_n(X,1),
D_0=2, D_1=X, D_(2m)=D_m^2-2.
```

For a public target `R=(r,y_R)`, the direct four-leaf equation is

```text
F_n(x_i)=0  (i=1,2,3,4),
S_5(x_1,x_2,x_3,x_4,r)=0.
```

The final answer is relative to the actual factor base: each `x_i` must lift
to the registered prime-field curve, a rational sign choice must sum to `R`,
and the signed row must replay on the curve. Roots of `F_n` that do not lift
are algebraic controls, not factor-base points.

The study asks two separate questions.

1. Does the direct target-specialized `S_5` object inherit exact
   Toeplitz-plus-Hankel/Dickson displacement structure?
2. Does that structure yield a deterministic exact witness-or-miss query with
   target-specific work `o(n^2)`, without constructing elliptic `A+A`?

Question 1 may pass while Question 2 fails. Low ordinary rank, low
displacement rank, a short circuit, a planted hit, or a compact description of
an operator does not by itself locate a zero entry. Promotion requires an
executable zero predicate and recovery path whose production and independent
verification both meet the stated cost boundary.

## 2. Literature and committed controls read before design

The shared research-KB MCP is not loaded. `scripts/research-kb status`
returns `no executable found`; the fallback below is therefore bounded and no
literature-wide novelty claim is made.

Primary sources:

- Semaev, *Summation polynomials and the discrete logarithm problem on
  elliptic curves* (2004), Theorem 1 and the resultant recursion,
  <https://eprint.iacr.org/2004/031.pdf>.
- Petit--Kosters--Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields* (PKC 2016), especially the
  composed low-degree selector and the unresolved solve term `T(E,m,L)`,
  <https://www.iacr.org/archive/pkc2016/96140156/96140156.pdf>.
- Amadori--Pintore--Sala, *On the discrete logarithm problem for prime-field
  elliptic curves* (FFA 2018), Sections 3--4. Its m=7 comparison assumes
  comparable solve costs rather than executing an m=7 extractor,
  <https://doi.org/10.1016/j.ffa.2018.01.009>.
- Faugere--Gaudry--Huot--Renault, *Using symmetries in the index calculus for
  elliptic curves discrete logarithm*, for the established invariant-coordinate
  treatment of symmetric Semaev systems,
  <https://doi.org/10.1007/s00145-013-9158-5>.
- Bostan--Flajolet--Salvy--Schost, *Fast Computation of Special Resultants*,
  especially Theorem 1 and Section 4. Its bounds price the retained quotient
  or output dimension; they are not a subquadratic zero-entry theorem,
  <https://cs.uwaterloo.ca/~eschost/publications/BoFlSaSc05.pdf>.
- Bostan--Jeannerod--Schost, *Solving structured linear systems with large
  displacement rank*, for the exact displacement-generator model and
  `O~(alpha^(omega-1) N)` structured solve bound in ambient dimension `N`,
  <https://cs.uwaterloo.ca/~eschost/publications/tcs.pdf>.
- Abhyankar--Cohen--Zieve, *Bivariate factorizations connecting Dickson
  polynomials and Galois theory*, equations (1.6)--(1.8), for the Dickson
  functional, product, and composition identities,
  <https://sites.lsa.umich.edu/zieve/wp-content/uploads/sites/822/2026/03/acz.pdf>.

Repository controls:

- `research/pplus_dickson_norm_gcd_20261127/{PROTOCOL.md,REPORT.md}`;
- `research/prime31_ic_20261115/native/
  M4_FACTORIZED_S4_RESULTANT_PROTOCOL_20261115.md` and its implementation note;
- `research/prime31_summation_compression_20261122/iteration9/
  PAIR_SPECTRUM_RESULTANT.md`;
- `research/prime31_summation_compression_20261122/iteration10/
  S3_PENCIL_CONTRACTION.md`;
- `research/prime31_summation_compression_20261122/iteration10/
  quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`;
- `research/prime31_summation_compression_20261122/iteration13/
  {EXPLICIT_KSUM_FRONTIER.md,RANK_STAR_CONTRACTION.md}`;
- `research/d4_symmetrised_s5_20261015/{PROTOCOL.md,README.md}`; and
- the existing Rust Dickson quotient and `ParamSymmetricSemaev` sources.

The controls establish three scope restrictions. The actual elliptic pair
spectrum need not be a Dickson tower. A rank-three S3 value pencil does not
index its zero components. Symmetrized S5 engineering over extension fields
does not supply the requested prime-field direct zero query.

## 3. Exact Dickson quotient normalization

Work over `Fp` with `p=2^31-1`; `2` is invertible. Use the basis

```text
B_n = (D_0,D_1,...,D_(n-1)),  D_0=2.
```

Thus the multiplicative identity has coordinate `(1/2,0,...,0)`. For
`0<=i,j<n`, first use

```text
D_i D_j = D_(i+j) + D_|i-j|.
```

Then reduce modulo `F_n=D_n` with

```text
D_n       = 0,
D_(n+k)   = -D_(n-k)       (1<=k<n).
```

The second identity follows from `D_n D_k=D_(n+k)+D_(n-k)`. No coefficient
normalization may silently replace `D_0=2` by `1`.

For `a=sum a_i D_i`, multiplication by `a` is constructed from two explicit
channels: the difference channel `D_|i-j|` and the sum/reflection channel
`D_(i+j)`. Their matrices are Toeplitz/Hankel-like up to the declared `D_0`
normalization and the `i+j=n` boundary. The candidate must emit both channels,
their sum, and exact finite-field displacement ranks under its declared shift
operators; it may not infer structure from a picture or from sparsity alone.

An independent monomial-basis implementation builds `D_n`, converts every
basis vector, multiplies and reduces in `Fp[X]/(D_n)`, and compares the entire
multiplication table. This check runs for `n=2,4,8,16,32,64`.

## 4. Direct four-leaf objects; no elliptic pair spectrum

### 4.1 Pair-flattened S5 value kernel

For the full Dickson fibre `X_n`, index ordered leaf-coordinate pairs by
`a=(i,j)` and `b=(k,l)`, so `N=n^2`. Define

```text
K_r[a,b] = S_5(x_i,x_j,x_k,x_l,r).
```

`S_5` is evaluated by the registered Semaev resultant recursion with padded
formal degrees. The candidate separately evaluates the same value using a
6-by-6 Sylvester determinant. All special degree drops remain values of the
specialized polynomial; they are not automatically rational relations.

In the evaluation basis define pair invariants

```text
s_(i,j)=x_i+x_j,
p_(i,j)=x_i*x_j.
```

Measure exact ranks over `Fp` of

```text
K_r,
Delta_s(K_r)[a,b] = (s_a-s_b) K_r[a,b],
Delta_p(K_r)[a,b] = (p_a-p_b) K_r[a,b].
```

These are the evaluation-basis forms of Sylvester displacements by the
Kronecker sum/product of the one-variable Dickson multiplication operator.
Rank is invariant under the full Dickson evaluation transform. The ranks test
the proposed pair-flattened displacement structure; they do not decide whether
an individual entry is zero.

### 4.2 The exact zero operator

A four-leaf algebraic hit exists exactly when at least one entry of `K_r` is
zero. Singularity of `K_r` is not an equivalent predicate. The literal
zero-eigenvalue operator is

```text
Z_r = diag(vec(K_r)),
```

of ambient dimension `N^2=n^4`. A compact formula for its entries does not
reduce that dimension. Applying a standard displacement solver in ambient
dimension `n^4` remains at least linear in `n^4`, even at constant displacement
rank, and is therefore quadratic in the pair domain.

The analyzer records, for every cell:

- actual zero count and canonical first zero;
- `rank(K_r)`, `rank Delta_s`, and `rank Delta_p`;
- whether the tempting predicates `det(K_r)=0` and `rank(K_r)<N` agree with
  the actual zero-entry predicate; and
- `N`, `N^2`, and the corresponding unordered-pair and four-multiset counts.

No `A+A` point, group-sum dictionary, pair-sum polynomial, or elliptic
pair-spectrum root is constructed.

## 5. Novel compact arm: quartic-divisor Dickson/PKM circuit

A four-leaf multiset with `d` distinct abscissae is represented by a monic
degree-`d` polynomial

```text
q(Z)=product_(u in U)(Z-u),   1<=d<=4.
```

For distinct full-fibre roots, membership is exactly `q | D_n`. Compute this
constraint without expanding `D_n` in the unknown coefficients of `q`:

```text
rho_0(Z) = Z mod q,
rho_(j+1)(Z) = rho_j(Z)^2 - 2 mod q,
rho_t(Z) = 0.
```

Each `rho_j` has fewer than `d` coefficients. Introducing every coefficient
of every `rho_j` as an auxiliary variable gives an `O(d log n)`-size
PKM-style quadratic circuit plus the final symmetrized target equation.

Completeness requires all multiplicity partitions of four:

```text
4; 3+1; 2+2; 2+1+1; 1+1+1+1.
```

For a partition with `d` parts, enumerate the constant number of assignments
of the multiplicities to the `d` distinct roots and specialize `S_5`
accordingly. This is a union of five fixed chart types, not an assumption that
four abscissae are distinct.

The candidate verifies the remainder recurrence on every monic divisor made
from at most four roots for development `n<=16`, and verifies every target
zero against direct `S_5` evaluation. The independent verifier instead
evaluates `D_n` at each claimed root and reconstructs the multiset directly.
[pcc:unverified]

The compact circuit is not automatically a fast solver. Before the target
equation, the distinct-divisor algebra has dimensions `binom(n,d)`; across all
five charts the four-multiset state count is

```text
binom(n+3,4).
```

A multiplication/minimal-polynomial operator that explicitly retains this
algebra is quartic in `n` and fails the online gate. The experiment records the
circuit size and the exact quotient-state count separately. A future solver
passes only by proving and executing a contraction below that state space.

## 6. Fixed public inputs and deterministic cells

Use registered curve `icv1-fp31-tm3989-ae9db972`:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644*x + 21
group order = 2147487637
G = (1391443431,177979044)
```

There is no random stream and therefore no seed manifest. Root order is the
same deterministic backwards-square-root order as the completed norm-gcd
study: at every level, the smaller square root is branch zero and its negative
is branch one.

Cells:

| component | depths / n | targets |
|---|---|---|
| quotient fold/table | `t=1..6`, `n=2..64` | fixed dense coefficient vector `a_i=(i+1)^3+17 mod p` plus every basis multiplier |
| full-fibre S5 kernel/ranks | `t=2,3,4`, `n=4,8,16` | deterministic planted target, `G`, `[17]G` |
| lift-filtered exact replay | `t=2,3,4` | same targets, every rational sign choice |
| divisor circuit/charts | `t=2,3,4` | all multisets and the same targets |

The planted target is the sum of the lexicographically first signed four-leaf
multiset with repetition whose sum is finite and nonzero. This selection rule
is frozen; a later run may not replace an inconvenient target. `G` and `[17]G`
are unrelated public controls. `R=O` is excluded from the affine `S_5` cells
and tested separately by direct signed replay.

The development cap `n=16` for the target kernel is intentional: a full
`n=32` kernel already has `n^4` entries and would itself execute the forbidden
quadratic pair-domain path. Quotient identities alone still run through
`n=64` because they use `O(n^2)` development tables and no target query.

## 7. Prospectively fixed pass/stop rule

### Algebra and structure pass

All of the following are required:

1. every Dickson product/fold coefficient matches the independent monomial
   quotient through `n=64`;
2. the two multiplication channels sum to the exact multiplication matrix;
3. the two independent `S_5` evaluators agree on every kernel entry;
4. divisor-remainder charts and direct multiset enumeration agree exactly;
5. every lift-filtered zero has the same complete signed witness set as direct
   group enumeration, and every reported miss has none; and
6. every exception or resource cap returns `INDETERMINATE`, never `EMPTY`.

The analyzer may label low pair-kernel displacement rank as
`STRUCTURE_PRESENT` only at this scope.

### Algorithmic promotion pass

Promotion to a larger run additionally requires one concrete deterministic
zero oracle, implemented and independently replayed, satisfying all of:

1. source setup `O~(n)` field elements and operations beyond constructing the
   registered leaves;
2. fewer than `n^2` target-specific field-element slots and arithmetic steps,
   with a fitted log-log slope strictly below `1.9` on at least four growing
   sizes before any production extrapolation;
3. no enumeration/materialization of elliptic `A+A`, no `n^2` leaf-pair scan,
   no `N^2` S5 kernel, and no explicit `binom(n+3,4)` divisor algebra;
4. an exact witness or an exact finite miss certificate whose independent
   verification obeys the same asymptotic boundary;
5. complete treatment of nonlifting roots, repetitions, signs, target
   infinity, degree drops, and zero denominators; and
6. no target-independent work shared across unrelated DLP targets beyond the
   fixed curve/source selector already charged by the parent protocol.

`rank(K_r)<N`, `det(K_r)=0`, a low displacement rank, the short divisor
circuit, or a low planted-only cost cannot substitute for these requirements.

If algebra/structure passes but no zero oracle passes, the frozen verdict is
`STRUCTURE_ONLY_STOP`. If any exactness check fails, the verdict is
`INVALID_STOP`. Only `ALGORITHM_PASS` permits a separately preregistered
larger census or m=7 integration.

## 8. Rust implementation and independent proof model

Use a dependency-free standalone Rust crate with three binaries:

- candidate: quotient channels, direct `S_5` kernel, divisor charts and raw
  deterministic evidence;
- verifier: separately expressed monomial quotient, Sylvester resultant,
  root/multiset enumeration and signed curve replay; and
- analyzer: the frozen ranks, state counts and pass/stop decision only.

Candidate and verifier may share only a text schema and public constants, not
Rust modules. Evidence hashes detect transport drift but do not prove the
claims. All inputs and witnesses are public, so the study provides exhaustive
deterministic replay rather than a zero-knowledge proof. Any result remains
`[pcc:unverified]` until an independent signed PCC verdict exists.

No timing or fastest-implementation claim is in scope for this structural
gate. If `ALGORITHM_PASS` is ever reached, a later protocol must add matched
external and complete cold one-target comparisons before making a speed claim.
