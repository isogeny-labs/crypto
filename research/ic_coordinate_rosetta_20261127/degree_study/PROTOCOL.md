# Matched coordinate degree study — frozen protocol

**Status:** frozen before native shared-F4 data

**Protocol SHA-256:** `e43dc87d2991eac64e96e64225fee97ac935e04d94ae06b52fd8e857adcc74d8`

**Instrument commit:** `e3fe5a216c81bfd75b1e5df768537a02bb1e03ce`

**Instrument binary SHA-256:** `8adb73af27429e1ca3491123a6f8f236a0d974844adf90b5f46f246b2483b8ba`

**Machine-readable source:** [`protocol.json`](protocol.json)

## Question and evidence boundary

On the fixed order-251 subgroup of

```text
v^2 = u^3 + 5*u^2 + u over F_1019,
```

compare native polynomial presentations while holding the represented
factor-base points and the relation `[9]G + [27]G = [36]G` fixed. The study
measures equation degree, a semi-regular Hilbert estimate, an affine
leading-ideal regularity index, completed native-F4 solving degree, selected
pair rows, matrix dimensions, and counted field multiplications.

This is one deterministic symbolic qualification. It does not measure local
wall time, Curve25519-scale behavior, complete relation collection, linear
algebra, recovery, rho, Coppersmith lifting, or an ECDLP speedup. Every output
remains `[pcc:unverified]` pending the repository evidence process; an external
CAS replay is required before solver-cost wording.

The user supplied results from an earlier standalone graded-lex Buchberger
prototype on this same toy fixture. Those values motivated the comparisons
but are not native `f4_fp` observations and do not count as A/A or independent
replay evidence. No `degree-study` invocation through the shared F4 engine was
made before this protocol was frozen.

## Fixed fixture and map

[`fixture.json`](fixture.json) freezes the source and destination EC1 aliases,
full representation UIDs, subgroup, generator, factor points, target, and map.
The degree-two isogeny has kernel `{O,(0,0)}`:

```text
U = u + 5 + 1/u
V = v * (1 - 1/u^2)
w = u + 1/u, so U = w + 5.
```

The instrument checks the homomorphism on all 251 subgroup scalars. Since
`gcd(2,251)=1`, the retained subgroup intersects the kernel trivially. The
independently constructed three-fibre norm must equal `h_3^4` exactly.

## Arms

| arm | variables / equations | isolated change |
| --- | --- | --- |
| Montgomery `u` | x-only `f_3` | reference |
| Montgomery `u` A/A | byte-identical equations | counted-state control |
| short-Weierstrass `x` | native x-only `f_3` | exact cubic translation |
| Montgomery affine | curve and chord equations with `lambda` | auxiliary presentation |
| quotient `w`, square-free | `h_3` on the isogenous destination | degree-two quotient |
| quotient `w`, raw | `h_3^4` | norm multiplicity only |
| twisted-Edwards `y` | native y-only `f_3` | model-native eliminated form |
| Edwards bilinear | `t_i=x_i*y_i`, `q=t_1*t_2` | quadratic auxiliary tree |
| Montgomery unsatisfiable | target `[37]G` | exhaustive negative control |

A Hessian arm is excluded. The fixed curve has order 1004, which is not
divisible by three; a rational Hessian fixture would change the curve and
confound a coordinate-only comparison.

## Degree conventions

- **Semi-regular estimate:** the first non-positive coefficient of
  `prod_i(1-z^d_i)/(1-z)^n`. It is an exact calculation from equation degrees,
  but only a model estimate.
- **Affine regularity index:** one plus the maximum standard-monomial degree
  of the completed grevlex affine leading ideal.
- **Productive F4 degree:** the highest F4 step that learns a new basis
  element or `1`.
- **Total solving degree:** the maximum of maximum input degree and productive
  F4 degree.
- **Raw processed degree:** also includes trailing certification steps whose
  pair rows reduce to zero.

None of the first two quantities is relabeled as a certified BFS degree of
regularity of the homogeneous top-form system.

## Controls and decision rule

All comparisons are invalid unless:

- the exact fixture, subgroup-map, norm, radical, and planted-solution checks
  pass;
- the A/A observations are identical;
- the exhaustive negative control is inconsistent;
- every F4 call completes with no timeout, resource limit, staircase stop, or
  pairs above the degree-32 bound; and
- the protocol, instrument, binary, fixture, and reducer hashes match.

The reducer reports exact per-arm tuples and signed differences. Equality
wording requires equality of every frozen degree-profile field. A lower-degree
statement requires a strict total-solving-degree inequality and must be shown
beside variable, equation, matrix, selected-pair-row, and field-operation
counts. No population equivalence or scaling inference is made from one
fixture.

## Execution

The repository checkpoint gate must pass before this command is invoked:

```bash
F4_FP_THREAD_OPS=1 \
RAYON_NUM_THREADS=1 \
OMP_NUM_THREADS=1 \
target/debug/ic_coordinate_rosetta degree-study \
  --protocol-sha256 e43dc87d2991eac64e96e64225fee97ac935e04d94ae06b52fd8e857adcc74d8 \
  --instrument-commit e3fe5a216c81bfd75b1e5df768537a02bb1e03ce \
  --max-degree 32 \
  --output research/ic_coordinate_rosetta_20261127/degree_study/raw/degree-study.json
```

The pre-data Rust reducer is
`src/bin/ic_coordinate_degree_report.rs`, frozen at SHA-256
`e51ce75636be764869644462fc476b766d39cab1793a0fc4b6a3bc12bf8ef5cf`.
It refuses failed controls or incomplete arms and emits `summary.json`,
`REPORT.md`, and `figures/degree_comparison.svg`.
