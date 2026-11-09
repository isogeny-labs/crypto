# Prime-field EFD formulas (`g1p`) in `prime_ecc::efd`

Integration of the genus-1 large-characteristic section of the
[Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/index.html),
mirroring the binary-field import described in `docs/efd/REPORT.md`.

## What was imported

| Quantity | Upstream | Imported |
| --- | ---: | ---: |
| Formula records | 471 | 471 |
| Representations (charts) | 33 | 33 |
| Curve models | 10 | 10 |

The catalogue is `src/prime_ecc/efd/catalogue.json`
(`schema = efd-prime-catalogue/v1`, `section = g1p`, `characteristic = "p"`).
For every formula it pins:

- the upstream `data/<id>` record (assumptions, prose definition),
- the `auto-code/<id>.op3` straight-line program,
- SHA-256 of both, and the `auto-sage/<id>.sage` upstream check URL.

The importer is shared with the binary section:

```sh
cargo run --release --manifest-path tools/efd/Cargo.toml -- \
  CACHE_DIR OUTPUT.json g1p        # or g12o, the default
```

Cache paths and record layout are identical between the two sections, so a
single importer keeps both catalogues reproducible.

Operations in the section:

| Operation | Count |
| --- | ---: |
| addition (incl. mADD, mmADD, zADD) | 199 |
| doubling (incl. mDBL) | 161 |
| tripling | 44 |
| scaling | 25 |
| ladder (LADD, mLADD) | 23 |
| differential addition | 19 |

Representations by model: short Weierstrass 11, Jacobi quartic 6, twisted
Edwards 4, Edwards 4, Jacobi intersection 2, Hessian 2, `2dik` 1, `3dik` 1,
Montgomery 1, twisted Hessian 1.

## Usage

```rust
use crypto::ecc::field::FieldElement;
use crypto::prime_ecc::efd::{catalogue, Bindings};

let modulus = p256_p.clone();               // BigUint
let rep = catalogue().representation("shortw/jacobian").unwrap();
let params = Bindings::from([
    ("a".into(), FieldElement::new(a, modulus.clone())),
    ("b".into(), FieldElement::new(b, modulus.clone())),
]);
let z = FieldElement::one(modulus.clone());
let point = rep.encode_affine(&modulus, &params, &x, &y, &z)?;
let mut inputs = params.clone();
inputs.extend(rep.bind_point(&point, 1));
inputs.extend(rep.bind_point(&second_point, 2));
let output = catalogue()
    .formula("shortw/jacobian/addition/add-2007-bl")
    .unwrap()
    .evaluate(&modulus, &inputs)?;
let result = rep.project(&params, &output, 3)?;
```

`weierstrass_operation` is the checked ADD/DBL adapter for short Weierstrass
charts: it validates inputs with the existing `ecc::curve::CurveParams` law,
encodes at unit scale, runs the formula, checks the result on the curve, and
reports `ExecutionPath::AffineExceptional` when it had to fall back.

Costs from `Program::operation_counts` are raw field operations in the
straight-line program, including parameter multiplications. Inversions are
counted separately; they are not weighted (an inversion is much more than one
multiplication in `F_p`). These are not timings or complete rho-step costs.

## Chart coverage (33/33)

Every published representation has an explicit encoder and projector; there is
no catch-all default. Unknown chart ids return `EfdError::Unsupported` from both
`encode_affine` and `project`. Cached coordinates (`XX`, `ZZ`, `R`, `T`, `SC`,
`DZ`, `XY`, ...) are filled from the chart's own `satisfying` relations.

| Chart | Variables | Affine map | `project` returns |
| --- | --- | --- | --- |
| `shortw/jacobian` | X, Y, Z | x = X/Z^2; y = Y/Z^3 | (x,y) |
| `shortw/jacobian-0` | X, Y, Z | x = X/Z^2; y = Y/Z^3 | (x,y) |
| `shortw/jacobian-3` | X, Y, Z | x = X/Z^2; y = Y/Z^3 | (x,y) |
| `shortw/modified` | X, Y, Z, T | x = X/Z^2; y = Y/Z^3 | (x,y) |
| `shortw/projective` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `shortw/projective-1` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `shortw/projective-3` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `shortw/w12-0` | X, Y, Z | x = X/Z; y = Y/Z^2 | (x,y) |
| `shortw/xyzz` | X, Y, ZZ, ZZZ | x = X/ZZ; y = Y/ZZZ | (x,y) |
| `shortw/xyzz-3` | X, Y, ZZ, ZZZ | x = X/ZZ; y = Y/ZZZ | (x,y) |
| `shortw/xz` | X, Z | x = X/Z | x |
| `montgom/xz` | X, Z | x = X/Z | x |
| `edwards/inverted` | X, Y, Z | x = Z/X; y = Z/Y | (x,y) |
| `edwards/projective` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `edwards/yz` | Y, Z | r\*y = Y/Z | y |
| `edwards/yzsquared` | Y, Z | r\*y^2 = Y/Z | y^2 |
| `twisted/extended` | X, Y, Z, T | x = X/Z; y = Y/Z | (x,y) |
| `twisted/extended-1` | X, Y, Z, T | x = X/Z; y = Y/Z | (x,y) |
| `twisted/inverted` | X, Y, Z | x = Z/X; y = Z/Y | (x,y) |
| `twisted/projective` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `twistedhessian/projective` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `hessian/extended` | X, Y, Z, XX, YY, ZZ, XY, YZ, XZ | x = X/Z; y = Y/Z | (x,y) |
| `hessian/standard` | X, Y, Z | x = X/Z; y = Y/Z | (x,y) |
| `jintersect/extended` | S, C, D, Z, SC, DZ | s = S/Z; c = C/Z; d = D/Z | (s,c,d) |
| `jintersect/standard` | S, C, D, Z | s = S/Z; c = C/Z; d = D/Z | (s,c,d) |
| `jquartic/2xxyzz` | X, XX, Y, Z, ZZ | x = X/Z; y = Y/ZZ | (x,y) |
| `jquartic/2xxyzzr` | X, XX, Y, Z, ZZ, R | x = X/Z; y = Y/ZZ | (x,y) |
| `jquartic/2xyz` | X, Y, Z | x = X/Z; y = Y/Z^2 | (x,y) |
| `jquartic/xxyzz` | X, XX, Y, Z, ZZ | x = X/Z; y = Y/ZZ | (x,y) |
| `jquartic/xxyzzr` | X, XX, Y, Z, ZZ, R | x = X/Z; y = Y/ZZ | (x,y) |
| `jquartic/xyz` | X, Y, Z | x = X/Z; y = Y/Z^2 | (x,y) |
| `2dik/standard` | X, Y, Z, ZZ | x = X/Z; y = Y/ZZ | (x,y) |
| `3dik/standard` | X, Y, Z, ZZ | x = X/Z^2; y = Y/Z^3 | (x,y) |

### Curve equations (10/10)

`Model::on_model(modulus, params, point)` evaluates every `satisfying` equation
of the pinned model record, so all ten shapes are covered by one code path:

| Model | Equation | Coordinates |
| --- | --- | --- |
| `shortw` | y^2 = x^3 + a x + b | x, y |
| `montgom` | b y^2 = x^3 + a x^2 + x | x, y |
| `edwards` | x^2 + y^2 = c^2 (1 + d x^2 y^2) | x, y |
| `twisted` | a x^2 + y^2 = 1 + d x^2 y^2 | x, y |
| `twistedhessian` | a x^3 + y^3 + 1 = d x y | x, y |
| `hessian` | x^3 + y^3 + 1 = 3 d x y | x, y |
| `jintersect` | s^2 + c^2 = 1, a s^2 + d^2 = 1 | s, c, d |
| `jquartic` | y^2 = x^4 + 2 a x^2 + 1 | x, y |
| `2dik` | y^2 = x^3 + a x^2 + 16 a x | x, y |
| `3dik` | y^2 = x^3 + 3 a (x+1)^2 | x, y |

Chart parameters fixed by the model are derived when absent: `r` from `d = r^2`
for the YZ charts, and the missing member of `a^2 + c^2 = 1` for the
doubling-oriented Jacobi quartic charts. A square root has two signs; the
formulas are identities modulo those defining relations, so either choice is
consistent, and callers that need a specific root can supply it.

## Legacy supplementary section: extended Jacobi quartic

The old EFD page `oldefd/extendedquartic.html` carries a coordinate system the
current sections do not: **extended Jacobi-quartic coordinates**
`(X:Y:Z:U:V:W:T)` with `x=X/Z`, `y=Y/Z^2`, `U=X^2`, `V=2XZ`, `W=Z^2`,
`T=U+W`. Its Magma blocks were converted statement-for-statement into
`src/prime_ecc/efd/legacy_catalogue.json`:

- 15 formulas: 6 addition, 6 doubling, 2 tripling, 1 scaling, named after their
  authors (for example `2007-duquesne-page-103-table-1`).
- Each record keeps the raw Magma block and its SHA-256; each program keeps the
  converted straight-line text and its SHA-256. The upstream HTML page hash
  (`c0d4a89f...`) is pinned in the catalogue and in `legacy::SOURCE_PAGE_SHA256`.
  The page has no `.op3` files, so this is the only machine-readable form.
- Parameters are derived from each block's setup: `b=-2a` or `b=a^2-1`,
  `epsilon=1`, `delta=-a`, and `alpha^2+delta^2=1` (Tonelli--Shanks).
- `jquartic/extended` is implemented in `formats.rs` (encode + project, caches
  from the chart relations) and verified against the pinned Jacobi-quartic
  affine law at `p = 109` by
  `legacy_extended_quartic_formulas_match_affine_law`.

These formulas are supplementary: they are not part of the current-EFD counts
(606) and are not in the Lean manifest, which pins only the 33 current charts.
With them the repository carries **621 machine-readable EFD formulas**.

## Formal verification of the chart maps

`research/efd_coordinate_charts_20261108/Charts.lean` machine-checks the
encode/project round trip for every chart family over an abstract field-like
structure (commutative multiplication, inverse on nonzero elements, the usual
inverse laws). Theorems: `affine`, `projective`, `jacobian`, `xyz`, `xyzz`,
`x_only`, `inverted`, `yz`, `yzsquared`, `jintersect`. Assumptions are explicit
nonzero denominators (projective scale, inverted coordinates, and `r` for the
YZ charts); the identities are characteristic-independent.

The manifest at the end of that file pins all 33 chart ids to their proof
family, and the Rust test `lean_chart_manifest_matches_catalogue` fails if the
manifest, the catalogue id set, or the family dispatch in `formats.rs` drift.

The family codes are also tied to the actual encoders: `chart_family_constants_match_encode`
checks that each chart writes `X = x*z^a`, `Y = y*z^b` (or `z/x`, `z/y`,
`r*y*z`, ...) with the exponents the corresponding Lean theorem proves.

```sh
bash tools/lean/proof-gate.sh research/efd_coordinate_charts_20261108/Charts.lean
```

Result: `PASS 37 thm 0 trusted`, i.e. every theorem is proved with no `sorry`,
`native_decide` or user axiom.

## Verification

Model and chart conversions (including the shared Weierstrass hub) are in
[`conversions.md`](conversions.md). Commands and coverage:
`docs/efd/prime-validation/`.

| Check | Result |
| --- | --- |
| Catalogue size, ids, per-source SHA-256 | 471 / 33 / 10, all match |
| Upstream `.op3` programs compile | 465 of 471 (6 upstream-incomplete, below) |
| Executable formulas | **471 of 471** (463 upstream + 8 record-derived fallback) |
| Chart ids and curve equations pinned | 33 ids, 10 models, all explicitly handled |
| Weierstrass bridge present both ways | 10/10 models (`Model::to_weierstrass`, `from_weierstrass`) |
| Chart round trips, machine-checked | 10 Lean theorems covering all 33 charts (37 theorems, 0 trusted) |
| Affine chart round trip, scales 1, 2, 7 | all 33 charts (also from model parameters alone, no `r`/`c` supplied) |
| Ten curve equations + 33 chart ids pinned, unknown charts rejected | `all_ten_curve_equations_and_thirty_three_charts_are_covered` |
| Formula vs pinned affine model law, `p = 109` | 471 formulas, >= 8 comparisons each (cap 32); fallback rows flagged in coverage |
| Checked short-Weierstrass adapter vs `ecc::point::Point` (`p = 109`, `a = 0` and `a = -3`) | all applicable formulas |
| Checked adapter on secp256k1 (`a = 0`) and P-256 (`a = -3`) planted generator multiples | all applicable formulas, formula path |

The model-law check evaluates the unoptimized affine `addition`, `doubling`,
`negation` and `tripling` definitions from each pinned `data/coordinates`
record; it is independent of the optimized `.op3` program under test. For the
eight formulas whose upstream `.op3` is unusable, the program under test is
reconstructed from that formula's own `compute` lines, so those rows are
record-consistency checks rather than independent ones; the six short
Weierstrass ones are additionally checked against `ecc::Point` on `p = 109`,
secp256k1 and P-256. Each coverage row carries `"program": "upstream"` or
`"record-fallback"`. Points
are enumerated exhaustively on `F_109`. Formulas whose declared assumptions do
not hold for a sample (for example `X2 = 1` mixed charts or `Z1 = 1`) are
skipped for that sample, and singular affine charts and vanishing denominators
are excluded. Coverage rows are in
`docs/efd/prime-validation/coverage.json`.

## Upstream defects found

These are pinned and reported, not silently repaired.

| Formula | Finding |
| --- | --- |
| `shortw/jacobian{,-0,-3}/addition/add-1986-cc-2` (3) | The upstream `.op3` contains the EFD generator's own `error: not sure how to handle ...` line instead of the `Y3` instruction. `compile` returns `Unsupported`; the record and hash are retained. |
| `shortw/projective{,-1,-3}/addition/add-1986-cc` (3) | Same: incomplete upstream program. |
| `jintersect/{standard,extended}/doubling/dbl-2001-ls` (2) | The `.op3` computes `r2 = -t7` with `t7 = r1 + l3`; the pinned record says `r2 = -r1 + l3`. The program disagrees with its own record on every tested point; verification uses the record-derived fallback. |

The 2 `jintersect` entries are listed in `KNOWN_PROGRAM_DEFECTS` in
`src/prime_ecc/efd/tests.rs` together with the evidence assertion.

No program was rewritten: the catalogue preserves the bytes that were served.
`Formula::compile` still returns exactly the upstream program (or `Unsupported`
for the six incomplete ones); `Formula::compile_record` and `Formula::compile_any`
expose the record-derived fallback, and `Program::source()` distinguishes the
two. Costs from a fallback program are record costs, not EFD's optimized counts.

## Scope limits

- No automatic conversion between curve models; the caller supplies a point on
  the representation's model and matching parameters.
- Quotient charts return only what they retain: `X` for `shortw/xz` and
  `montgom/xz`, `Y` for `edwards/yz`, `Y^2` for `edwards/yzsquared`, and
  `(s, c, d)` for Jacobi intersections.
- Parameter derivation follows defining `assume` lines (bare variables, sums
  with known terms, constant multiples, products with a known factor, and
  squares via Tonelli--Shanks). Assumptions that need a root of a general
  polynomial (for example `w^2 + w + 1 = 0`) must be supplied by the caller.
- Eight upstream programs are unusable; use `compile_any` when a formula must
  execute, and check `Program::source()` before quoting operation counts.
- `encode_affine`/`encode_jintersection` are coordinate maps. They do not by
  themselves assert curve membership; use `Model::on_model` when that check is
  wanted (the checked short-Weierstrass adapter always performs it).
- The sweeps run at two primes: the default `p = 109` and `EFD_TEST_P=97`.
- `tools/efd` builds again (the check target stubs the cryptanalysis ledger and
  the prime-field Certicom lookup). `tools/efd/replay.sh G1P_CACHE G12O_CACHE` regenerates
  both catalogues and diffs them, ignoring the retrieval date.
- Only short Weierstrass has an adapter to the crate's existing curve law.
  The other nine models are verified against the pinned EFD affine records;
  for `2dik` that record was observed not to reproduce the short-Weierstrass
  law induced by its own `toweierstrass` map at `p = 109`, so treat that
  family as record-faithful rather than independently group-law-verified.
- Nothing here changes default arithmetic, routing or any rho walk.

## Reproducing

```sh
# regenerate the catalogue (network; caches responses under CACHE_DIR)
cargo run --release --manifest-path tools/efd/Cargo.toml -- CACHE_DIR \
  src/prime_ecc/efd/catalogue.json g1p

# full prime EFD test group
cargo test --lib prime_ecc::efd

# also emit coverage rows
EFD_COVERAGE_OUT=/tmp/prime-coverage.json cargo test --lib prime_ecc::efd
```
