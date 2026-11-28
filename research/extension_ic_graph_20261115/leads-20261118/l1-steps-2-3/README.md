# L1 steps 2 and 3: class universe and matched partial rows

Lane key `l1-steps-2-3`, 2026-11-19. Every number here is a producer
observation, labelled [pcc:unverified]. No pcc claim was registered. No paid
compute, Runpod, EKS or remote side effect was used. All science is Rust: three
binaries built in one scoped local build. This is finite algebra and counted
outcomes. It is not a timing result, and it is not a descent saving. Following
critic F6 (`../CRITIC_ROUND2.md`), no combined descent-saving figure is
computed.

## Questions and criteria (`../scout-graph-ic/LEADS.md` §L1, "Cheapest local falsification")

- **Step 2.** Count `|D_universe|` exactly: the monic irreducible `U` over
  `F_1009` and the `V` with `V² ≡ H mod U`, on the Run50 cover `H`. The rule
  as written: "If |D_universe| > 5e5, f halves and m(0.079) drops below 1.7."
- **Step 3.** Rerun Run50's deferred collection locally and export the
  anchored canonical classes at the kernel checkpoint (`D_exact`). Look up the
  quadratic partial classes of every `F_p`-solution of the step-1 stream
  (seed 389, 3,415 residuals, 3,419 solutions) in `D_exact`. The criterion as
  written: **KILL** if the matched rows per solution are below 1.5x the
  full-split rate; **PROMOTE** if they are at least 2x; otherwise
  inconclusive.

Exact correctness conditions applied to a matched row (LEADS.md L1 / C2):

- the pattern of `S` is `(1^4,2)`, `(1^2,2^2)` or `(2^3)`, with `S` squarefree;
- every quadratic factor `U = (x−r)(x−r^p)` satisfies:
  - `C(r), C(r^p) ≠ 0` and `u_R(r), u_R(r^p) ≠ 0`;
  - no ramified point;
  - `V = −A/C mod U`, with `V² ≡ H mod U` checked by polynomial remainder;
  - a signed canonical key `[u0,u1,v0re,v0im,v1re,v1im]` (the lexicographic
    minimum of `±V`), present in `D_exact`;
- every linear root satisfies the materializer's conditions: it is in the
  descent base, `μ(x) ≠ 0`, `y = −A/C` with `y² = H(x)`, and `y = ±base.y`;
- the classes are pairwise distinct. Otherwise the coefficient would be doubled
  exactly. With `S` squarefree the `U` differ, so this never occurs. It was
  counted anyway: 0.

Each matched row was then checked in the group, beyond the count:

- the principal-divisor identity `R + ΣQ_i + ΣD_j = 0`, with Cantor
  arithmetic;
- for each matched class, its anchor identity `Σ small_i·B_i + D_key = 0`,
  where `small` is the integer small-base part of its exported anchor witness.

## Method

**Build** (`build/`). `fetch.sh` reads two trees with read-only `git show`:

- `fcda1e22`. Its 33 `src/` files listed in Run41's Root41 manifest
  (`../../validation/extension-pipeline-source.sha256`) match byte for byte.
  Run50's driver linked that Root41 library.
- Run50's recorded revision `11c9a031`. Its 11 deferred-pair `jv_*` modules
  are checked against `../../validation/deferred50-source.sha256`.

`assemble.py` builds a mini crate:

- the real `jv_sieve` closure (`two_large_prime` with its submodules,
  `large_prime_filter`, `f4_fp`, `zero_dim_la`, `jv_bitops`, `bitops`);
- the nagao-e2e `jv_cover` overlay. This is fcda1e22's `jv_cover` plus an
  optional solver callback. With no callback, the code path is the original;
- byte-copied item shims, as in step 1;
- the 11c9a031 modules, mounted under the same names and attributes as Run50's
  `examples/jv_sieve_deferred_pair_compare.rs`.

Run50's archived binaries and the Root41 rlib are LOCAL elsewhere and absent
here, so this is a rebuild, not the original binary.
`RUSTUP_TOOLCHAIN=1.98`, `CARGO_TARGET_DIR=/home/user/cargo-target-round2`,
`CARGO_INCREMENTAL=0`, `-j 1`, `--offline --release`, about 1 min; 13 GB free.
Hashes are in `build/build.log`.

**`run50_deferred.rs`** is the "deferred-pairs" arm of Run50's worker, trimmed
to that one method. It keeps:

- the same `JV_*` environment pinning;
- a one-thread pool;
- the bitops claim;
- the `Model` phase around the public transfer;
- the options from Run50's argv: `b_count 1018081`, `split cap 524288`,
  `deferred-edge cap 262144`, `accepted-descent cap 32`.

`D_exact` is every graph `anchor` event in `collection.sources[].graph`.
Collection stops at the complete batch that certifies the sparse kernel, so
these are the anchors at the kernel checkpoint. Every anchor witness is
replayed for integer incidence: the large part is exactly `+D` and every other
class cancels. Its small part is exported.

**`universe_count.rs`** reads `H` from the rerun's collection record and
requires it to equal the library's `ctx.cov.hx`. It checks all 509 recorded
factor-base points on `y² = H(x)` with its own `F_{p²}`. For every monic
irreducible `U` it counts the `V` in two ways that must agree:

- **(a)** the character product `(1+χ(H(r)))(1+χ(H(r^p)))`;
- **(b)** explicit square roots, from a brute-force table of `F_{p²}`, and
  interpolation of `V`, then a polynomial check that `V² − H ≡ 0 mod U`.

It also independently checks that every `D_exact` key is a valid Mumford pair.

**`partial_match.rs`** reconstructs the descent stream exactly as
`../l1-pattern-tally/pattern_tally.rs` does. For each solution it:

- finds the linear and quadratic factors of `S` by brute force, over `F_p` and
  over `F_{p²}∖F_p`;
- rebuilds `A` and `C` with its own arithmetic and checks
  `C²H − A² = u_R·S` over `F_{p²}`;
- applies the conditions above.

`ffield.rs` holds that own `F_p` / `F_{p²}` code. The arithmetic does not use
the library's field types; it uses the same presentation `w² = 11`, which is
checked against the library.

## Commands

```
df -h /                                      # 13G free
cd build && SCRATCH=$S OUT=$S/bin ./build.sh > /dev/null   # ~1 min
P=../../runpod-deferred-pair-20261117-50/public.json
$S/bin/run50_deferred $P 389 8192 full.raw.json full.dexact.jsonl full.summary.json       # control, 55 s
$S/bin/run50_deferred $P 389 1 nodesc.raw.json nodesc.dexact.jsonl nodesc.summary.json   # without descent, 44 s
$S/bin/universe_count $P full.summary.json full.dexact.jsonl universe.json               # 2 s
$S/bin/partial_match $P 389 3415 3415 full.dexact.jsonl full.summary.json match-n3415.rows.jsonl match-n3415.summary.json     # 35 s
$S/bin/partial_match $P 389 30000 3415 full.dexact.jsonl full.summary.json match-n30000.rows.jsonl match-n30000.summary.json  # ~6 min
# step-1 rows regeneration (control), with ../l1-pattern-tally/build/build.sh:
$S/step1/bin/pattern_tally $P 389 3415 tally-seed389-n3415.rows.jsonl
```

`$S` = `/tmp/claude-0/-home-user/53beab17-35e6-5dd9-bf91-c8753a71ee81/scratchpad/l1-steps-2-3`.
Every run is deterministic. The large outputs (`full.raw.json`, 101 MB; the
`D_exact` export, 4.7 MB; the match row files, 0.5 and 4.7 MB) stay outside Git
under `$S/runs`. Their SHA-256 values are in `runs/RAW.sha256`, and they can be
regenerated with the commands above. The two raw JSONs carry rank-check
wall-clock fields, so only their scientific content is reproducible. The
`D_exact` export was byte-identical across the with-descent and without-descent runs.

## Controls [pcc:unverified]

Run50 reproduction (`runs/run50-control.json`) against
`runpod-deferred-pair-20261117-50/run-study.json` outcome `pair-deferred`:

- **All equal.** Every count field, all 15 graph stats (20,510 anchors,
  125,491 vertices, 77,994 deferred, 8,145 activated, 69,849 live, witnesses
  16/13), 90,877 sources, 518 projections, rank 508 at B 155,648, and the stop
  reason. Also 251,537 screen calls and 167,516 screen refusals, which equal
  Run50's early refusals.
- **Bitops.** The rank-check observed bitops (443,496,216,906) are equal. All
  nine phase bitops are equal, including descent 3,426,280,619,436. The
  subtotal is 5,913,875,255,822.
- **Descent and scalar.** 3,415 descent residuals, 2 accepted descents, and
  the scalar 242624768583643540, verified.
- **Without descent.** The run with `MAX_DESCENT_RESIDUALS=1` has identical
  collection phases and an identical `D_exact`.
- **Anchor replay.** 20,510 of 20,510 anchor witnesses replay with exact
  integer incidence.

The rerun reproduces Run50 exactly in every recorded number checked. Run50's
original raw witnesses are LOCAL elsewhere, so no byte comparison with them was
possible.

Further controls, all of which passed:

- **Recorded `H`.** The recorded `H` equals the library cover. All 509 base
  points lie on `y² = H(x)` in the tool's own field.
- **Universe methods.** Methods (a) and (b) disagree on 0 of the 508,536 `U`.
- **`D_exact` validity.** 20,510 of 20,510 `D_exact` keys are valid Mumford
  pairs, lie in the universe, and are canonical.
- **Step-1 stream.**
  - The regenerated step-1 rows' summary line equals the committed
    `../l1-pattern-tally/runs/tally-seed389-n3415.summary.json`.
  - This tool's per-residual patterns, `(a,b)` values, solution counts and
    decomposition counts equal those rows on all 3,415 residuals.
  - The `.gz` hash differs only by the gzip header, since the original was
    not written with `-n`.
- **Norm identity.** `C²H − A² = u_R·S` holds for every non-trivial solution.
- **Group checks.**
  - All full splits pass the group identity: 2 of 2, and 31 of 31 in the
    extension.
  - All matched rows pass the group identity: 6 of 6, and 57 of 57.
  - All matched classes pass their anchor identity: 6 of 6, and 67 of 67.
- **Clean outcomes.** 0 class failures, 0 duplicate-class rows, and 0
  linear-part failures among the target patterns.

## Results [pcc:unverified]

### Step 2: exact universe (`runs/universe.json`)

| Quantity | Value |
|---|---:|
| Monic irreducible `U` over `F_1009` | 508,536 (= (p²−p)/2) |
| `U` with 4 admissible `V` / with 0 | 127,265 / 381,271 (no `U` has 1 or 2) |
| Signed `(U,V)` pairs | **509,060** |
| Canonical classes up to sign (the graph's key unit) | **254,530** |
| `D_exact` (anchors at the kernel checkpoint) | 20,510 |
| `f = |D_exact| / |D_universe|` (both up to sign) | **0.0806** |

Exactly one `U` has a root with `H(r) = 0`. It has no admissible `V`.

**Step-2 verdict: the rule does not trigger, and `f` does not halve.**
`D_exact` is counted in canonical keys, so the comparable universe is 254,530,
which is below 5e5. LEADS.md's model estimate was about 2.5e5 classes and
`f ≈ 0.079`. The signed count, 509,060, is above 5e5, but in that unit
`D_exact` doubles too, to 41,020, so `f` is the same 0.0806. C2's heuristic
"≈ p²/2 ≈ 5·10^5" is the signed count.

### Step 3: matched rows (`runs/match-n3415.summary.json`, `runs/matched-rows-n3415.jsonl`)

On the prespecified stream (seed 389, 3,415 residuals, 3,419 solutions):

| | `(1^4,2)` | `(1^2,2^2)` | `(2^3)` | total |
|---|---:|---:|---:|---:|
| Solutions with the pattern | 69 | 238 | 73 | 380 |
| Quadratic classes formed | 69 | 476 | 219 | 764 |
| Classes in `D_exact` | 6 | 43 | 16 | 65 (8.5%) |
| **Matched rows** | **6** | **0** | **0** | **6** |

Comparison:

- Full splits `(1^6)`: **2**, both valid and both group-checked.
- Matched rows per solution: 6/3,419 = 1.75e-3. The exact Poisson 95% interval
  on the count is 2.2 to 13.1 rows.
- Full-split rate: 2/3,419 = 5.85e-4. The Poisson 95% interval is 0.24 to 7.2.
- **Ratio: 3.0x.** The conditional-binomial 95% interval for the rate ratio,
  from 6 of 8, is **0.54x to 30x**.
- The thresholds: KILL below 3 rows (1.5x), PROMOTE at 4 rows or more (2x).

**Step-3 verdict, applying the criterion exactly as written: PROMOTE.** The
point ratio, 3.0x, is at least 2x. The verdict is not robust: the 95% interval
covers both thresholds by a wide margin. The ratio rests on only 2 full splits,
where the random-sextic model expects 4.75 for 3,419 solutions. Against that
model rate the same 6 rows give 1.26x (95% 0.46x–2.75x).

### Extension, not prespecified (`runs/match-n30000.summary.json`)

The same target, the same `D_exact` and the same walk, continued to 30,000
residuals (30,250 solutions):

- **Target solutions.** `(1^4,2)` 623, `(1^2,2^2)` 1,929, `(2^3)` 596.
- **Matched rows: 57.** These are 47 `(1^4,2)` and 10 `(1^2,2^2)`; the
  `(1^2,2^2)` rows are the first two-class matches. The Poisson 95% interval
  is 43.2 to 73.9.
- **Full splits: 31.** All 31 are valid, and they equal the 31 pipeline
  decompositions.
- **Ratio: 1.84x.** The conditional-binomial 95% interval is **1.17x to
  2.95x**. That point estimate falls in the inconclusive band. Against the
  model full-split rate (42.0 expected) the ratio is 1.36x (95% 1.03x–1.76x).
- **Class match rate.** 482 of 6,269 classes are in `D_exact`, 7.7% (Wald 95%
  7.0–8.3%). That is consistent with `f = 8.06%`. By pattern: `(1^4,2)` 7.5%,
  `(1^2,2^2)` 8.0%, `(2^3)` 7.0%. Partial classes behave like uniform draws
  from the universe.
- **Observed vs model.** The observed `(1^6)` rate is 1/976 against the model's
  1/720, while `(1^4,2)` is 1/48.6 against 1/48. Two-class matches appear at
  10/1,929 = 0.52%, against `f² = 0.65%`.

At `f = 0.0806` the model's expected excess over the full-split rate is
`15f + 45f² + 15f³ = 1.51x`. The KILL line, 1.5x, therefore sits on the model's
own expectation. Against the model full-split rate this test cannot separate
L1's model from the kill outcome. Its decision rests mainly on the empirical
full-split rate of the stream.

## Verdict

- **Step 2:** |D_universe| = 254,530 canonical classes, or 509,060 signed
  pairs. The rule does not trigger, and `f = 0.0806` at Run50's kernel
  checkpoint.
- **Step 3, as written:** **PROMOTE.** Matched rows are 6 against 2 full
  splits, or 3.0x, with a 95% interval of 0.54x–30x.
- **Robustness:** weak. The non-prespecified 30,000-residual extension gives
  1.84x (95% 1.17x–2.95x), which is inconclusive. Against the random-sextic
  full-split rate it gives 1.36x.

The mechanism itself is confirmed. Partial classes match `D_exact` at the
predicted rate `f`. Matched rows are valid: they satisfy the group identity,
and their classes satisfy the anchor identity. Two-class rows exist at about
the predicted `f²` rate.

## Next step

Take the PROMOTE as written to an M3-style paired protocol only on its stated
terms:

- fresh targets;
- at least 100 pooled successes per arm (critic F4, item 3);
- the full affine path for matched rows: target coefficient `−b`, RHS `a`,
  kernel projection and elliptic replay, none of which were run here.

The extension suggests the measured gain per solution is about 1.4–1.8x the
full-split rate at `f = 0.08`. L2 (more anchors after rank readiness) is the
lever that moves `f`. A preregistered decision on the measured full-split rate
versus the model rate should come before any further L1 claim.

## Files

- `run50_deferred.rs`, `universe_count.rs`, `partial_match.rs`, `ffield.rs`:
  the Rust tools.
- `build/`: the build scripts, plus `extract_helper.py`, copied from the step-1
  build, and `Cargo.lock` and `build.log` with source and binary SHA-256
  values. Binary SHA-256: `run50_deferred 6a721985…`,
  `universe_count 5b1af9df…`, `partial_match 9a4e8b33…`.
- `runs/`: `run50-control.json`, `universe.json`,
  `match-n3415.summary.json`, `match-n30000.summary.json`,
  `matched-rows-n3415.jsonl`, `matched-rows-n30000.jsonl` and `RAW.sha256`.
  Each file is under 64 KiB.

## Errors and deviations

- **Rebuild, not the original binary.** Run50's worker binary and the Root41
  rlib are not on this host. The rebuild uses Root41-equivalent sources
  (manifest-checked) and the nagao-e2e `jv_cover` overlay, a strict superset
  whose callback-free path is unchanged. The exact reproduction of every
  recorded count and phase bitop is the evidence that the behaviour matches.
- **Trimmed driver.** The driver is a one-method trim of Run50's example, not
  the example itself.
- **Anchor export format.** In the first build, the anchor export lacked the
  integer small part. It was added, and the tool was rebuilt and rerun; the
  graph outputs did not change.
- **`μ(x) = 0` roots.** A linear root with `μ(x) = 0` would be counted as a
  linear-part failure rather than resolved in the group. None occurred.
- **Glue in Python.** The control-comparison JSON, the matched-row extracts
  and the step-1 per-residual comparison were made with short Python glue
  (`python3 -I`, equality checks and extraction only).
- **No research-kb query.** No novelty or literature claim is made.
- **No report or record.** There is no visual report or PDF, and no
  `bench.record`, because there was no timing or benchmark run. Wall times
  above are practicality only. This is left for the lane's PR owner.
- **Clippy.** One unused-import warning (`Fld`) in `partial_match.rs` was left
  in place, so that the hashed sources match the binaries that ran.
