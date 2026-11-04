# Affine-input cached subtraction

`ER_AFFINE_QUERY` specializes `target - cached_right` when a full right cache is
present. `Curve::affine_sub` uses EFD's Bernstein–Lange `mmadd-2007-bl` formula
with both finite inputs at Z=1. Infinity, inverse points, doubling and zero-y
doubling use explicit branches. The output remains Montgomery Jacobian; Point
and cache storage remain ordinary affine coordinates. Partial-cache and direct
sum paths retain the original mixed addition. All query kernels share the
residual helper so packed/unpacked and persistent winner resolution agree.

The nonexceptional formula uses4M+2S instead of7M+4S, excluding four coordinate
conversions, exceptional cases, normalization, memory traffic and lookup. This
is an established formula, not a new algebraic construction or GPU-time claim.
Primary source read2026-10-30:
https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-mmadd-2007-bl
Prior design: `research/explicit_relations_e2e_20261030/NEXT-AFFINE-QUERY.md`.

`affine-host-check` compares37,584 cases with both the old formula and an
independent ordinary-coordinate affine law using u128 intermediates. All point
pairs on two101-element fields include zero-y points; deterministic63-bit
cases add infinities, inverses, doublings and randomized pairs. The CUDA fixture
reuses these independently checked expectations and block normalization with
padded lanes. Query fixtures cover full-cache and forced partial fallback,
minimum duplicate witnesses, misses, reset/reprepare and both scheduling paths.

The matched experiment is `research/explicit_relations_affine_20261030`.
Opt-in remains off by default pending measured results. No coordinate-cache
representation change or target preconversion is combined with this factor.
