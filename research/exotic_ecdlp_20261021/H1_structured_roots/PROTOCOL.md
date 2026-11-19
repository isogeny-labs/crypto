# H1 — Symmetrized Semaev small-roots over F_p (does 4-torsion symmetry beat the box bound)

Pre-registration. Tests C1.

| Field | Value |
|---|---|
| Object | `E/F_p` with full rational 4-torsion (so `E[2] ⊂ E(F_p)` and a rational 4-torsion point), `p ∈ {2^16, 2^20, 2^24, 2^28}` (nearest prime; curve built to have `4 | gcd`-structure, i.e. `E(F_p) ⊃ Z/4`). Third Semaev polynomial `S_3(X_1,X_2,X_3)` and the 3-sum relation `S_4` via resultant, i.e. decompose a target `R` as `P_1+P_2+P_3` with `x(P_i)` in a box of side `B = ⌈p^{1/3}⌉`. |
| Claim (C1 counterexample) | A Coppersmith/lattice method that bakes in the 2-/4-torsion symmetry recovers roots in a box of side `p^{1/3+ε}` (ε>0 growing), i.e. reaches strictly past the generic Coppersmith threshold, giving relation cost `< 2^{21}`. |
| Observable | (a) **Count control**: number of box-`B` decompositions of random `R`, vs the heuristic `≈ 1/3!` per expected solution — confirms the relation model. (b) **Reach**: the largest root-box side the symmetric lattice actually solves, for the symmetrized bivariate (elementary-symmetric coords `e_1=x_1+x_2`, `e_2=x_1x_2` with the third point fixed), vs the generic Coppersmith bound `p^{1/deg}`. (c) relation cost in field-ops per found decomposition. |
| Null (rho) | Symmetric lattice reach = generic Coppersmith reach (no gain); box enumeration cost `≈ B = p^{1/3}` per relation (`2^{43}` at 131-bit → `≫ 2^{21}`). |
| Win | Reach side grows faster than generic with `p` (fit exponent of recovered-box-side vs `log p`, compare to generic `1/deg`); extrapolated relation cost `< 2^{21}`. |
| Kill | Reach matches generic Coppersmith at every `p`; symmetry gives only the known constant `×4` degree drop, not a threshold change. |
| Scale | The recovered-box exponent vs `log p` is the scale test; must exceed `1/3` and grow. |

## Method
1. Build `S_3` (Semaev 3rd): `S_3 = (X_1-X_2)^2 X_3^2 - 2((X_1+X_2)(X_1X_2+a)+2b)X_3 + ((X_1X_2-a)^2 - 4b(X_1+X_2))` for `y^2=x^3+ax+b`. Verify on random collinear triples.
2. Fix target `R`; substitute `X_3 = x(R)`. Get bivariate `f(X_1,X_2)=S_3(X_1,X_2,x(R))` of total degree 4 (deg 2 each). Symmetrize: rewrite in `e_1,e_2`; the 2-/4-torsion translation `P_i ↦ P_i + T` (T a 2-torsion point: `x ↦ known Möbius`) induces a group action; quotient to lower the effective degree (FGHR). Record the symmetrized degree.
3. **Count control**: for 200 random `R`, exhaustively enumerate box-`B` solutions at the two smallest `p` (feasible), confirm the count ≈ model.
4. **Reach**: run Sage `f.small_roots` / a hand-built Coppersmith lattice (`X,Y` bounds swept upward) on both the raw `f` and the symmetrized `g(e_1,e_2)`; record the largest bound `β` (as `p^{β}`) for which a true root is recovered. Do this for raw and symmetrized; the comparison is the result. Also record the generic predicted `β = 1/deg`.
5. Output JSON per `p`: symmetrized degree, count-control ratio, raw-reach `β_raw`, symmetric-reach `β_sym`, generic `β_gen`, median lattice time, implied relation cost at 131-bit by extrapolation.

## Decision rule
Kill if `β_sym ≤ β_gen + o(1)` at all `p` (no threshold gain beyond the degree-drop constant). Win if `β_sym − 1/3` is positive and increases with `log p`.

## Isolab
One job, `cpu 4 mem 8Gi seconds 3600`. Exhaustive enumeration only at `p ≤ 2^20`.
