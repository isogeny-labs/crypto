# Proofs and conjectures: what is closed, what is open

Notation: `E/F_p` an elliptic curve, `#E(F_p) = N` (prime for ECCp-131). `E/F_{2^n}` for
ECC2-131 with `n = 131`. "rho" means Pollard rho at `≈ 0.886√N` (negation) iterations. A
route "threatens rho" if its total cost is `o(√N)`.

The point of separating these: the brainstorm produced verdicts too fast. Below, a claim is
either **(T)** a theorem with a proof sketch, or **(C)** a conjecture with the heuristic and
the explicit statement of what a counterexample is. The experiments test only the (C)s and
the measurable gaps the (T)s leave.

---

## Part I — Closed by theorem

### T1 (no subfield factor base over F_p). 
*Any subset `S ⊂ F_p` that is the `F_p`-points of a variety `V/F_p` of dimension `≥ 1` has
`#S = p + O(√p)`; any `V` of dimension `0` has `#S = O(1)`. Hence no algebraic factor base
of intermediate size `p^{1/m}`, `1 < m`, exists.*

Proof. Lang–Weil: for an absolutely irreducible `V/F_p` of dimension `d` and degree `δ`,
`#V(F_p) = p^d + O(δ^2 p^{d-1/2})`. For `d ≥ 1` this is `p + O(√p)` after summing components
of top dimension; for `d = 0` it is at most `δ`. A size-`p^{1/m}` set is neither. ∎

Consequence: the Weil-descent / GHS factor base (the `F_q`-points of a curve inside
`Res_{F_{q^n}/F_q} E`) has no analogue over `F_p`, because there is no proper subfield and no
positive-dimensional subvariety of the right size. This is why "descend to a smaller
subfield" is empty over a prime field, not merely unknown. The FPPR attack (H4) replaces the
subfield by an `F_2`-linear *subspace* of `F_{2^n}`; that is a prime-field-of-characteristic-2
phenomenon with no odd-`p` analogue.

### T2 (generic lower bound on both Certicom curves).
*Any algorithm that accesses `E` only through the group law, the Frobenius endomorphism, and
`End(E)` solves the DLP in `Ω(√ℓ)` operations for the large prime factor `ℓ | N`.*

Proof sketch. Shoup's generic lower bound is `Ω(√ℓ)` for the cyclic group of order `ℓ`. The
extra operations do not help: on `E(F_p)` Frobenius `π` acts as the integer `1` (every point
is `F_p`-rational), and `End(E) = Z[π]` acts through its image in `Z/N ≅ End(E)/(π-1)`, i.e.
by scalars already available to a generic algorithm. On `E(F_{2^{131}})` Frobenius acts as a
fixed integer `λ` with `λ^2 - tλ + q ≡ 0 (mod N)` on the order-`N` subgroup, again a scalar.
So the GLV/GLS speedups are constant-factor (`×√2`, `×√3`, `×√6`), not exponent-changing. ∎

This is the wall every exotic route must get under. It does **not** forbid algorithms that
use the *coordinate representation* non-generically (summation polynomials, p-adic digits,
incidence geometry): those read `x(P) ∈ F_p` as a field element, outside the generic model.
That gap is precisely H1/H2/H3/H5.

### T3 (canonical-lift fixed points under a prime-to-`p` order hypothesis).
*Let `E/F_p` be ordinary with `N = #E(F_p)` and `p ∤ N`. For its canonical lift
`Ẽ/Z_p` and lifted Frobenius `φ̃`, reduction identifies the `φ̃`-fixed points
with `E(F_p)`. Each fixed point has order prime to `p`. Every additive
homomorphism defined on this fixed subgroup with torsion-free codomain
vanishes on it.*

Proof sketch. Write `t = p + 1 - N`. The two conditions are distinct:
ordinarity gives `t ≠ 0 mod p`, while `p ∤ N` gives `t ≠ 1 mod p`. The unit
root `a` of the Frobenius polynomial `X²-tX+p` satisfies `a ≡ t mod p`, so
`a-1` is a unit under the second condition. Consequently `φ̃-1` has unit
linear term on the formal group and is locally invertible there. Hensel
lifting gives one fixed point in each residue disc; reduction identifies
the fixed subgroup with `E(F_p)`. Its order is `N`, which is prime to `p` by
hypothesis. Any additive map from this finite group to a torsion-free
additive group is zero. The formal elliptic logarithm is defined on the
formal subgroup; applying a logarithm to other fixed points requires an
explicit extension and domain check. ∎

The prime-to-`p` condition is essential to the stated argument. The smooth
curve `y²=x³+3x+2` over `F₅` has five group points and trace one. It is
ordinary because its trace is nonzero modulo five, yet `5 | #E(F₅)` and the
unit root satisfies `a ≡ 1 mod 5`. The actual point count, trace, and
discriminant are checked in Lean in
[`AnomalousOrdinaryException.lean`](https://github.com/isogeny-labs/cryptanalysis/pull/1875).
The recorded H3 canonical-lift digit measurements used curves with
`p ∤ #E(F_p)`; their data and decision boundary are unchanged. This theorem
addresses additive maps on the fixed subgroup. The digit and sublevel-set
questions remain the separate C3/H3 experiment.

### T4 (pairing transfer is out of range for both instances).
*MOV/Frey–Rück maps the DLP into `F_{p^k}^*` (resp. `F_{2^{nk}}^*`) with `k` the embedding
degree; for ECCp-131, `k = N-1 ≈ 2^131`, and for ECC2-131, `k = (N-1)/29 ≈ 2^126`.*
The target field has `≈ 2^{131·k}` elements; its DLP is astronomically harder than rho. ∎
(Computed in `H*/verify_instance.py`.) No generalized-Jacobian or `G_a`/`G_m`-extension
transfer improves this, because those transfers factor through the same Tate pairing or the
additive/multiplicative group whose DLP is either trivial (giving no information) or the
finite-field DLP already priced out.

---

## Part II — Open, stated as conjectures with explicit counterexamples

### C0 (the organizing conjecture). As stated in `README.md`.
Heuristic: a decomposition algorithm for `E(F_p)` needs a notion of "size" `s` on points with
(i) `s` bounded on a set of size `≈ √p` (the factor base), (ii) `s` subadditive under `+` so
that relations compose, (iii) a membership test "is `s(P)` small" cheaper than `√p`. The only
natural `s` on `E` is the Néron–Tate height `ĥ`, which is *quadratic* (`ĥ(mP) = m^2 ĥ(P)`),
so (ii) fails: the sum of two small points is generically large and does not recurse. Over
`F_p` the height is replaced by any coordinate functional, and T1 says no algebraic functional
has a size-`p^{1/m}` sublevel set. 
**Counterexample = attack:** exhibit `s` and a factor base meeting (i)–(iii) on ECCp-131.
**Probe (H0):** measure the additive-closure defect of the Semaev interval base
`F = {P : x(P) < p^{1/2}}` — the sumset growth and the non-recursiveness of factorization —
to quantify that (ii) fails at the predicted rate.

### C1 (symmetrized small roots). 
Semaev decomposition over `F_p` seeks `x_1, …, x_m ∈ [0, p^{1/m})` with `S_{m+1}(x_1,…,x_m,X(R))
= 0`. Coppersmith finds roots of a polynomial of degree `d` in a box of volume `< p^{1/d}`;
the naive box has volume `p^{(m-1)/m}` at `m` variables, far above threshold. 
Conjecture: symmetrizing by the rational 2-/4-torsion action (FGHR), which lowers the degree
of `S_{m+1}` in the elementary symmetric variables by a factor up to `4`, does **not** lower
the Coppersmith root-box threshold enough to make relation cost `< 2^{21}` on a curve of
cryptographic size.
**Counterexample = attack:** a symmetry-aware lattice that recovers box-`p^{1/3+ε}` roots of
the symmetrized `S_4`. **Falsifier (H1):** measure the actual recovered root-size vs. the
generic Coppersmith bound at `p ∈ {2^16…2^28}` on a 4-torsion curve.

### C2 (Hermitian relation matrix is generic).
The degree-`p+1` Hermitian surface `H(3,p^2)/F_{p^2}` carries `(p+1)(p^3+1)` lines (the count `(p^3+1)(p^2+1)` is the number of points) forming
a rank-`O(p^3)` Néron–Severi sublattice with `PGU(4,p)` acting. Restricting lines to a
Legendre fiber `E_λ(F_{p^2})` gives relations not implied by the `F_p` group law.
Conjecture: the resulting relation matrix has generic rank (full minus the tautological
pencil/hyperplane relations) and its spectrum matches the incidence spectrum of the
associated generalized quadrangle `GQ(p, p^2)` — i.e. no *extra* low-rank structure that a
sub-`O(p^2)` solve could exploit.
**Counterexample = attack:** a rank deficit growing with `p`, or a spectral gap giving an
implicit/fast solve. **Falsifier (H2):** build the matrix at `p ∈ {7,11,13,31}`, compare
rank and singular-value spectrum to a random matrix of the same density and to the known
`GQ(p,p^2)` eigenvalues.

### C3 (canonical-lift digits carry no size).
The lift `P ↦ P̃ ∈ Ẽ(Z_p)^{φ̃=1}` is a group isomorphism; writing `P̃ = (x_0 + x_1 p + x_2
p^2 + …, …)` gives p-adic digit vectors computable by Satoh/Harley Newton iteration. By T3 the
digits of `P̃+Q̃` are a fixed polynomial (the formal group law) in those of `P̃, Q̃` — that is
just the group law. 
Conjecture: no `F_p`-linear (or low-degree) functional `L` of the digit vector of `P̃` has a
small-valued sublevel set that is closed under `+`; equivalently the map `P ↦ (x_1(P̃) mod p)`
is not a homomorphism and its fibers are not a usable factor base.
**Counterexample = attack:** such an `L` (a hidden character/size on the digits).
**Falsifier (H3):** at `p ∈ {2^12,2^16,2^20}` compute `P̃ mod p^3` for many `P`, test every
`F_p`-linear functional of `(x_1, x_2)` for the homomorphism property and for closure of its
sublevel sets, and measure mutual information between digit functionals and `log_G P`.

### C4 (FPPR first-fall degree grows with n).
The one descent that exists on ECC2-131 is Faugère–Perret–Petit–Renault: write the Semaev
decomposition over an `F_2`-linear subspace `V ⊂ F_{2^n}` of dimension `n/m`, expand into `n`
Boolean equations by the `x^2`-linearity of Frobenius, and solve by Gröbner. No subfield is
needed, so prime `n` is no obstruction. Cost is governed by the first-fall / solving degree
`d_reg(n)`.
Conjecture: `d_reg(n)` grows (at least like the Bardet–Faugère–Salvy bound for the relevant
system), so the `m=3` decomposition cost at `n=131` is `≫ 2^{21}` group operations, above the
rho budget `2^{65}` for the whole attack.
**Counterexample = attack:** `d_reg(n)` bounded (constant) as `n → 131`, giving a
subexponential relation step. **Falsifier (H4):** measure `d_reg` and solve cost of one
symmetrized `m=3` decomposition at prime `n ∈ {17,23,31,41,53}`, fit the trend, extrapolate
to `131`, and compare to `2^{21}`.

### C5 (Ulmer f=2 kernel needs collection).
Ulmer's Legendre curve over `F_p(u)` at `d = p^f+1` has explicit rank-`p^f` Mordell–Weil
sections specializing into `E_λ(F_{p^{2f}})`; the kernel of specialization is a sublattice of
`Z^{p^f}` of index `≤ p^{2f}`.
Conjecture: for `f ≥ 2` the geometric (intersection-form) relations among sections are
tautological in the fiber subgroup — as the `f=1` collapse showed, they reproduce Gaudry's
`F_{p^2}` index calculus — so computing the kernel still costs relation collection at the rho
rate of the target group.
**Counterexample = attack:** the kernel sublattice computed from the Hermitian-surface
geometry at corank `o(p^f)` and negligible cost, giving sub-rho relations.
**Falsifier (H5):** at `p ∈ {7,11}`, `f=2`, build the sections, compute the true
specialization kernel, and check whether the geometric relations span it or only Ulmer's
two known relations.

---

## Scoring

Every experiment reports its gap to the matched rho as a distance, not a pass/fail (per
`CLAUDE.md` success criterion). A Win on any (C) is extraordinary and triggers the full
validation gate (`docs/VALIDATION_GATE.md`: replay certificate, validator report, matched-rho
band) before any claim leaves this directory. The expected outcome is a measured Kill on each,
which is still a result: it turns a hand-waved "dead" into a quantified wall with a scale
trend.


> Erratum (2026-10-23, found by the H2 runner): lines of H(3,p^2) number (p+1)(p^3+1); (p^3+1)(p^2+1) counts points. N N^T has spectrum {(p+1)(p^2+1), p^2+p, 0}. No faithful map from H to a Legendre fibre exists, so H2 tests a norm-circle surrogate.
