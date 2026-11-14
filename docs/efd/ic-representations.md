# EFD representations in the index-calculus engine

`crypto_lib::cryptanalysis::efd_representations` wires the prime-field EFD
catalogue (`prime_ecc::efd`, 471 formulas / 33 charts / 10 models) into the
prime-field index calculus used by `ic prime`.

The relation engine is unchanged: it works on

```text
y^2 = x^3 + A x + B
```

because the third summation polynomial is defined in that x-coordinate.  The
representation layer converts the input curve and points to that model and
converts results back to the selected EFD model and chart.

## Model bridge

Every EFD model record pins the long-Weierstrass invariants and the two
affine maps:

```text
a0 y^2 + a1 x y + a3 y = x^3 + a2 x^2 + a4 x + a6
(u, v) = toweierstrass(x, y)
(x, y) = fromweierstrass(u, v)
```

`Model::weierstrass_bridge` evaluates those records over `F_p` and performs
the standard transformations:

```text
X = a0·u,          W = a0^2·v
B2 = a0·a2 + a1^2/4
B4 = a0^2·a4 + a1·a0·a3/2
B6 = a0^3·a6 + (a0·a3)^2/4
A  = B4 - B2^2/3
B  = B6 - B2·B4/3 + 2·B2^3/27
X' = X + B2/3
Y  = W + (a1·X + a0·a3)/2
```

The inverse is

```text
X  = X' - B2/3
W  = Y - (a1·X + a0·a3)/2
u  = X/a0
v  = W/a0^2
```

This covers all ten models, including the ones with non-zero `a1` or `a3`
(twisted Hessian).  `a0` must be invertible and `p > 3`; the EFD affine maps
have exceptional points (identity, 2-torsion, chart boundaries) which return
`ZeroDenominator` and are reported rather than silently moved.

| model | equation |
| --- | --- |
| `shortw` | `y^2 = x^3 + a x + b` |
| `montgom` | `b y^2 = x^3 + a x^2 + x` |
| `edwards` | `x^2 + y^2 = c^2 (1 + d x^2 y^2)` |
| `twisted` | `a x^2 + y^2 = 1 + d x^2 y^2` |
| `twistedhessian` | `a x^3 + y^3 + 1 = d x y` |
| `hessian` | `x^3 + y^3 + 1 = 3 d x y` |
| `jintersect` | `s^2 + c^2 = 1`, `a s^2 + d^2 = 1` |
| `jquartic` | `y^2 = x^4 + 2 a x^2 + 1` |
| `2dik` | `y^2 = x^3 + a x^2 + 16 a x` |
| `3dik` | `y^2 = x^3 + 3 a (x + 1)^2` |

## Charts

`EfdRepresentation` accepts any of the 33 catalogue chart ids, plus `affine`.
`encode_short` converts a short-Weierstrass point to the chart;
`decode_short` converts chart coordinates back.  Full charts round-trip both
coordinates.  Quotient charts return only what they retain:

| chart | retained |
| --- | --- |
| `shortw/xz`, `montgom/xz` | `x` |
| `edwards/yz` | `y` |
| `edwards/yzsquared` | `y^2` |
| `jintersect/*` | `(s, c, d)` |

For a quotient chart the caller selects a lift with `--efd-lift N`.  The
generator and target can use different lifts
(`--efd-generator-lift`, `--efd-target-lift`), because a y-only chart does not
determine the relative sign of two points.  Chart parameters that the
catalogue does not fix are supplied as ordinary parameters, for example
`--efd-param r=8` for the Edwards `yz` chart when `d = r^2`.

## CLI

```bash
# Montgomery affine
ic prime --efd-model montgom --efd-p 1019 --efd-order 269 --efd-cofactor 4 \
  --efd-param a=131 --efd-param b=1 --size 6

# Montgomery xz (the EFD Montgomery representation)
ic prime --efd-model montgom --efd-p 1019 --efd-order 269 --efd-cofactor 4 \
  --efd-param a=131 --efd-param b=1 --efd-chart xz --size 6 \
  --efd-generator 604,1 --efd-target 855,1 \
  --setup-candidates 32 --target-candidates 32

# Edwards yz (y-only; d = r^2)
ic prime --efd-model edwards --efd-p 1019 --efd-order 131 --efd-cofactor 8 \
  --efd-param c=1 --efd-param d=64 --efd-param r=8 --efd-chart yz \
  --efd-generator 628,1 --efd-target 47,1 \
  --efd-generator-lift 0 --efd-target-lift 2 \
  --setup-candidates 64 --target-candidates 64

# Twisted Edwards extended coordinates
ic prime --efd-model twisted --efd-p 1019 --efd-order 257 --efd-cofactor 4 \
  --efd-param a=2 --efd-param d=12 --efd-chart extended \
  --efd-generator 893,734,1,245 --efd-target 795,916,1,654 \
  --setup-candidates 32 --target-candidates 32
```

`--efd-model` ignores `--curve`; the factor base is always the small-x base
of the short-Weierstrass S3 coordinate.  The JSON report carries a
`representation` object with the model, chart, model parameters, chart affine
map, pinned EFD sources and the short-Weierstrass `(A, B)`.

## Operational checks

- `--efd-list` prints all ten models, their parameters and equations, every
  chart, and the number of imported formulas per chart.
- `--efd-check` round-trips points through the selected chart;
  `--efd-check --efd-chart all` checks every chart of the model and reports
  per-chart results.
- `--efd-p` and `--efd-order` are checked with Miller-Rabin (bases 2..53).
- Unknown model, chart, or parameter names are rejected with the accepted set.
- A model whose parameters give a singular short-Weierstrass curve is rejected.
- Chart parameters that are supplied are checked against their defining
  relation (`r^2 = d` for Edwards YZ).
- Coordinates must be canonical and belong to the representation's field.
- `--efd-lift` must be below `EfdRepresentation::max_lift` (2 for x-only and
  y-only, 4 for squared-y).
- A supplied generator or target is checked for curve membership and for
  `[n]P = O` before the exact probe.
- Every public conversion returns an error instead of panicking on malformed
  input or an exceptional point of the rational map.

No production path uses `unwrap`/`expect` on user input; the remaining
`unwrap`s are in test code.

## `ic bench`

A prime instance can be built directly from an EFD model/chart, so sweeps and
end-to-end pipelines use the same representation layer:

```bash
ic bench --efd-model montgom --efd-p 1019 \
  --efd-param a=131 --efd-param b=1 \
  --efd-chart xz --efd-order 269 --efd-cofactor 4 \
  --efd-generator 604,1 \
  --factor-base prime-abscissa:size=8 --oracle subtract --targets walk
```

The instance name is `EFD <model>/<chart> over F_p`; the JSON report carries a
`representation` block with the chart, parameters, affine map and the derived
short-Weierstrass `(A, B)`.  `PrimeInstance::from_efd` decodes the generator in
the chart, checks it on the short-Weierstrass curve and checks `[n]G = O`
before the bench starts.  `decode_efd_point` and `encode_efd_point` expose the
conversion to callers; instances without a representation keep
`representation: None`, which is omitted from serialized output.

## `ecbench` and boundary records

`ecbench` accepts an EFD instance as a `CurveSpec::Efd` JSON:

```bash
ecbench fb --curve '{"kind":"efd","model":"montgom","chart":"xz","p":1019,
  "params":{"a":131,"b":1},"r":269,"cofactor":4,"generator":[604,1],"lift":0}' \
  --factor-base prime-abscissa:size=6
```

The workload and factor-base dump carry a `representation` block (model,
chart, parameters, lift); the curve facts still describe the derived
short-Weierstrass curve.  Boundary ledger rows carry the same block on the
prime regime.

## Chart arithmetic

`EfdRepresentation::chart_add` and `chart_double` evaluate the imported EFD
formula in the selected chart and project the result back to the
short-Weierstrass point.  `formula_ids(operation)` lists the available
formulas.  This uses the 471-formula catalogue directly; it does not replace
the optimized engine arithmetic.

## Verification

```sh
cargo test --lib prime_ecc::efd          # 10 tests, 471 formulas
cargo test --lib efd_representations     # 5 tests: models, charts, IC, invalid input
cargo test --bin ic parses_efd_integers  # CLI parsing
cargo test --bin ic efd_catalogue_lists  # model/chart catalogue
cargo run --release --example efd_ic     # library use on Montgomery xz
```

The tests check:

- every model round-trips short-Weierstrass points through its pinned
  `fromweierstrass` / `toweierstrass` maps and validates them with
  `Model::on_model`;
- every model and every chart encodes and decodes points;
- a Montgomery `xz` curve runs the index-calculus measurement and the exact
  relation/rank/descent probe;
- quotient-chart lifts select both points that the chart identifies.

Observed exact probes on `F_1019`:

| model/chart | order | generator -> target | recovered |
| --- | ---: | --- | ---: |
| `montgom/xz` | 269 | `604,1 -> 855,1` | `0x89` = 137 |
| `twisted/extended` | 257 | `893,734,1,245 -> 795,916,1,654` | `0x89` = 137 |
| `edwards/yz` | 131 | `628,1 -> 47,1`, lifts 0/2 | `0x6` = 137 mod 131 |

## Scope limits

- The caller supplies `p`, the subgroup order and the cofactor; the engine does
  not count a large curve.
- The factor base is enumerated on the short-Weierstrass S3 x-line, not on the
  input model's x.  For Edwards and twisted Edwards those are different
  quotients.
- Quotient charts identify a finite set of points; the lift index is part of
  the input, not inferred.
- Exceptional points of a rational model map are outside its domain.
- Binary-field EFD data is not used here; `ic prime` is the prime-field path.
