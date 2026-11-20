# L1 step 1: factorization-pattern tally of the descent sextic

Lane key `l1-pattern-tally`, 2026-11-19. Every number here is a producer
observation, labelled [pcc:unverified]. No pcc claim was registered. No paid
compute, Runpod, EKS or remote side effect was used. This is finite algebra
on reconstructed inputs: counted outcomes, not wall time, and not a descent
saving.

## Question

Lead L1 (`../scout-graph-ic/LEADS.md`) proposes a change to the descent. It
would also accept `F_p`-solutions whose monic sextic `S = (C²H − A²)/u_R`
factors as `(1^4,2)`, `(1^2,2^2)` or `(2^3)`, provided the quadratic factors
are exact-log classes of the deferred graph. Its yield model
`m(f) = 1 + 15f + 45f² + 15f³` assumes that these patterns occur at the
random-sextic cycle-type densities 1/48, 1/16 and 1/48, against 1/720 for
`(1^6)`. Step 1 tests that assumption alone, on the recorded descent streams.
It does not measure `f`, it does not test whether a quadratic factor
materializes as a valid class `(U, V)` with a sign, and it does not count any
cost.

## Prediction and falsification criterion (LEADS.md L1, as written)

For the 3,419 `F_p`-solutions of the Run50 stream (seed 389, 3,415
residuals), LEADS.md predicts:

| Pattern | Predicted count |
|---|---:|
| `(1^4,2)` | ≈ 71 |
| `(1^2,2^2)` | ≈ 214 |
| `(2^3)` | ≈ 71 |
| `(1^6)` | ≈ 5 (observed 2) |

The criterion, applied as written: "If `(1^4,2)` ≤ 35 or `(1^2,2^2)` ≤ 110,
the 15x and 45x multipliers are wrong by at least 2x." Either condition
kills the yield model. The nagao-e2e stream (seed 401) is a secondary
replicate, and its predictions are scaled to its own solution count. The
criterion is applied only to the Run50 stream, as LEADS.md states it.

## Method

`pattern_tally.rs` reconstructs each stream the same way
`descent-finder/traced_descent_check.rs` does: same public transfer, same
factor-base and stream seeds, same per-residual RNG seeds. It runs the
production descent unchanged: staircase-64 F4 without trace, then the
`LinMode::Fast` linear stage, through `cover::nagao_decompose_using` with a
recording `NagaoSolver` callback.

For every `F_p`-solution, the tool evaluates `S`'s coefficients with
`WeilForms::eval`, exactly as the materializer does. It then classifies the
factorization of `S` over `F_p` in two independent ways:

- **(a)** a squarefree test, `gcd(S, S')`, followed by distinct-degree
  factorization;
- **(b)** a brute-force count of roots over all of `F_p`, plus the Berlekamp
  count `6 − rank(Q − I)` of irreducible factors.

For a squarefree sextic, the pair `(roots, factors)` determines each of the
four target patterns uniquely. Every solution's result from (a) is checked
against (b). The intervals are exact Clopper–Pearson 95% intervals on the
per-solution rate.

Controls, all of which passed:

- **Solutions per residual.** The `(a, b)` and the solution count of each
  residual equal descent-finder's recorded rows
  (`../descent-finder/runs/traced-seed389-n3415.rows.jsonl.gz`) on all
  3,415 residuals, with 0 mismatches. The totals are 3,419 and 1,452
  solutions, the same as descent-finder.
- **Decomposing residuals.** The pipeline's own decompositions fall at
  residuals 263 and 3414 (seed 389) and 997 and 1505 (seed 401), as recorded.
- **Sextic coefficients.** No sextic had a coefficient outside `F_p`
  (`non_fp_sextic = 0`).
- **Method agreement.** Methods (a) and (b) disagree on 0 solutions.

## Commands run

```
df -h /                                   # 13G free before the build
cd build && SCRATCH=$S OUT=$S/bin ./build.sh > build.log
#   RUSTUP_TOOLCHAIN=1.98, CARGO_TARGET_DIR=/home/user/cargo-target-round2,
#   CARGO_INCREMENTAL=0, cargo -j 1; 1m03s; rustc 1.98.1
$S/bin/pattern_tally ../../runpod-deferred-pair-20261117-50/public.json 389 3415 runs/tally-seed389-n3415.rows.jsonl   # 29.8 s, one thread
$S/bin/pattern_tally ../../nagao-e2e/public.json 401 1506 runs/tally-seed401-n1506.rows.jsonl                          # 7.9 s
```

`$S` is a session scratch directory. The build boundary is identical to
`../descent-walltime/build`: the frozen `fcda1e22` descent closure, the
nagao-e2e `jv_cover` overlay, and byte-copied shims. `fetch_frozen.sh`,
`assemble.py` and `Cargo.lock` are copied unchanged. Source and binary
SHA-256 values are in `build/build.log`. The binary's SHA-256 is
`3f87175c…74df`. The run outputs' hashes are in `runs/RAW.sha256`.

**Custody note (artifact policy).** `runs/tally-seed389-n3415.rows.jsonl.gz`
(92,524 bytes, SHA-256 `60401c8f036e0dbf6e16364ca45b2cae99d3e3827837898452f9374c364bbf0e`,
listed in `runs/RAW.sha256`) exceeds the 64 KiB data-file limit in
`tools/artifacts/artifacts.py`, so it is not committed to Git. It belongs in the
artifact store, but the store push needs AWS credentials, which are currently
expired. Until then it is LOCAL only, and it is reproducible: rebuild with
`build/build.sh`, rerun the seed-389 command above (deterministic, ~30 s), gzip
it and compare against the recorded hash. The seed-401 rows file (42 KB) and
both summaries stay in Git.

## Results [pcc:unverified]

### Run50 stream (seed 389): 3,415 residuals, 3,419 solutions

| Pattern | Observed | Predicted (density × 3,419) | 95% CI on count | Observed / predicted (95% CI) |
|---|---:|---:|---:|---:|
| `(1^4,2)` | **69** | 71.2 | 53.8 – 87.1 | 0.97 (0.76 – 1.22) |
| `(1^2,2^2)` | **238** | 213.7 | 209.6 – 268.9 | 1.11 (0.98 – 1.26) |
| `(2^3)` | **73** | 71.2 | 57.3 – 91.5 | 1.02 (0.81 – 1.29) |
| `(1^6)` | 2 | 4.7 | 0.2 – 7.2 | 0.42 (0.05 – 1.52) |
| other patterns | 3,035 | 3,058.1 | | |
| non-squarefree | 2 | | | |

### nagao-e2e stream (seed 401): 1,506 residuals, 1,452 solutions

| Pattern | Observed | Predicted (density × 1,452) | 95% CI on count | Observed / predicted (95% CI) |
|---|---:|---:|---:|---:|
| `(1^4,2)` | 26 | 30.2 | 17.0 – 37.9 | 0.86 (0.56 – 1.25) |
| `(1^2,2^2)` | 95 | 90.8 | 77.3 – 115.3 | 1.05 (0.85 – 1.27) |
| `(2^3)` | 46 | 30.2 | 33.8 – 61.0 | 1.52 (1.12 – 2.02) |
| `(1^6)` | 2 | 2.0 | 0.2 – 7.2 | 0.99 (0.12 – 3.58) |

Both streams together give 95 `(1^4,2)` against 101.5 predicted, 333
`(1^2,2^2)` against 304.5, 119 `(2^3)` against 101.5, and 4 `(1^6)` against
6.8. The pooled figures are simple sums, with no pooled interval computed.

The other seven partitions of 6 are tabulated in the summary JSONs. Across
both streams there are 22 pattern intervals. Three of them exclude ratio 1,
against about 1.1 expected at 95%:

- seed 389 `(1^2,4)`: 0.90 (0.82 – 0.99);
- seed 401 `(2,4)`: 1.17 (1.03 – 1.32);
- seed 401 `(2^3)`: 1.52 (1.12 – 2.02).

The seed-401 `(2^3)` excess favours L1. It has not been replicated, and
no multiple-comparison correction was applied.

Residuals with at least one target-pattern solution: 359 of 3,415 (seed 389)
and 160 of 1,506 (seed 401).

Secondary: on both streams, every linear root of every sextic lies in the
509-element descent factor base. This is vacuous for `(2^3)`. Roots of `S`
over `F_p` therefore never fail on support. They can still fail on ordinate
consistency, which this tool does not test.

## Verdict: KEEP. Step 1 passes and L1's pattern-frequency assumption holds.

Under the criterion as written, on the Run50 stream:

- `(1^4,2)` = 69, which is greater than 35;
- `(1^2,2^2)` = 238, which is greater than 110.

Neither kill condition is met. The lower 95% bounds, 53.8 and 209.6, are also
above both thresholds. The observed rates match the random-sextic densities
that give the coefficients 15, 45 and 15 in `m(f)`, to within about ±25% at
95%.

What this does **not** establish:

- **No value of `f`.** `f = |D_exact| / |D_universe|` is still a model,
  about 0.079.
- **No match rate.** No partial was matched against an exact-log class.
- **No materialization check.** The tool did not test that `V = −A/C mod U`
  gives a valid, signed class.
- **No descent saving.** Nothing here is a descent saving in counted or wall
  terms.

Per the critic (CRITIC_ROUND2 F6), the pattern rates here should not be
multiplied with `f` estimates or with C1's counted ratio to quote a combined
descent figure. L1's step-3 kill/promote criterion is about matched rows per
solution, and it remains the deciding test.

## Next step

L1 step 2 is the exact count of `|D_universe|`. Enumerate the monic
irreducible `U` over `F_1009` and count the `V` with `V² ≡ H mod U` on the
mini-build's `jv_cover` cover, in Rust, in seconds. If `|D_universe| > 5e5`,
`f` halves. After that comes step 3: the exact-log class set at the deferred
graph's kernel checkpoint, and the match count of this tally's
`(1^4,2)`, `(1^2,2^2)` and `(2^3)` partials. The per-residual rows in
`runs/*.rows.jsonl.gz` list each solution's pattern for that lookup. Step 3
still needs Run50's graph state, or a local deferred collection.

## Files

- `pattern_tally.rs`: the tally, Rust.
- `build/`: the scoped build (`build.sh`, plus copies of descent-walltime's
  `fetch_frozen.sh`, `assemble.py`, `Cargo.lock` and `Cargo.toml.in`) and
  its `build.log` with hashes.
- `runs/tally-seed389-n3415.{summary.json,rows.jsonl.gz}` and
  `runs/tally-seed401-n1506.{summary.json,rows.jsonl.gz}`, plus
  `runs/RAW.sha256`.

## Errors and deviations

- `/usr/bin/time` is absent on this host. The first launch attempt exited 127
  before running, and its empty outputs were overwritten. Timings use the
  bash `time` builtin.
- The cross-check of per-residual solution counts against descent-finder's
  recorded rows was a one-line Python comparison. This was tooling, not
  science.
- No research-kb query was made. Step 1 makes no novelty or literature claim.
- No visual report or PDF, and no `bench.record`: there was no timing or
  benchmark run. This was left for the lane's PR owner.
