# Weighted-Proj relation: checked homogeneity rung

The pinned Lean source `Common22WeightedProjHead.lean` proves two statements
for a field `K`, weights `(1,3,1)`, and arbitrary coefficients `a3 a2 a1 a0`:

1. `equation_weighted_homogeneous`: the polynomial
   `Y² - X⁵Z - a3 X³Z³ - a2 X²Z⁴ - a1 XZ⁵ - a0 Z⁶`
   is weighted homogeneous of degree six.
2. `relationIdeal_homogeneous`: its principal ideal is homogeneous for the
   weighted homogeneous submodule grading.

This supersedes only the *compiler status* of the earlier bounded API attempt
in `WEIGHTED_PROJ_HEAD.md`. That file accurately records its own failed
commands and missing quotient-grading API search. The two current theorems do
not construct the graded quotient, a `Proj` chart isomorphism, a projective
completion, smoothness, or the `Pic⁰` transfer.

## Reproduction and receipts

The pinned toolchain is `leanprover/lean4:v4.34.1` (Lean commit
`5045d0056413266e57c625dcd7c365b10e377c52`); pinned Mathlib revision is
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The cached OLeans came from
`/dev/shm/crypto-pr2304-lake-packages`; the already-existing toolchain at
`/dev/shm/lean434-toolchain-copy` reported the same Lean version, and its
`bin/lean` SHA-256 matched the source toolchain executable:
`e8baaa71855a616dc351028f3ad2200051b0671f423a1696a100e809302d5550`.
No new Lake build was made.

Exact final command, including the shell's package-path construction:

```bash
cp research/projective_law_followup_20261022/formal/Common22WeightedProjHead.lean /dev/shm/common22-weighted-proj-final.lean
cmp research/projective_law_followup_20261022/formal/Common22WeightedProjHead.lean /dev/shm/common22-weighted-proj-final.lean
proj_lean_path=''
for package_dir in /dev/shm/crypto-pr2304-lake-packages/*; do
  if test -d "$package_dir/.lake/build/lib/lean"; then
    if test -z "$proj_lean_path"; then proj_lean_path="$package_dir/.lake/build/lib/lean"
    else proj_lean_path="$proj_lean_path:$package_dir/.lake/build/lib/lean"; fi
  fi
done
TMPDIR=/dev/shm LEAN_SYSROOT=/dev/shm/lean434-toolchain-copy LEAN_PATH="$proj_lean_path" \
  timeout --signal=TERM --kill-after=10s 180s \
  /dev/shm/lean434-toolchain-copy/bin/lean /dev/shm/common22-weighted-proj-final.lean
```

The compiler exited **0**. `weighted-proj-head-final-build.log` preserves the
complete output. Both `#print axioms` entries list only `propext`,
`Classical.choice`, and `Quot.sound`; neither lists `sorryAx`. The source
SHA-256 was `81e62fdb82407288a4e737719a8e86bdd7e2d85ba729a6fd5a495bbecd31c98f`;
the compiler log SHA-256 was
`bd77149b9973cc97a31044890728a8c19a7531953c9416258f9ba4cd471f73c5`.

Before that successful run, a separate attempt to copy the full pinned
toolchain byte-for-byte from the NFS mount into
`/dev/shm/lean4-v4.34.1-local` hit its `timeout --signal=TERM --kill-after=10s
240s` deadline and exited **124**. Its partial destination contained
1,616,872,096 bytes (`du -sb`), with no `bin/lean`; it was not used. The exact
copy command and empty output are preserved in `weighted-proj-head-copy.txt`
and `weighted-proj-head-copy.log`. The pre-existing matching executable was
found by a later read-only `find /dev/shm -maxdepth 4 -type f -name lean`.

The remaining concrete bridge obligations are (a) a grading on the quotient
by this homogeneous ideal, (b) an explicit isomorphism between the degree-zero
part after inverting `Z` and `K[x,y]/(y²-f(x))`, (c) the analogous infinity
chart and agreement on overlap, and (d) projectivity, smoothness, and `Pic⁰`
compatibility with the original curve. The availability of
`Proj.basicOpenIsoSpec` addresses only the generic localization-to-`Spec`
step after (a); it does not discharge (b).

## Checked chart-equation substitutions

`Common22WeightedProjCharts.lean` imports the exact weighted sextic above and
kernel-checks two additional identities over any field and arbitrary
`a3,a2,a1,a0`. With `(X,Y,Z)=(x,y,1)`, its equation evaluates to
`y²-(x⁵+a3*x³+a2*x²+a1*x+a0)`. With `(X,Y,Z)=(1,w,z)`, it evaluates to
`w²-z*(1+a3*z²+a2*z³+a1*z⁴+a0*z⁵)`. These are the two equations used by
the separately constructed affine and infinity charts. They check the
forward coordinate substitutions only: no quotient/localization
isomorphism, chart cover, smoothness, projectivity, or `Pic⁰` transfer follows.

The pinned Lean v4.34.1/Mathlib environment and prebuilt matching toolchain
above were reused. From the repository root, the exact commands were:

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
  -o /dev/shm/Common22WeightedProjHead.olean \
  research/projective_law_followup_20261022/formal/Common22WeightedProjHead.lean
TMPDIR=/dev/shm LEAN_SYSROOT=/dev/shm/lean434-toolchain-copy \
  LEAN_PATH="$proj_lean_path" timeout 180s \
  /dev/shm/lean434-toolchain-copy/bin/lean \
  research/projective_law_followup_20261022/formal/Common22WeightedProjCharts.lean
```

The chart compiler exited **0**; the exact
output is retained in `weighted-proj-charts-build.log`. Both theorem audits
report only `propext`, `Classical.choice`, and `Quot.sound`, with no `sorryAx`.
The chart source SHA-256 is
`a3a8f4adc679534f79de247934e89ddb2a4e423729e36e8ee319ca0cff38638b`;
the log SHA-256 is
`3e7d85fc137a0b47fa1cb5844e2776617c870f00d1076de78c22d282c8268f01`.
The updated CSV, graph, and PDF distinguish these two checked equations from
the still-open chart-isomorphism obligations.
