# Mixed Frobenius sectors and Semaev fiber multiplicity

This is an independently reconstructed derivation and native development
gate. It does not identify the text of the earlier, unpublished multiplicity
invariant. Mathematical arguments below are written arguments, not Lean
theorems or signed PCC verdicts. New arithmetic uses only
`p = 2147483647`, `a = p - 3`, `b = 21`. [pcc:unverified]

## 1. The exact specialized divisor

Let `k` have characteristic greater than three and let
`E : y² = x³ + ax + b` be nonsingular. Fix `m >= 2` finite abscissae
`x_1,...,x_m` and geometric lifts `P_i`. Put `D = 2^(m-1)` and

`F(X) = S_(m+1)(x_1,...,x_m,X)`.

The indexing set is the `D` sign vectors modulo simultaneous negation.
One may choose representatives with first sign `+1`. Keep this indexing
multiset: two sign vectors can give the same point or the same abscissa.
For each representative let `Q_e = sum_i e_i P_i`.

The homogeneous fiber has the divisor

`div(F_hom) = sum_e [x(Q_e)]` on the projective Kummer line,

where `x(O) = infinity` and repetitions count. Equivalently, for a nonzero
scalar `c`,

`F_hom(U,V) = c V^r product_(Q_e != O) (U - x(Q_e)V)`,

where `r` is the number of identity sums, counted with their sign-vector
multiplicity. Thus the affine fiber is nonzero, its actual degree is
`D-r`, and its finite-root multiplicities are exactly the multiplicities
in the signed-point multiset. A constant affine fiber is possible; a zero
fiber is not an acceptable normalization input. [pcc:unverified]

One justification that retains exceptional fibers is as follows. Pull the
projective Semaev hypersurface back along the covering that orients the
input points. Generically the `D` graphs of the maps
`(P_1,...,P_m) -> x(sum e_i P_i)` are distinct and are exactly its roots.
The projective Semaev projection is finite and flat of degree `D`.
Consequently there is no vertical divisor component to introduce or remove
upon specialization. Equality with the sum of those graph divisors extends
over the input space. The maps to the projective Kummer line are defined
even when a partial sum or the final sum is the identity. This is why one
must retain homogeneous Sylvester degrees: replacing a degree-two input
by its smaller specialized affine degree can delete an infinity factor.

Diem's Proposition 3.2(c) establishes the finite-flat projection and its
degree; Lemma 3.5 uses fixed homogeneous Sylvester degrees for the
recursion. The signed-divisor argument here is a deduction from that
geometry and generic factorization, not a claim that the paper states the
mixed-sector consequence below verbatim.

## 2. Even multiplicity in a mixed specialized fiber

Assume now `k = F_p`, all fixed `x_i` are rational, and
`f(x_i) = x_i³ + ax_i + b != 0`. Each chosen lift lies in `E(F_(p²))` and
has Frobenius sign `sigma_i` in `{+1,-1}`:

`Frob(P_i) = sigma_i P_i`.

Suppose both signs occur among the **fixed** inputs. Frobenius sends a
sign class `[e]` to `[sigma e]`. This involution has no fixed sign class:
a fixed class would require `sigma` to be uniformly `+1` or uniformly
`-1`. If the abscissa `x(Q_e)` is rational, Frobenius preserves that
abscissa. The sign classes contributing to this root therefore split into
pairs. Its multiplicity is even. Infinity classes also pair, so `r` is
even in this mixed case. [pcc:unverified]

In particular every rational finite root satisfies `F'(X)=0`, and

`gcd(F, X^p-X)` divides `gcd(F,F')`.

This implication concerns the rational-root factor. It does **not** assert
that every geometric root has even multiplicity, that `F` is globally a
square, or that a square root can be obtained without additional work.
Nonrational roots can form distinct conjugate pairs with odd individual
multiplicity.

Pure `+` and pure `-` input fibers are necessary negative controls.
Frobenius acts trivially on their sign classes modulo global negation;
their rational roots can be simple. Adjoining `F'=0` to those fibers can
delete valid roots. A repeated-root condition is therefore not a valid
unconditional replacement for the original Semaev constraint.

## 3. Strengthening to the full singular locus

Let `(x_1,...,x_N)` be a rational zero of `S_N`, with every `f(x_i)`
nonzero and with both Frobenius signs present. Choose lifts and signs
realizing the zero sum. Write `A` for the sum of the `+` sector and `B`
for the sum of the `-` sector. The relation and its Frobenius image give

`A+B=O`, `A-B=O`, hence `A=B=T` with `2T=O`.

Each sector contains at least two leaves. A sector consisting of one
leaf could equal `T` only if that leaf were two-torsion, contradicting
its nonzero ordinate. Removing any single coordinate therefore leaves a
mixed set of fixed inputs. Apply the specialized-fiber result to that
coordinate. For every `j`,

`(partial S_N / partial x_j)(x_1,...,x_N) = 0`.

Thus these mixed rational zeros lie in the singular locus of the
Semaev hypersurface. Any derivative along a fixed affine parameterization
also vanishes there by the chain rule. The converse is not asserted:
pure-sector points can be singular for other reasons, including collisions
of signed sums. [pcc:unverified]

This argument excludes zero-ordinate leaves. At such a leaf the sign of
Frobenius is not unique, the input double cover is ramified, and the
sector-size argument fails. Those fibers need their own branch. The frozen
M31 curve in this study has no rational zero-ordinate leaf: the native gate
computes the exact polynomial gcd

`gcd(X³-3X+21, X^p-X) = 1`.

The implementation records this domain fact instead of changing the curve
to manufacture a torsion example. It separately checks the field-level
zero-square-root primitive. That primitive check is not relabelled as a
curve two-torsion test.

## 4. Native gate and independence boundary

`src/multiplicity.rs` uses its own M31 arithmetic and its own quadratic
extension with `omega²=-1`. It does not call the candidate tensor or
quotient implementation. One path forms exact fixed-degree Sylvester
determinants and reconstructs each coefficient vector using interpolation
at consecutive field elements. The other uses complete elliptic group
addition, retaining all signed branches modulo global negation.

Every fixture checks all of the following:

- The affine resultant fiber is nonzero.
- The monic full coefficient vector equals the signed-branch product over
  `Fp²`, with duplicates retained.
- Actual affine degree plus identity-branch multiplicity equals `2^(m-1)`.
- `gcd(F,X^p-X)` equals the product of **all distinct** rational abscissae
  in the signed-sum oracle. This is complete rational-root-set equality.
- Repeated exact division gives the same multiplicity as the signed
  branch multiset at each rational root.
- Mixed fixtures satisfy even multiplicity and derivative vanishing.
- Adding one to the constant coefficient is rejected using an existing
  independently supplied root and the full branch product.

The fixed deterministic panel has 50 fixtures in each of nine cells:
`m=3,4,5`, crossed with pure `+`, pure `-`, and mixed positive inputs.
The mixed fixtures are deliberately positive: a cancelling twist pair for
`m=3,4`, or a three-point twist zero-sum for `m=5`, accompanies one or two
base-sector leaves. A finite base-sector target completes the relation.
Every partial derivative is checked at each complete mixed tuple, giving
`50*(4+5+6) = 750` partial-derivative checks.

Ten additional fixtures per `m` force an identity signed sum and repeated
abscissae. They use `(P,P,2P)`, `(P,P,Q,Q)`, or `(P,P,2P,Q,Q)` as input
abscissae. All 30 must have an actual degree drop. Twenty additional mixed
fixtures, ten each for `m=4,5`, combine a cancelling base-sector pair with
a cancelling twist pair or twist triple. They require a degree drop and
explicitly test that its infinity multiplicity is even. The total panel is
500 specialized fibers. Pure-sector cells must each contain a simple-root case,
so the derivative negative control cannot pass vacuously. Zero-polynomial
and noncanonical-input controls are explicit.

The generator seed, exact curve, cell counts and a noncryptographic input
checksum are printed as JSON by `run_multiplicity_gate()`. Compiler/test
receipts and source SHA-256 must be retained with any reported execution.
The sign oracle is independently formulated within this module, but both
paths still share this module's field arithmetic. This is not an independent
signed execution attestation and does not replace external replay.

## 5. Consequences for the proposed comparison

The invariant can support a separately guarded multiplicity-aware branch.
It cannot be inserted into every factor-base relation system: points of
an ordinary factor base on `E(Fp)` and the rational target belong to the
pure `+` sector. A full decomposition must retain pure-sector, mixed-sector,
and any applicable zero-ordinate branches, and recover the same projected
set after their union. Branch selection, field restrictions, root recovery
and duplicate removal all remain part of the charged computation.

These 500 fixtures are specialized univariate correctness tests. They are
**not** the requested 50 unspecialized projected Gröbner solves per
configuration, and supply no F4 runtime, maximum Macaulay degree or memory
measurement. The latter requires the matched monolithic, preclosed
sequential, preclosed balanced and multiplicity-aware systems with common
input restrictions and resource limits. [pcc:unverified]

## Sources checked

- Claus Diem, *On the discrete logarithm problem in elliptic curves*,
  Compositio Math. 147 (2011), 75–104, Proposition 3.2(c), Lemmas 3.4–3.5:
  <https://doi.org/10.1112/S0010437X10005075>.
  Full primary PDF read; finite-flat projection, projective Kummer
  semantics and homogeneous resultants inform the exceptional-fiber proof.
- Alessio Caminata and Elisa Gorla, *Solving degree, last fall degree, and
  related invariants*, <https://arxiv.org/abs/2112.05579>.
  Generator degree and a derivative consequence are not solving-degree
  bounds.
- Existing repository `summation_split/SECTOR_FINDINGS.md`,
  `SECTOR_COMPLETE.md`, `VERONESE_FINDINGS.md`, and
  `M31_LOCAL_PRECLOSURE.md` at PR #5087 head
  `76998f1c0b15255bb26ea60427e35c52b968776e`.
- Semaev's ePrint PDF was unavailable (403) in this audit; no unread
  contents are cited as evidence. KB MCP was not loaded and the repository
  wrapper reported no executable, so exact repository and primary-source
  fallback was used. No novelty or priority claim is made.
