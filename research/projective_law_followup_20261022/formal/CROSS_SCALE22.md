# Weighted 2+2 cross-scaling boundary

Schema: `projective-law-cross-scale22/v1`. The restricted checker extracts
eight power intermediates and eight cross-scaled input expressions from the
common Rust 2+2 branch and the corresponding Lean `crossScale22` definition.
It then substitutes those inputs into seven Rust and Lean coefficient
expressions. It compares the expanded integer polynomials and rejects a
multiplication-to-addition mutation on each side and a composed-coefficient
subtraction mutation. All 23 expressions match. The
[certificate](cross-scale22.json) records exact source hashes and every
expanded term; the [CSV](cross-scale22.csv) and [graph](cross-scale22.svg)
show all 23 checked rows and the eight input weights respectively.
The two-page [PDF](CROSS_SCALE22.pdf) embeds the source-linked graph.

The Lean theorem `crossScale22_represents_affine` proves that, when the
weighted numerator equations hold, the eight cross-scaled inputs have common
weights `Z=z1*z2` and `W=w1*w2`: `q` has `Z²`, `r` has `Z⁴`, `s` has `Z³W`,
and `t` has `Z⁵W`. The theorem is a polynomial statement over any
commutative ring. Five further Lean theorems derive common weights
`B=Z⁸ B_aff`, `A=Z⁹W A_aff`, and `C=Z⁷W C_aff`, then prove that nonzero
`Z,W` preserve the `B≠0` and `C≠0` branch conditions. The [build
log](cross-scale22-build.log) records the kernel and axiom output. The exact
exponents and theorem names are in the [weights CSV](cross-scale22-weights.csv)
and [JSON](cross-scale22-weights.json). Rust-to-Lean runtime correspondence
and the returned divisor-class statement remain open.

This extends the [seven-coefficient slice](COEFFICIENT_SLICE.md), but does not
establish the full Rust control flow, exceptional branches, or the concrete
Mumford/Pic⁰ addition theorem. The 1,260-target cost ratios are unchanged.

Recheck from this directory with `python3 check_cross_scale22.py --check`,
`lake env lean CrossScale22.lean`, and `python3 render_cross_scale22.py`.
