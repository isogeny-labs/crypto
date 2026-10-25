# Weighted quotient: checked `Z = 1` specialization

The Lean file `Common22WeightedProjQuotient.lean` constructs an algebra map
from the ordinary quotient of `K[X,Y,Z]` by the weighted sextic to the affine
coordinate ring `K[x,y]/(y²-f(x))`, for any field `K` and coefficients
`a3,a2,a1,a0`. It proves that the sextic maps to zero, the descended map sends
`X` to `x`, `Y` to `y`, and `Z` to `1`, and the map is surjective. It also proves
that `Z-1` lies in its kernel. The definitions reproduce the existing affine
coordinate-ring presentation; they are local here so this proof can import the
already-built weighted head without recompiling the earlier long import chain.

This is an ordinary-ring specialization. It does **not** identify the kernel
as exactly `(Z-1)`, construct the quotient's weighted grading, identify the
degree-zero ring after localizing at `Z`, or prove a `Proj` chart isomorphism.
It changes no end-to-end operation or cost count.

## Replay

Pinned Lean: `leanprover/lean4:v4.34.1` with Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The matching, previously
verified toolchain at `/dev/shm/lean434-toolchain-copy` and prebuilt Mathlib
packages at `/dev/shm/crypto-pr2304-lake-packages` were used. From the repo
root:

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
  research/projective_law_followup_20261022/formal/Common22WeightedProjQuotient.lean
```

Exit status: **0**. The complete compiler output is
`weighted-proj-quotient-build.log`. All eight printed theorem audits have only
`propext`, `Classical.choice`, and `Quot.sound`; none lists `sorryAx`.

| Artifact | SHA-256 |
| --- | --- |
| `Common22WeightedProjQuotient.lean` | `877794f97a96f62c52b6ec9d1f68d6e2daaceb9e583cc8793fe5c2fe3b13696d` |
| `weighted-proj-quotient-build.log` | `e05bcc8da4d17e153757f88bb379428a7d7878142a3d0087724583f32cab93cb` |

The standardized status rows are in `weighted-proj-quotient.csv`; the graph
and PDF render the same checked/open boundary without a performance claim.
