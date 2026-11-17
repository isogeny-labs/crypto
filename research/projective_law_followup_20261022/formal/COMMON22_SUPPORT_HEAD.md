# Common 2+2 residual-support head

Schema: `projective-law-common22-support-head/v1`. The Lean theorem proves
that the `X⁶`, `X⁵`, and `X⁴` coefficients of `N²-u₁u₂U` are respectively
`0`, `B²`, and `0`, with `B,A,C,Q,Rbar` defined by the unit-weight common
2+2 formulas. It is an exact identity over any commutative ring. The
[build log](common22-support-head-build.log) records the kernel check.

The source checker compares the Rust `B,A,C,Q` polynomials and the factored
`Rbar` with the Lean definitions, then independently expands the three head
coefficients. All eight obligations match; sign mutations of `Q` in Rust and
Lean are rejected. The [certificate](common22-support-head.json) retains
every expanded term, the [CSV](common22-support-head.csv) has one row per
obligation, and the [graph](common22-support-head.svg) shows exact term counts.
The [PDF](COMMON22_SUPPORT_HEAD.pdf) is hash-bound in this directory's
artifact manifest; pull the directory with `tools/artifacts/artifacts.py`
before opening it in a fresh checkout.

This establishes the high-degree shape required for the residual-support
argument in the unit-weight branch. It does not prove the lower coefficients
of `N²-u₁u₂U=B²F`, the input curve congruences, coprime-support uniqueness,
arbitrary weighted normalization, exceptional branches, or the final
Jacobian-class statement. The 1,260-target cost ratios are unchanged.
