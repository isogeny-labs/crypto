# Retained local Lean attempts

The final checked module is `../../lean-adaptive/AdaptiveS3.lean`. The exact
compiler output of each intermediate build is retained in `lean-build-02.log`
through `lean-build-07.log`. These are local development attempts, not accepted
proofs and not separate research results. Whole-file source snapshots were
not captured for every edit, so the records below preserve the decisive
failed tactic forms and error goals rather than claiming reconstructible full
intermediate modules.

| Build | Attempt and observed failure | Correction |
| --- | --- | --- |
| 02 | `mainDual_rows` unfolded `hm=41*x*y+39*(x+y)+27` and ended with `decide +kernel`; coefficients of `C 41*x*y` stayed symbolic. | Rewrite `hm` as a sum of the repository's monomial `term` constructors before using `coeff_monomial`. |
| 03 | The twist positive certificate used `linear_combination 22*h2-22*h0` and analogous lines; the goals retained multiples of43 such as `-(d*43)-a*1419-b*688=0`. | Add the actual `ZMod 43` characteristic equation, then include its exact multiplier in each linear combination. |
| 04 | The corrected coefficient proof built; its successful log is retained as a checkpoint. | Add an exact three-root theorem. |
| 05 | The first root theorem used `decide at huv` after setting `u=v=17`; this failed to close the impossible fourth grid point. | Derive `huv : (68:k)=0` and explicitly contradict `68≠0`. |
| 06 | `norm_num at huv` alone left the `68=0` goal unsolved. | Use `exact (show (68:k) ≠ 0 by decide) huv`. |
| 07 | The complete module built, including the three-root and quadratic vanishing-space theorems. | Run the unmodified scoped repository proof gate, retained separately. |

No `sorry` or trusted native evaluation is present in the accepted module.
