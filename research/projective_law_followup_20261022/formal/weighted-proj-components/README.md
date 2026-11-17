# Weighted-Proj componentwise quotient prerequisite (bounded proof packet)

This packet records two new checked Lean lemmas for the weight `(1,3,1)` sextic in `Common22WeightedProjHead.lean`. It is a prerequisite for constructing a graded quotient, not a completed graded quotient or a projective chart. The theorem statements hold over an arbitrary field `K` and all coefficients `a3 a2 a1 a0 : K`. The [PDF](REPORT.pdf) embeds the [dependency graph](componentwise-dependencies.svg); the [CSV](componentwise-status.csv) keeps checked and open obligations in a standard format.

| Theorem | Checked statement | What it does not establish |
| --- | --- | --- |
| `weightedRing_finite_homogeneous_lift` | Every class in the ordinary quotient `Poly K ⧸ (equation)` can be represented as a finite sum of images of weighted-homogeneous polynomials of their stated degrees. | An internal graded decomposition or multiplication law on the quotient. |
| `weightedRing_eq_iff_components` | Two polynomial representatives give the same quotient class iff the images of all corresponding weighted-homogeneous components agree. | An `Away` localization, a degree-zero chart, or a `Proj` isomorphism. |

The second proof applies mathlib's `MvPolynomial.mem_iff_weightedHomogeneousComponent_mem` to the previously proved homogeneous principal ideal. The first uses `weightedHomogeneousComponent_finsupp` and `sum_weightedHomogeneousComponent`. The precise next API boundary is to define degree pieces in `WeightedRing`, prove they decompose the quotient as a `GradedRing`, and then identify `HomogeneousLocalization.Away` at the image of `Z` with the existing ordinary `Z = 1` chart. Mathlib's `AlgebraicGeometry.Proj.basicOpenIsoSpec` can only be applied after that ring identification is supplied.

## Evidence and replay

- Source: [`WeightedProjDegreeZeroScratch.lean`](../WeightedProjDegreeZeroScratch.lean), SHA-256 `2026d342f368bd89bc60f16267edbdd8d6299c5401d6c625c539202aa3384377`.
- Compiler output: [`WeightedProjDegreeZeroScratch.log`](../WeightedProjDegreeZeroScratch.log), SHA-256 `d8ebceae0a02c6ce59ce3269cbb9cc7cb07ba1b8e4e9face104c67ad73414317`.
- Lean `4.34.1` (`5045d0056413266e57c625dcd7c365b10e377c52`), pinned mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`; compiler exit status 0.
- Both axiom audits contain only `propext`, `Classical.choice`, and `Quot.sound`, with no `sorryAx`.
- The imported `Common22WeightedProjHead`, `Common22WeightedProjQuotient`, and `Common22WeightedProjKernel` modules were compiled separately under the same pins into `/dev/shm/weighted-degreezero-agent`. Their source lives in `research/projective_law_followup_20261022/formal/`.

From the repository root, with the pinned toolchain and mathlib build at the recorded paths, replay with:

```sh
proj_lean_path='/dev/shm/weighted-degreezero-agent'
for package_dir in /dev/shm/crypto-pr2304-lake-packages/*; do
  if test -d "$package_dir/.lake/build/lib/lean"; then
    proj_lean_path="$proj_lean_path:$package_dir/.lake/build/lib/lean"
  fi
done
TMPDIR=/dev/shm LEAN_SYSROOT=/dev/shm/lean434-toolchain-copy \
  LEAN_PATH="$proj_lean_path" timeout 60s \
  /dev/shm/lean434-toolchain-copy/bin/lean \
  research/projective_law_followup_20261022/formal/WeightedProjDegreeZeroScratch.lean
```

The CSV records the checked/open proof boundary. The SVG depicts dependencies only; its arrows do not assert new maps. The Typst source embeds that SVG into the PDF. No curve experiment or cost ratio is part of this packet.
