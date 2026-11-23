# CT13 continuation — naive cubic Richelot map does not descend coefficientwise

2026-11-11. Scope: exact finite-field **development** checks on the public
F17 example in `CT13_F17.md`. [pcc:unverified]. No normalized Richelot
homomorphism, dual map, or DLP benchmark was executed or certified.

## Literature / context checked before modifying the tests

- Existing `CT13_F17.md`, `QUEUE.md`, `native/tests/ct13_f17.rs`,
  `docs/session-logs/2026-11-11.md` and sibling CT11/CT12 results.
- Kunzweiler, *Efficient computation of (2^n,2^n)-isogenies*, 2024,
  Proposition 7 (codomain) and Proposition 8 (Richelot correspondence):
  <https://doi.org/10.1007/s10623-024-01366-1>.
  These formulas are stated with labeled quadratic factors over their
  **coefficient field**; the point formula does not by itself supply
  descent when the three factors are cyclically permuted by Frobenius.
- Magma Richelot handbook, kernel modeled by a quadratic over a cubic
  étale algebra, and the separate rational *double Richelot* class with
  abstract kernel (2,2,4):
  <https://magma.maths.usyd.edu.au/magma/handbook/text/1615>.
- Bruin–Doerksen's non-rational-kernel treatment (2011):
  <https://doi.org/10.4153/CJM-2011-039-3>.
- The shared research-KB MCP was not registered and the wrapper / encrypted
  lead atlas are unavailable in this isolated execution environment.
  No absence/novelty claim is made.

## A concrete descent obstruction for the *raw support formula*

Keep the source `C/F17: y²=X⁶+2X²+2` with `#J(F17)=285`.
Over `L=F17[t]/(t³+6t²+t+7)`, write
`beta_i=t^(17^i)` and
`G_i(X)=(X-beta_i)(X+beta_(i+k mod 3))`,
for k=1 or 2. Define `Delta=det(G_i)` and the normalized brackets
`H_i(Z)=(G_j'G_l-G_jG_l')/Delta`, indices cyclic.

The classical labeled two-term correspondence (Kunzweiler Proposition 8)
uses the quadratic
`A_X(Z)=G_0(X)H_0(Z)+G_1(X)H_1(Z)`,
and a second equation to produce the target ordinates.

For `X=0`, the source has two F17 points `(0, +/-6)`.
In the ordered basis (1,t,t²) of L, the **monic** form of A_0 is

| kernel k | constant coefficient | coefficient of Z | Z² coefficient |
|---|---|---|---|
| 1 | (9,14,4) | (13,6,11) | (1,0,0) |
| 2 | (14,8,11) | (0,9,11) | (1,0,0) |

Neither quadratic is fixed by the 17th-power Frobenius, even after monic
normalization. This is not merely a common scalar factor in L.

A stronger bounded check enumerates all seven affine abscissae of
`C(F17)`: `0,3,4,8,9,13,14`. Each supports two rational points; the
two coefficient ratios of A_X fail the Frobenius fixedness check for
**all 14/14 affine points**, for **both** k=1 and k=2 (28 cases total).
The inversion-free test is
`a_i^17 a_2 != a_i a_2^17` for i=0,1.
Two independently written local development oracles (Python finite
field and JavaScript via cross-multiplication without inversion) agreed.
The new Rust regression encodes the same test; Rust has **not** been
compiled on this runner.

**Interpretation:** Although `prod_i H_i` has coefficients in F17,
the labeled two-term formula over F17³ does **not** directly produce a
rational closed degree-two divisor on the presented F17 target. A valid
descent needs an explicit Frobenius-equivariant identification of
the conjugate correspondences / target models and correct origin/twist
normalization. This **does not** disprove the existence of the
F17-defined quotient isogeny implied by a Frobenius-stable kernel.
Matching zeta functions does not prove that the displayed target
coordinates realize the descended homomorphism.

## A separate exact 2-power obstruction (not a universal no-go)

Since `#J(C)(F17)=285` is odd, `J(C)[2](F17)=0`.
The same holds for both reported 285-point target Jacobians.
Independently, a (3)(3) or (6) Frobenius permutation on six branch
roots fixes no unordered pair, confirming no nonzero rational
geometric 2-torsion classes.

For any F17-defined finite kernel K whose abstract structure is
`(Z/2)² x Z/4`, its characteristic subgroup `2K` has order two.
Frobenius preserves `2K`, hence fixes its unique nonzero point,
contradicting `J[2](F17)=0`. Therefore **the specific nondual
double-Richelot kernel type (2,2,4) is impossible over F17** on
these models. The identical argument excludes F17-rational cyclic
4-kernels. It says **nothing** about (4,4) or other 2-power
kernels with Frobenius acting freely on their 2-torsion,
extension-field intermediary maps, or odd-degree isogenies.
This is a standard characteristic-subgroup argument applied to
this fixture, not a novel theorem.

## Follow-up and acceptance test

1. Compute the Frobenius conjugates of the labeled correspondence and
   determine the explicit target-isomorphism/cocycle data. Prove
   that the resulting map is defined on F17-rational Jacobian classes
   (or show the proposed codomain model requires a twist).
2. Construct the normalized map and its dual on balanced even-sextic
   classes. Check all 285 classes, `dual ∘ Phi = [2]`, nonsplit and
   ramified divisors, origins, and the 19 source scalars. Do **not**
   infer correspondence correctness from the polynomial gate alone.
3. Run the touched Rust tests with `cargo test --release --offline
   --locked --manifest-path
   research/chained_jacobian_transfers_20261110/native/Cargo.toml
   --test ct13_f17`; perform exact-head CI/PCC follow-up and only then
   register certified claims.
4. Treat a degree-4 Richelot map as an *operation-cost* candidate,
   not a reduction in the 19-subgroup order; retain direct elliptic
   baseline and full-cost construction/transport/recovery arms.

No cloud run, spending, signed PCC verdict or ECDLP speedup is claimed.
