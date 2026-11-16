# A checked exclusion for the dense two-summand presentation

This is an elementary specialization of the usual highest-degree-form
analysis, not a claimed novel factor-base improvement.

Let p>3 be prime, E: y²=x³+Ax+B nonsingular, and V⊂Fp have Bv≥2 distinct
elements. Use the exact membership polynomial
`F_V(T)=∏_{v∈V}(T−v)`. For a fixed target abscissa r, the complete dense
two-summand system is

`F_V(x)=0, F_V(y)=0, S3(x,y,r)=0`.

The highest total-degree forms of its original generators are
`x^Bv`, `y^Bv`, and `x²y²`. The last assertion follows directly from the
standard short-Weierstrass S3 formula: its degree-four coefficient is 1;
the target r and curve coefficients occur only in lower degrees.

Define d_reg here as the first degree at which the quotient by the ideal
of these highest-degree forms has zero homogeneous component. Its value is

`d_reg = Bv + 1`.

**Written proof.** A monomial x^i*y^j is outside
`J=(x^Bv,y^Bv,x²y²)` exactly when i<Bv, j<Bv, and either i<2 or j<2.
Every such monomial has degree at most Bv. The monomial x^(Bv−1)*y
has degree Bv and is outside J. At every degree at most Bv there is also
an outside monomial: x^d for d<Bv, and the displayed witness at d=Bv.
Consequently Bv+1 is the first empty homogeneous degree. This uses the
standard monomial basis of a monomial ideal; no genericity assumption is used.

Thus any two size-matched V give the same d_reg in this precise dense
presentation. Curve changes and target selection cannot alter that statement.
This does not say their affine Gröbner computations take equal work or reach
equal step degree. It does not constrain every possible structured presentation.
It does not apply as stated to more summands, unequal slot-base cardinalities,
or a different coordinate model without checking the corresponding top ideal.

## Formal proof map

`proof/TopDegree.lean` defines the monomial generator-divisibility predicate
`Covered b i j := b≤i ∨ b≤j ∨ (2≤i ∧ 2≤j)`.

| Written step | Lean declaration | Scope |
| --- | --- | --- |
| Every monomial above degree b is covered | `DensePairTop.all_above` | Kernel checked for arbitrary naturals |
| A degree-b survivor exists for b≥2 | `DensePairTop.survives_at_top` | Kernel checked |
| Every lower degree has a survivor | `DensePairTop.survives_below` | Kernel checked |
| First fully covered degree is b+1 | `DensePairTop.first_full_degree` | Kernel checked |
| S3 and membership highest forms; monomial ideal/quotient interpretation | Written argument above | Polynomial-ring bridge not formalized; the Lean theorem is not presented as that full bridge |

Verified with Lean 4.19.0 (`6caaee842e94`):

```sh
lean research/prime_factor_base_degree_20261031/proof/TopDegree.lean
```

All four `#print axioms` outputs list only combinations of `propext`,
`Classical.choice`, and `Quot.sound`. No `sorry`, `admit`, custom axiom,
or native evaluation is used. This proves an exclusion for the chosen metric;
it does not fulfill the user's requested positive novel degree reduction.
