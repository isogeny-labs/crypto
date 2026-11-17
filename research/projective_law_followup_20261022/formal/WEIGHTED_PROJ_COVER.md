# Weighted-Proj chart cover: checked prime-containment prerequisite

The [PDF](weighted-proj-cover.pdf) embeds the [dependency graph](weighted-proj-cover.svg),
and the [standardized CSV](weighted-proj-cover.csv) lists three checked and
five open obligations. This packet changes no operation count or cost ratio.

For any field `K`, arbitrary coefficients `a3,a2,a1,a0`, and the proposed
weighted sextic `F`, pinned Lean proves:

1. `equation_factor`: `F = Y² - Z*tail`, with the explicit quintic tail in
   `Common22WeightedProjCover.lean`;
2. `y_mem_of_z_mem`: every prime ideal of the ambient polynomial ring that
   contains `F` and `Z` also contains `Y`;
3. `irrelevant_le_of_x_z_mem`: if that prime also contains `X`, it contains
   the coordinate ideal `(X,Y,Z)`.

The second statement follows from the first because `Y²` is in the prime;
the third is ideal-span containment. They are prerequisites for a `D₊(X)`,
`D₊(Z)` cover, **not** a theorem about `Proj`: the quotient grading, its
irrelevant ideal, and the relevant homogeneous-prime correspondence have not
been constructed here. The chart-ring isomorphisms, smooth projective
completion, and `Pic⁰` transfer are also open.

## Reproduction

The source uses Lean `v4.34.1` and Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The pre-existing matching
toolchain and Mathlib OLeans were used; no toolchain rebuild occurred. From
the repository root, after compiling the unchanged
`Common22WeightedProjHead.lean` to
`/dev/shm/Common22WeightedProjHead.olean` as recorded in
[the head packet](WEIGHTED_PROJ_HEAD_VERIFIED.md), run:

```sh
proj_lean_path='/dev/shm'
for package_dir in /dev/shm/crypto-pr2304-lake-packages/*; do
  if test -d "$package_dir/.lake/build/lib/lean"; then
    proj_lean_path="$proj_lean_path:$package_dir/.lake/build/lib/lean"
  fi
done
TMPDIR=/dev/shm LEAN_SYSROOT=/dev/shm/lean434-toolchain-copy \
  LEAN_PATH="$proj_lean_path" timeout 180s \
  /dev/shm/lean434-toolchain-copy/bin/lean \
  research/projective_law_followup_20261022/formal/Common22WeightedProjCover.lean
```

The compiler exited **0**. All three `#print axioms` lines in
`weighted-proj-cover-build.log` list only `propext`, `Classical.choice`, and
`Quot.sound`, with no `sorryAx`. Source SHA-256:
`8fb14500cc661695aec938b933ef740048d82b47933b90ba7f2bcd20ca4f947f`.
Log SHA-256:
`02cb4467c3b4e64b6d76ebd1b20874bfad3d6c928b6f61fd4c1e95194a32ddbf`.

The native [identity certificate](weighted-proj-cover-identity.json),
`IDC1h2022ac86a412d974`, independently seals **only the polynomial
factorization** over `F_(2^61-1)` with 64 statement-derived evaluations and
64 extra checker-seed evaluations. It points to the separately checked Lean
source for the all-field theorem. The native issuer is
[`examples/weighted_proj_cover_identity.rs`](../../../examples/weighted_proj_cover_identity.rs);
the exact commands were:

```sh
CARGO_TARGET_DIR=/dev/shm/pr2304-merge-target TMPDIR=/dev/shm \
  cargo run --release --example weighted_proj_cover_identity -- issue \
  > research/projective_law_followup_20261022/formal/weighted-proj-cover-identity.json
/dev/shm/pr2304-merge-target/release/examples/weighted_proj_cover_identity \
  check research/projective_law_followup_20261022/formal/weighted-proj-cover-identity.json
```

The check printed `CHECKED IDC1h2022ac86a412d974`, retained in
`weighted-proj-cover-identity-check.log`. Certificate SHA-256:
`bc6b0b67d19e92335c0517ae03766e255a8a148dc1f97c2f29d8f6d4e1dd7902`.
The [Typst source](weighted-proj-cover.typ) embeds the same SVG as the PDF;
`pdftoppm` opened and rendered its one page for visual inspection, and
`pdftotext` extracted the status labels and limitations.
