# Explicit public-target IC commands

`ic` and `icx` accept caller-supplied public curve, generator, and target points for
two bounded field families. Both commands reject malformed curves and points
before relation collection. A successful result includes a scalar checked by
an independent multiplication of the supplied generator.

## Prime field

```sh
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 --json
```

The example target is `[190]G`. The command requires a prime `p` in
`5..=1000000`, a nonsingular short-Weierstrass curve, a prime subgroup order
`n` in `3..=4000000`, canonical coordinates, and `[n]G = [n]Q = O`.
It counts `#E(F_p)` and records the exact cofactor before collection.
Relation collection is limited by `--fb-size`, `--extra-relations`,
`--max-trials-per-relation`, and `--max-relation-attempts`. The JSON report
separates factor-base construction, relations, linear algebra, and final
verification, and includes a same-target Pollard-rho reference unless
`--rho-steps 0` is set. The prime relation sampler currently uses the
process random source, so a second invocation is an independent run rather
than a bit-for-bit replay.
The verified public solve output used for the S3 integration check is frozen
as [`prime271_solve_report_20261018.json`](fixtures/prime271_solve_report_20261018.json).
Its SHA-256 is recorded in the linked
[`IC_REPORT_S3_ACCEPTANCE_20261018.json`](../store/IC_REPORT_S3_ACCEPTANCE_20261018.json)
receipt; the S3 acceptance object was deleted after the fresh pull check.

### Certified isogeny route

`solve-prime` can transport the supplied `G` and `Q` along one root-origin
`IW1` route before collecting IC relations:

```sh
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 \
  --route-file docs/ic/fixtures/prime271_isogeny_routes.json \
  --route-id IW1E11h1h1a619f4ab250 --json
```

The fixture was enumerated by `isogeny_walk walk` with `--primes 11`,
`--max-curves 3`, and `--max-depth 1`; the two edge certificates also passed
`isogeny_walk verify`. The IC command independently counts the starting
curve order, re-verifies every graph node, edge, and route ID, constructs each
point map from its certified kernel polynomial, checks the codomain and
isomorphism, and tests that each map commutes with point doubling. It rejects
an edge that kills the selected subgroup. Its JSON receipt records the route
hash, both transported points, and validation counts. Any scalar found on
the destination is checked on both the destination and the original public
curve. The Pollard-rho reference stays on the original `G,Q` pair.

## Cubic extension field

```sh
ic solve-cubic --input docs/ic/fixtures/cubic_f7_public_target.json \
  --seed 1 --max-residuals 100000 --json
```

The input describes `F_p[t]/(t³-c)` and `y²=x³+ax+b`, with field elements
as three canonical coefficients in increasing powers of `t`. The included
fixture has `p=7`, `c=2`, full curve order `347`, and target `[5]G`; its
coordinates and curve order were also computed independently with direct
extension-field arithmetic and point enumeration. The command requires prime
`p` in `7..=101`, a noncube `c<p`, prime `n` in `3..=2000000`, a nonsingular
curve, valid subgroup points, and a counted curve order equal to `n`. The
current Gaudry factor base requires that full prime-order condition.
It reports the Gaudry-style relation and linear-algebra counters and runs a
same-target rho reference unless `--skip-rho` is set.
The checked `ic solve-cubic` output for this input is frozen as
[`cubic_f7_solve_report_20261018.json`](fixtures/cubic_f7_solve_report_20261018.json).

An earlier public input on a curve of order `344 = 8 × 43` is retained as
[`cubic_f7_cofactor8_input.json`](fixtures/cubic_f7_cofactor8_input.json).
Before the full-order check was added, its bounded run used 100,000 residuals
and found zero decompositions; the exact report is
[`cubic_f7_cofactor8_bounded_20261018.json`](fixtures/cubic_f7_cofactor8_bounded_20261018.json).
That observation does not establish that no decomposition exists. The updated
CLI rejects the input before relation collection because the current factor
base requires the full curve group to have prime order.

The prime and cubic commands return `bounded_unknown` with a nonzero exit
status if their configured IC budget ends without a verified scalar. These
finite input ranges describe the implemented solvers; they do not certify
performance or coverage on larger curves. The timings are local diagnostics,
not a claim against the repository's full IC-versus-rho measurement gates.

## AES analysis adapters

```sh
crypto cryptanalysis aes-attack --technique differential
crypto cryptanalysis aes-attack --technique linear
crypto cryptanalysis aes-attack --technique square
crypto cryptanalysis aes-distinguisher --technique impossible --samples 4096
crypto cryptanalysis aes-distinguisher --technique yoyo --samples 4096
crypto cryptanalysis aes-distinguisher --technique mixture --samples 4096
crypto cryptanalysis aes-distinguisher --technique zero-correlation
```

These commands expose the existing AES-128 two-round differential,
one-round linear, and four-round integral recoveries. `--key-hex` supplies
the analysis instance; `--samples` and `--seed` configure the differential
and linear corpora. Each recovered result is checked against that instance.
The rest of the AES modules have separate input models and remain outside
this adapter; `crypto cryptanalysis sbox`, `auto`, `boomerang`, `rectangle`,
`aes-related-key`, and `aes-visual-demo` provide other entry points.
`aes-distinguisher` exposes four further bounded analyses: a four-round
impossible differential, a two-versus-three-round yoyo comparison, a
one-versus-two-round mixture identity comparison with subspace diagnostics,
and exact zero-correlation
mask enumeration on the AES S-box. The first three use a supplied AES-128
analysis key; the S-box check concerns the public substitution table. Each
command reports the configured sample count and its observed structural
check. These are distinct input models and do not imply an attack on
full-round AES.
