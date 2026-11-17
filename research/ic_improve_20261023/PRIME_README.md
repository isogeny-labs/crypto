# Prime-field twin of the Z[tau] multiplier base: `cm-multiplier`

**Date.** 2026-10-23. **Status.** Measured in `ic bench` (operation-counted `S =
total group-addition equivalents / sqrt(r)`, every relation row and the
recovered logarithm verified: all `correct = yes`). Toy sizes only (r about 2^33..2^38).

## What changed

New factor base `cm-multiplier:size=K[,norm=N][,endo=i]`
(`src/cryptanalysis/ic_framework/plugins.rs`, registered in `src/bin/ic/bench.rs`).
For each of `K` smallest-abscissa seeds `P` it adds, for every `alpha = a + b*phi`
with `a^2 + t*a*b + d*b^2 <= N` (`phi^2 - t*phi + d = 0`), the point
`alpha(P) = [a]P + [b]phi(P)` on `P`'s column with weight `a + b*lambda`. `phi` is
the curve's free automorphism (`j = 0`: zeta_3, `j = 1728`: i) when it has one, else
its degree-2/3 CM endomorphism from the Velu constructors in `glv_invariant_base`
(`D = -7, -8, -11`, evaluation charged as one addition per image). `norm = 0` is the
plain seeds-only control. Unit test: every point is `[coef]` times its column seed.

Run (the `ic` binary is built from this branch):

    ic bench --bits 36 --family j0 --seed 5 \
      --factor-base cm-multiplier:size=6,norm=64 \
      --oracle mitm:negation_folded=1,m=3 --targets walk --repeats 4

## Measurements (mean S over 4 targets; 3-summand `mitm`, negation folded)

Best plain base = best over a size scan of the existing incumbent for that
family (`glv-orbit` on j0 / j1728, which folds by the automorphism group;
`prime-abscissa`-equivalent `cm-multiplier:norm=0` on D7 / D8).

| curve | bits | best incumbent S (points) | best `cm-multiplier` S (cols, norm, points) | ratio |
|---|---:|---:|---:|---:|
| j = 0 | 36 | 106 (`glv-orbit` 800 seeds, 4,800) | 9.5 (6, 64, 1,440) | 11x |
| j = 0 | 40 | 208 (`glv-orbit` 1,500, 9,000) | 10.2 (10, 96, 3,480) | 20x |
| j = 1728 | 36 | 118 (`glv-orbit` 1,200, 4,800) | 8.5 (8, 64, 1,568) | 14x |
| D = -7 | 36 | 108 (plain 2,520 cols, 5,040) | 9.4 (10, 64, 1,580) | 11x |
| D = -7 | 40 | 185 (plain 6,000, 12,000) | 11.8 (20, 64, 3,160) | 16x |
| D = -8 | 36 | 107 (plain 2,520, 5,040) | 10.8 (10, 64, 1,380) | 10x |

At equal points (D = -7, 36 bit): 1,260 points 258 -> 19.4; 2,520 points 139 -> 20.2;
5,040 points 108 -> 58.

## What this is not

* A constant factor against the repo's own table-based `mitm` arm, **not against a
  published index calculus and not against rho.** The framework's rho reference at
  these sizes is S about 1 to 1.4, so the best row is still about 7 to 12 times rho;
  the incumbents were about 80 to 200 times. As `K` goes to 1 the base is a
  baby-step/giant-step table over `Z[phi]`-multiples (the lead workflow priced the
  integer-multiple version at its BSGS optimum, about 0.825 rho): the gain is the
  pipeline closing part of its distance to that generic bound.
* Only for curves with a small CM endomorphism or automorphism over `F_p`: `j = 0`
  and `j = 1728` curves (secp256k1 has `j = 0`) and `D = -7, -8, -11`. Not for
  generic prime curves such as P-256.
* Toy sizes (instance generator is limited to 10..40 bit subgroups; 44 bit
  failed to generate). No scaling claim to cryptographic sizes: the exponent is
  unchanged (the Corrigan-Gibbs-Kogan bound still applies to this table pipeline).
