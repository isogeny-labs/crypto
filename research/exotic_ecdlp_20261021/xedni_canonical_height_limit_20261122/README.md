# Canonical-height limit on the fixed xedni curve

The checked theorem `fixed_curve_naive_height_function` constructs a real-valued
function `H` on the rational points of

\[E: y^2=x^3-47x^2+751x-3997.\]

For every point `P`, `H(P)` is the limit of `h(2^n P)/4^n`, where `h` is
Mathlib's elliptic `naiveHeight`; it satisfies `H(nP)=n²H(P)` for every
integer `n`, and `|H(P)-h(P)|≤D/3` for one curve-wide nonnegative
constant `D`.

## Proof and correspondence

`CanonicalLimit.lean` first proves that a uniform defect
`|h(2P)-4h(P)|≤D` gives a geometric tail bound. It constructs the limit,
proves the `D/3` comparison, and derives its doubling law by shifting the
convergent sequence. The error in the approximate parallelogram law vanishes
after division by `4^n`, giving an exact parallelogram law for `H`; a two-step
induction then proves `H(nP)=n²H(P)` for natural `n`, and evenness extends
this to every integer. Mathlib's
`WeierstrassCurve.Affine.approx_parallelogram_law` then
instantiates the argument for elliptic naive height. The final theorem uses
the exact rational Weierstrass model above and proves its discriminant is
nonzero.

| Mathematical result | Lean declaration | Hypotheses/status |
| --- | --- | --- |
| Geometric one-step bound | `normalized_step_bound` | Uniform defect, `D≥0`; checked |
| Limit and `D/3` comparison | `normalized_limit` | Uniform defect, `D≥0`; checked |
| Parallelogram to doubling defect | `doubling_defect_of_parallelogram` | Additive commutative group, approximate law; checked |
| Fourfold limit law | `limits_double` | Two stated sequence limits; checked |
| Exact parallelogram law for the limit | `limit_exact_parallelogram` | Approximate law and all pointwise limits; checked |
| Quadraticity for every natural multiple | `nsmul_square_of_exact_parallelogram` | Exact law; checked |
| Quadraticity for every integer multiple | `zsmul_square_of_exact_parallelogram` | Exact law; checked |
| Height function from the approximate law | `height_function_of_parallelogram` | Additive commutative group, approximate law; checked |
| Elliptic naive-height function | `elliptic_naive_height_function` | Elliptic curve over `ℚ`; checked |
| Fixed curve height function | `fixed_curve_naive_height_function` | The displayed rational curve; checked |

## Reproduction and boundary

Run `sh formal/build.sh`. The script compares the checked-in Lean toolchain
and Lake manifest against the pinned workspace before building the theorem
module. `formal/build-output.txt` retains the build and each `#print axioms`.
Every reported theorem uses only `propext`, `Classical.choice`, and
`Quot.sound`; the source contains no `sorry`, `admit`, or custom axiom.

The quadraticity extension was added after the initial limit protocol. It
strengthens the same theorem without changing the frozen curve or height
definition; the versioned amendment records this addition.

The theorem gives an existential curve-wide comparison constant. The
explicit numerical height bound needed for the proposed 130-bit cutoff is
still a separate obligation, as are a Mordell–Weil generator theorem and a
formal rational-to-finite reduction homomorphism. Consequently, this proof
does not by itself decide the full xedni calculus proposal. The next proof
step is an explicit comparison bound for this exact curve, followed by the
rank/generator and reduction bridges.
