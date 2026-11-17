# Explicit infinity-chart affine stalk

For any field `K` and coefficients `a3,a2,a1,a0`, the established chart ring is
`A=K[z,w]/(w²-z(1+a3 z²+a2 z³+a1 z⁴+a0 z⁵))`. Its origin ideal `m` is prime,
and the prior packet proves `A_m` is a DVR with uniformizer `w`.

The new theorem `Common22AffineStalk.affineStalkIso` identifies the structure
sheaf stalk of the **affine scheme** `Spec A` at the prime `m` with `A_m`. This
is an unconditional consequence of the explicit `originPoint` and the pinned
Mathlib theorem `AlgebraicGeometry.Spec.stalkIso`. Lean 4.34.1 checked it with
axioms `[propext, Classical.choice, Quot.sound]`.

| Requirement | Status | Exact evidence or missing step |
| --- | --- | --- |
| Explicit affine origin point | Checked | `Common22AffineStalk.originPoint` |
| Affine scheme stalk `≅ A_m` | Checked | `Common22AffineStalk.affineStalkIso` |
| Smooth projective curve completion | Open | No glued or graded projective model in this packet |
| Projective infinity stalk `≅ A_m` | Open | Needs a specific open immersion of `Spec A` into that completion and stalk transport |
| Principal divisor and `Pic⁰` transport | Open | Needs the projective model, rational-function identification, and divisor-class map |
| Independent Rust law semantic equivalence | Open | No proof added |

The theorem is about the affine chart only. It does not change any cohort,
operation-cost, or rho result. `affine-stalk.csv` gives machine-readable gate
status and `affine-stalk.svg` shows the dependency path. Replay with
`TMPDIR=/dev/shm lake env lean Common22AffineStalk.lean` from this directory.
