# Large standard-curve applications

The ordinary-ring theorem applies at arbitrary prime-field size. This
extension removes the native checker's machine-word limit and supplies four
fixed public correctness fixtures, each with five polynomial variables and
three explicit cubic membership inputs.

| Standard curve | Field bits | Replayed DAG nodes | Maximum intermediate degree | Curve witness recheck |
|:--|--:|--:|--:|:--|
| secp256k1 | 256 | 413 | 5 | accepted |
| secp256r1 / P-256 | 256 | 415 | 5 | accepted |
| secp384r1 / P-384 | 384 | 415 | 5 | accepted |
| secp521r1 / P-521 | 521 | 415 | 5 | accepted |

Node counts and maximum degrees are recorded replay outputs, not timing
measurements. Each serialized proof independently derives `1` within degree
five. Exact first/last pair `(4,5)` additionally uses the uniform mathematical
lower/first-fall proofs; no new degree-four closure was computed.

## Public construction and verification

For each official generator `G`, use `S={x(G),x(2G),x(4G)}` and target
`R=16G`. Every signed three-point sum has coefficient `s` with `abs(s)<=12`.
Equality to either sign of `16G` would imply a nonzero zero-multiple `kG=O`
with `4<=abs(k)<=28`. Rechecking `G,...,28G` excludes that possibility,
including repeated factor-base abscissas. It also establishes distinct
abscissas and an affine target outside the membership set.

The fixture checker pins the parameters, checks the curve and all point
recurrences, rebuilds all six polynomial inputs, and checks the serialized
witness, metadata, and normalized input SHA. The constructive producer emits
elementary operations; a separate sparse-arithmetic implementation replays
them. Parameter provenance and polynomial replay are distinct checks.
Replay alone does not certify primality or curve semantics.

An independent OpenSSL 3.5.7 check matches all 112 public point coordinates
and all four sets of field/curve parameters. Its native helper, complete
decimal-string output, and per-curve comparison receipts are retained under
`validation/` and `fixtures/*/openssl-recheck.json`.

The field primes and generators come from [SEC 2 v2.0](https://www.secg.org/sec2-v2.pdf),
Sections 2.4–2.6, source SHA-256
`87b8f3703364ed5b21ba8582e411cc0cbf477bcaa3f4f45e0d6580d1c00d9952`.
[standard_parameters.json](standard_parameters.json) records each existing
ICV1 slug, EC1 representation, and curve UID. No new group-order or primality
certificate is claimed. See [the applicability and fixture proof](proof_scope.txt).

## Reproduce

From the repository root:

```sh
cargo build --release --locked --manifest-path research/guarded_fall_degree_20261028/native/Cargo.toml --bins
study=research/guarded_fall_degree_20261028
"$study/native/target/release/large_curve_fixtures" check "$study/extensions/large_curves/fixtures/secp521r1/problem.json" "$study/extensions/large_curves/fixtures/secp521r1/curve-witness.json"
"$study/native/target/release/big_prime_replay" verify "$study/extensions/large_curves/fixtures/secp521r1/problem.json" "$study/extensions/large_curves/fixtures/secp521r1/upper_D5.json"
```

To regenerate into a separate output directory, run
`large_curve_fixtures issue CURVE OUTDIR`, then
`construct_big_prime PROBLEM.json UPPER_D5.json CONSTRUCTION.json` and the
independent replay command. The committed [fixtures](fixtures/) each include
the input, point witness, construction report, upper DAG, and replay/recheck
records. [validation/](validation/) records the native checks.

The producer supports up to 4096-bit moduli. Replay uses exact arbitrary-
precision integers with explicit document/token/structure limits. Large
input values remain numeric JSON tokens without floating-point conversion.

## Scope and remaining work

A separate [prime-field bit-restricted theorem](prime_bit_restricted_upper.txt)
proves `d_F<=3*ell+2` for normalized lookup inputs over `F_p`, independently
of modulus size and auxiliary bit width. All lookup images must have rational
point lifts and auxiliary bit encodings must cover `F_p`; every bit equation
is explicit. Raw inputs have bound `max(3*ell+2,d_raw)`, at most
`max(3*ell+2,4*ell)` for arbitrary multilinear lookups. Affine lookups retain
the `3*ell+2` bound even for raw inputs. This concerns `F_p` coefficients,
not a `GF(2)` carry circuit, and supplies no small proof-size or runtime bound.

Four fixed fixtures were planned, issued, constructed, and independently
replayed; all four succeeded. These are native development correctness checks
of known-answer cases, not a field/base/target sweep or timing comparison.
Every scalar is public construction data; this is not an ECDLP recovery run.

The ordinary binary theorem already covers arbitrary `n`. SEC 2's binary
sizes `163,233,239,283,409,571` are odd, so an unsatisfiable three-abscissa
instance satisfying that theorem has pair `(4,5)`. This extension supplies
no numerical proof files for those large binary curves. The separate fully
Boolean bound `d_F<=3*ell+1` remains independent of `n`, under its stated
linear-embedding and explicit-bit-equation hypotheses.

These three-abscissa fixtures are not linear Boolean factor-base subspaces.
They do not validate the Boolean theorem or E12 closure computations. Growing
cardinality retains its separate bounds and unresolved exact values. The
original Lean source remains unchanged with twelve checked lemmas. A separate
`PrimeBitLocal` module adds eight checked local dependencies, including the
nonzero-discriminant branch and quadratic cancellation. Neither module
formalizes the complete degree theorem, public curve witness, or group argument.

The [coverage diagram](coverage.svg) separates executed checks from theorem-
only binary coverage. The main [report PDF](../../reports/report.pdf) includes
this table and its limits. No performance dashboard changed because no
timing or cost ratio was measured.
