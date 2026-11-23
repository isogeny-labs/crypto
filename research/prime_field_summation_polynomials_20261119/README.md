# Prime-field Semaev summation-polynomial structure lab

> **Repository status:** exploratory historical import, not a canonical benchmark
> result. The measurements and correctness receipts are repository-unverified
> `[pcc:unverified]`; no end-to-end ECDLP claim, fastest-tool claim, or
> `benchmarks/README.md` admission is made. See [IMPORT_NOTES.md](IMPORT_NOTES.md)
> for provenance, overlap, evidence boundaries, and the independent validation
> still required by this repository.

This imported study generates specialized Semaev summation polynomials

\[
S_m(x_1,\ldots,x_{m-1},X) \in \mathbf F_p[X]
\]

for short-Weierstrass curves `y^2 = x^3 + a*x + b`, with odd prime `p > 3` and
nonzero curve discriminant. Inputs `x_i` are sampled as the x-coordinates of known
curve points. Coefficients are stored low degree first.

The harness compares four ways to recover roots:

1. coefficient-only Cantor–Zassenhaus factorization;
2. recomputing every signed point sum;
3. Gray-code signed-sum enumeration;
4. meet-in-the-middle enumeration with a Montgomery batch inversion for the
   independent final additions.

Two experimental level-expansion methods are also available:

5. `tree`, which batches all affine denominators once per signed-sum level; and
6. `projective`, which keeps the expansion in Jacobian coordinates and performs
   one final batch normalization.

The structure-aware methods require the curve and originating `x_i` values (and a
square root of each curve right-hand side). They are not generic factorization
algorithms for an arbitrary coefficient vector.

## Production cryptanalysis integration

The maintained Rust implementation is exported as
`crypto_lib::cryptanalysis::prime_summation_roots` and is available through the
existing cryptanalysis CLI as `ic summation-roots`. It reuses the production
`prime_fast` Montgomery field and batched affine curve arithmetic; it does not
call the imported C++ harness.

```bash
ic --json summation-roots --p 1000003 --a 2 --b 3 \
  --x 409692,926728,748446 --coefficients

ic --json summation-roots --mode one --p 1000003 --a 2 --b 3 \
  --x 409692,926728,748446 --sign-mask 2

ic --json summation-roots --mode target --p 1000003 --a 2 --b 3 \
  --x 409692,926728,748446 --target-x 448482,17
```

The production interface keeps state allocation explicit through
`--max-states`, preserves finite-root multiplicity and roots at infinity, and
returns replayable sign-mask certificates for one-root and target hits. Optional
coefficient materialization supports quadratic or balanced product-tree
construction. These are correctness-oriented integration paths; the imported
timings remain exploratory and are not promoted by this implementation
`[pcc:unverified]`.

## Build and verify

```bash
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build
ctest --test-dir build --output-on-failure
```

On a minimal machine without CMake, use `make test`.

The self-test checks the `S3` discriminant identity, direct `S3` coefficients,
`S4` computed independently as a 4x4 Sylvester resultant, agreement of all root
methods, orientation/permutation invariance, generic factor recovery, equality
of quadratic and subproduct-tree coefficient construction through degree 512,
one-root certificates, and target-search hit/miss certificates.

## Reproduce the full campaign

```bash
./scripts/run_campaign.sh
```

Defaults generate 100,002 polynomials per field/curve pair (16,667 at each arity
3 through 8), or 900,018 total over `p = 1009, 1000003, 2147483647` and
`(a,b) = (2,3), (0,7), (5,11)`. Override the workload with
`SAMPLES_PER_ARITY`, `BENCH_CASES`, and `BENCH_REPEATS`.

Each JSONL record contains the field, curve, seed, arity, sample index, nominal
degree, roots at infinity, number of distinct finite roots, input x-coordinates,
and the complete coefficient vector. The campaign compresses each corpus and
records checksums, raw logs, summaries, benchmark conditions, and environment.

## Use the structure-aware root finder

Pass the fixed input x-coordinates with repeated `--x` arguments. `auto` uses the
serially measured crossover among affine-tree, projective-tree, reusable-workspace,
and MITM paths for the three campaign fields; other fields use the affine tree.
These are implementation defaults for the recorded CPU, so explicit methods
remain available for target-specific calibration.

```bash
./build/semaev_lab roots --prime 1000003 --a 2 --b 3 \
  --x 409692 --x 926728 --x 748446 --method auto --emit-coefficients
```

Every supplied x-coordinate must lift to an `F_p` point on the named curve. The
chosen square-root signs do not affect the root multiset. The JSON response keeps
root multiplicities and reports projective roots at infinity separately.

Use `--method tree`, `projective`, `workspace`, or `mitm` to select a path
explicitly. The exact `auto` cutovers and their evidence are recorded in
`research/final_optimization.md`.

When coefficients are requested, `--coefficient-method auto|quadratic|product`
selects their construction. `auto` retains the simple quadratic update below
degree 256 and uses a Karatsuba subproduct tree from degree 256 upward. To measure
coefficient construction without charging root generation, run:

```bash
./build/semaev_lab coeff-benchmark --prime 1000003 --a 2 --b 3 \
  --min-degree 2 --max-degree 4096 --bench-cases 5 --bench-repeats 3 \
  --output results/optimization/product_tree_p1000003.csv
```

The automatic crossover is a conservative portable default, not a per-machine
autotuner. See `research/product_tree.md` for validation, provisional measurements,
and accounting boundaries.

For primes below `2^32`, field multiplication automatically uses a division-free
Barrett reducer; `p=2^31-1` uses a Mersenne fold. Arbitrary supported primes keep
the original 128-bit generic fallback. Pass `--reduction generic` to `generate`,
`benchmark`, or `roots` for matched reducer comparisons and diagnostics.

For repeated calls in an embedding application, `--method workspace` exercises
the allocation-reusing MITM implementation. The CLI itself makes one call, so
the reusable-workspace benefit is primarily exposed through the C++
`roots_mitm_workspace_into` API and batch benchmarks. See
`research/workspace_simd.md` for provisional measurements and SIMD blockers.

### One root without enumeration

If the caller needs any root rather than the complete root set, one signed point
sum is sufficient and costs only `m-2` point additions:

```bash
./build/semaev_lab one-root --prime 1000003 --a 2 --b 3 \
  --x 409692 --x 926728 --x 748446
```

The optional `--sign-mask` selects signs for the second and later input points.
An identity sum is reported as an exact projective root at infinity.

### Search only a target x-coordinate set

`target-root` hashes a signed right-half table and probes it with batched
`±Q-L` point differences, avoiding coefficient materialization and full root
listing:

```bash
./build/semaev_lab target-root --prime 1000003 --a 2 --b 3 \
  --x 409692 --x 926728 --x 748446 \
  --target-x 206671 --target-x 42
```

Unliftable targets are exact misses. A hit includes a sign-mask certificate.
This path can lose to full enumeration for large target sets; use
`benchmark-target` to measure the workload-specific crossover.

The maintained Rust `ic summation-roots --mode target` path additionally folds
the right table by global negation: it stores one representative of `{R,-R}`
and reconstructs the omitted orientation by complementing the right sign-mask
bits. Its JSON report exposes nominal `right_signed_states` beside actual
`right_stored_states`; the stored count is half whenever the chosen right half
has a sign variable. This is a structural state reduction, not a wall-time
claim `[pcc:unverified]`.

## Audit curve automorphisms

For `j=0` and `j=1728` curves, audit whether a fixed input specialization is
actually invariant under the rational automorphism:

```bash
./build/semaev_lab automorphism-audit --prime 43 --a 0 --b 7 \
  --x 2 --x 12 --x 29
./build/semaev_lab roots --prime 43 --a 0 --b 7 \
  --x 2 --x 12 --x 29 --method orbit --emit-coefficients
```

The audit distinguishes covariance to a transformed-input specialization from
valid within-specialization folding. See `research/automorphism.md` for exact
field gates, positive and negative controls, timings, and evidence obligations.

## Mathematical scope

For points `P_i = (x_i,y_i)`, fix the sign of `P_1`. Away from degree drops, the
monic specialized polynomial is

\[
\prod_{(\epsilon_2,\ldots,\epsilon_{m-1})\in\{\pm1\}^{m-2}}
\left(X-x\left(P_1+\sum_{i=2}^{m-1}\epsilon_iP_i\right)\right).
\]

Changing a chosen `y_i` sign or permuting the inputs only permutes this multiset.
An infinite signed sum is a projective root at infinity and lowers the affine
degree. Equal x-coordinates create repeated roots. The small-field campaign is a
deliberate failure control for algorithms that silently assume full degree or
square-freeness.

The formula above identifies roots for point-origin inputs; it does not establish
an end-to-end index-calculus or ECDLP speedup `[pcc:unverified]`. See `REPORT.md` for measured scope,
limitations, and the resulting algorithm design.

The integrated follow-up campaign, including all pursued optimization leads and
serial final measurements, is in `research/final_optimization.md`.
