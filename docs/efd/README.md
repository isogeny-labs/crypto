# Explicit-Formulas Database in this repository

The current EFD at <https://www.hyperelliptic.org/EFD/> has exactly two
sections, and both are imported:

| Section | Field | Models | Charts | Formulas | Module |
| --- | --- | ---: | ---: | ---: | --- |
| `g12o` | GF(2^m) | 3 | 19 | 135 | `binary_ecc::efd` |
| `g1p` | GF(p), p odd | 10 | 33 | 471 | `prime_ecc::efd` |
| **Total** | | **13** | **52** | **606** | |

All 606 current-EFD formulas are executable. A supplementary section adds 15
formulas from the legacy EFD's extended Jacobi-quartic coordinates (the one
coordinate system absent from the current sections), converted from its Magma
programs and verified; see `prime-field.md`. In the prime section 463 run EFD's optimized
`.op3` directly and 8 use a record-derived fallback because the served program is
incomplete or contradicts its record (see `prime-field.md`).

Coverage was checked by crawling every representation page of both sections:
the set of `auto-code/*.op3` formula ids equals the catalogue id set exactly
(471/471 and 135/135, no missing and no extra ids). The legacy
`oldefd/` pages are the superseded genus-1 HTML tables. They carry no
machine-readable records or programs (only speed records and Magma checks); the
one coordinate system there not present in the current sections is "Extended
Jacobi-quartic coordinates", which has no explicit formula listing to import.
Their other content is covered by the two current sections. No genus-2 or
genus-3 section exists on the site.

Model conversions and chart conversions are drawn in
[`conversions.md`](conversions.md) (Graphviz source in `conversions.dot`,
rendered `conversions.svg`, Mermaid in `conversions.mmd`).

## Binary section

The complete [linked EFD binary-field section](https://www.hyperelliptic.org/EFD/g12o/index.html)
is available through `binary_ecc::efd`: **135 executable reference programs in
19 representations on three curve models**. This is an opt-in library addition.

| Operation | Imported programs |
| --- | ---: |
| Addition | 41 |
| Doubling | 47 |
| Tripling | 2 |
| Differential addition | 16 |
| Differential addition plus doubling (ladder step) | 16 |
| Scaling / normalization | 13 |

The representations are binary Edwards `w`, `wz`, `xy`, `xyz`, each generic and
with `d1=d2`; Hessian projective; and binary short Weierstrass affine, projective,
Jacobian, lambda, Lopez–Dahab (generic, `a2=0`, `a2=1`), extended Lopez–Dahab
(`a2=0`, `a2=1`), and XZ. Every representation has finite-point encoding and
projection helpers; quotient formats return a typed `X` or `W` projection.

This index concerns characteristic two. The prime-field `g1p` section is a
separate import under `prime_ecc::efd` (471 programs, 33 charts); see
[`prime-field.md`](prime-field.md). The same shared importer reads both
sections (`tools/efd` with an optional `g12o`/`g1p` argument). A binary Edwards
or Hessian formula requires its own model parameters and points. Selecting a
format does not convert a Weierstrass curve into another curve model.

The prime-field section is wired into the index-calculus front end through
`cryptanalysis::efd_representations`: every model converts to the
short-Weierstrass S3 curve and every chart can encode or decode points.  See
[`ic-representations.md`](ic-representations.md) and `ic prime --efd-model`.

## Use

Run these commands at the Rust crate root: `crypto/`, or `cryptanalysis/suite/`.

```sh
cargo run --release --example efd_binary -- list
cargo run --release --example efd_binary -- list shortw/lambda
cargo run --release --example efd_binary -- show shortw/extended-0/addition/madd-2005-dl
cargo run --release --example efd_binary -- demo
```

The list prints coordinate count and raw field primitive counts, including
parameter multiplications. It excludes conversions, parameter preparation,
validation and walk selection. It is not a timing table or a rho-step price.

For full-point short Weierstrass ADD/DBL, `weierstrass_operation(curve, id, p, q)`
accepts the existing `BinaryCurve`/`BinaryPoint` types. Use `Some(q)` for ADD and
`None` for DBL. Its result exposes `ExecutionPath::Formula` or
`ExecutionPath::AffineExceptional`; infinity, equal/opposite inputs and chart
exceptions use the existing complete affine law. The adapter validates input
and output points and normalizes the result, so its conversions are real work.

For lower-level use:

1. Find a `Formula` and its `Representation` in `catalogue()`.
2. Encode finite points with `encode_affine(field, x, y, scale)` and suffix their
   names with `bind_point`. Supply the correct model's parameters in `Bindings`.
3. Compile once with `formula.compile()` and evaluate with `program.evaluate`.
4. Project result 3 for ADD/DBL/TPL/SCALE, result 5 for DADD, and results 4/5
   for the doubled/summed points of a ladder step.

For DADD/LADD the inputs are `P1=P3-P2`, `P2`, `P3`. The low-level evaluator
checks numerical assumptions and cached-coordinate consistency, but the caller
must establish curve membership and that difference relation. XZ/W/WZ formats
do not supply ordinary arbitrary-point addition or recover an omitted ordinate.

Mixed-input restrictions such as `Z2=1`, model restrictions such as `a2=0`, and
parameter relations such as `sqrta6^2=a6` are checked. Missing derived cache
parameters are computed from their definitions; supplied ones must agree.
Missing point coordinates, zero inversions, field-width mismatches, invalid
inputs in the checked adapter, and unavailable finite charts return errors.
The existing field API's irreducibility assumption is retained: the caller
supplies the exact irreducible polynomial, not just the field degree.

## Validate and refresh

```sh
# Normal library integration and example test.
cargo test --release --lib
cargo test --release --example efd_binary

# Focused native target: compiles the actual field, curve and EFD source files.
EFD_COVERAGE_OUT=/tmp/efd-coverage.json \
  cargo test --release --manifest-path tools/efd/Cargo.toml efd:: -- --nocapture

# Retrieve original mathematical records and three-operand programs.
cargo run --manifest-path tools/efd/Cargo.toml -- \
  /tmp/efd-source-cache src/binary_ecc/efd/catalogue.json

# From crypto; from cryptanalysis/suite use ../../crypto instead.
bash tools/efd/check-mirror.sh ../cryptanalysis/suite
```

The importer is native Rust with curl for HTTP retrieval. It preserves original
model definitions, representation relations, source attribution, assumptions,
three-operand code, exact source URLs and SHA-256 hashes. It fails if coverage
differs from the reviewed 135/19 snapshot. Delete the external cache or use a new
cache directory to retrieve a new snapshot; inspect the resulting diff before
updating both copies. Source code is parsed as arithmetic, never executed as
Rust, Python, Sage or shell.

## Evidence and limits

See [the dated implementation report](REPORT.md),
[coverage receipts](validation/), and [report PDF](report.pdf).
The core source, tests, importer and catalogue are copied byte for byte between
the two Rust crates; `check-mirror.sh` checks that property. The example differs
only in its crate import name.

All 135 formulas are compared with EFD's unoptimized affine model laws over
GF(2^4) and GF(2^5), with both unit and nonunit projective scales where allowed.
Chart exclusions are counted. Applicable full-point Weierstrass ADD/DBL
programs are also compared with each repository's pre-existing affine arithmetic
at `a2=0,1,z`, and on public known-scalar fixtures over 113-, 131- and 163-bit
fields. These finite checks are not universal algebraic proofs; upstream Sage
verification scripts are linked in every formula record and were not replayed.

No production kernel or walk selector is changed. In particular, a rho partition
or distinguished-point predicate must be independent of projective scaling;
hashing raw `X` is insufficient. GPU code generation, occupancy and complete
rho-step benchmarking remain follow-up work. There is no new throughput,
end-to-end ECDLP, asymptotic or deployed-curve-security claim.

## Sources checked

- Bernstein and Lange's [binary EFD index](https://www.hyperelliptic.org/EFD/g12o/index.html),
  all 19 representation records, 135 formula records and 135 three-operand
  programs, plus the three affine model definitions; snapshot 2026-10-21.
- [Lambda coordinates](https://www.hyperelliptic.org/EFD/g12o/auto-shortw-lambda.html),
  including the 2013 Oliveira–López–Aranha–Rodríguez-Henríquez formulas and their
  cached-parameter assumptions.
- [Binary Edwards model and maps](https://www.hyperelliptic.org/EFD/g12o/data/edwards/coordinates)
  and [Hessian model and maps](https://www.hyperelliptic.org/EFD/g12o/data/hessian/coordinates).
- `crypto:ecc2k130/LAMBDA-PROJECTIVE.md` and its historical codegen tests:
  existing coordinate arithmetic, scaling-invariance requirements and the
  previously priced comparison with batched affine arithmetic. No old result
  is rerun, rewritten, or promoted by this implementation.

The imported methods are established prior work; no formula novelty is claimed.
