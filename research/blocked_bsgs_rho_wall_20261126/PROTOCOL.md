# Frozen protocol: paired blocked BSGS versus rho

Protocol ID: `blocked-bsgs-rho-wall/v1`

Freeze date: 2026-11-26

Primary comparison: cold, one target, one CPU thread

Status at freeze: no harness timing has run

## 1. Question and allowed conclusion

On each fixed public curve below, does the existing paired x-coordinate blocked
BSGS implementation have lower elapsed time than the existing tuned batched
negation-map Pollard rho implementation when both solve and verify the same one
target and BSGS's table build is charged in full?

The strongest possible conclusion is a host-, compiler-, implementation-, and
size-specific constant-factor timing result. This protocol cannot establish a
smaller asymptotic exponent, a memory-neutral change, a universal break of
rho, or applicability to production keys. BSGS uses `O(sqrt(n))` memory while
rho has much smaller state. No external target or curve is accepted by the
harness.

## 2. Pinned solver sources

Repository base: `a2c2c400` (`origin/main` at task start).

| source | SHA-256 |
|---|---|
| `src/cryptanalysis/bsgs_fast.rs` | `ebe890abcbdcfb2e719eff99e046847f4f1efb0864f627ba2ad2a073a5f4a7bb` |
| `src/cryptanalysis/prime_fast.rs` | `6a9dc015b1558045935912688beebf141a6b4161c045c821faac81fc3a430bc1` |
| `examples/lit2026_w11_bsgs_time.rs` | `4c4d274b01e6784a905ecd6f2bc563eb1b45884bcc05cc6365152c539bc00595` |

The study crate must include the two solver modules from those repository paths,
not copied or modified solver bodies. A pre-run build manifest will pin the
study commit, lockfile, compiler, executable hash, source hashes, and host. A
source-hash mismatch is a hard refusal.

## 3. Fixed public curves

All five groups have prime order and cofactor one. The source is
`docs/curves/registry.json`; `toy40`'s generator is pinned by
`gpu/ecc/specs/ecbench-toy40.json` and `bsgs_fast::toy40_params`.

| curve slug | p | a | b | Gx | Gy | n |
|---|---:|---:|---:|---:|---:|---:|
| `icv1-fp24-tm5099-a6f137c6` | 16777199 | 16777196 | 31 | 2 | 5412647 | 16782299 |
| `icv1-fp28-t28201-65099091` | 268435399 | 268435396 | 3 | 1 | 1 | 268407199 |
| `icv1-fp32-t7319-90da43e9` | 4294967291 | 4294967288 | 31 | 2 | 2120051858 | 4294959973 |
| `icv1-fp36-t454193-44a20818` | 68719476731 | 68719476728 | 33 | 2 | 51741851039 | 68719022539 |
| `icv1-fp40-tm506303-75549727` | 649522587953 | 552810359288 | 555863716954 | 549251065028 | 609044633619 | 649523094257 |

Before timing either arm, construct `Q = [k]G` independently in both solver
representations and require equal canonical coordinates. Target construction is
common known-answer fixture preparation and is outside both charged intervals.
The scalar is retained by the evaluator and is never passed to either solve
call.

## 4. Frozen arms and charged intervals

### BSGS

- `BsgsFastPlan::new(n, 0, true, 1, 8)`.
- Explicit `StepKernel::Paired` and `pair_k = 32`, carried from the prior W11
  study rather than tuned on these rows.
- Cold time starts immediately before `BsgsFast::build` and includes table
  allocation/build, `solve`, its candidate verification, and a separately
  timed evaluator check that the returned scalar regenerates `Q`.
- Record build, search, evaluator verification, total, plan table bytes, and
  every `BsgsFastStats` field.

### Rho

- `RhoConfig::tuned(n)`, then fix `threads = 1`, the derived per-row walk seed,
  and `max_steps = 64 * ceil_sqrt(n)`.
- Time starts immediately before `rho_parallel`; it includes the target-
  dependent r-adding table, starts and restarts, batched walk, DP storage,
  collision recovery, its internal scalar-multiplication check, and a separately
  timed evaluator check that the returned scalar regenerates `Q`.
- Record outer elapsed time, the implementation's inner `wall_secs`, every
  `RhoResult` field, and every `RhoOpCounts` field.

Allocation and deallocation needed by an arm are charged. Process launch,
fixture construction, source verification, and output serialization are outside
both intervals. The implementations use different Montgomery backends; this is
an end-to-end implementation comparison, not an isolated field-kernel test.

## 5. Trials and schedule

For each curve:

- Four untimed warm-up pairs use dedicated warm-up stream identities.
- Primary panel: 512 independent target/walk replicate IDs `0..511`.
- BSGS A/A: 64 replicate IDs `0..63`; two identical cold BSGS invocations on
  the same target.
- Rho A/A: 64 replicate IDs `0..63`; two identical rho invocations with the
  same target and walk seed.
- Secondary amortized panel: 16 batches, each with 64 independently derived
  targets. One BSGS table is built per batch and serves the 64 targets;
  comparison is against 64 sequential one-thread rho invocations. This panel is
  labeled `amortized-B64` and never substitutes for the primary result.

Within the primary panel, even replicate IDs run BSGS then rho and odd IDs run
rho then BSGS. A/A invocation order is likewise alternated. No trial is rerolled,
dropped, or silently repeated. A solver failure, cap, timeout, panic, malformed
row, or incorrect scalar is retained with its status. Any censoring at a size
blocks an unconditional positive timing result at that size; a capped time may be shown
only as a bound.

The raw log is event-sourced. Immediately before each charged arm, flush an
`intent` row naming the curve, panel, replicate, item, arm, invocation label,
target-seed identity, rho-seed identity where applicable, and schedule position.
Flush one `result` row after the arm returns. An intent without exactly one
matching result is a censored arm, including process death or an external
timeout; it is never interpreted as an arm that did not run.

## 6. Randomness

`seed-manifest.json` freezes the 256-bit master seed, exact replicate ranges,
stream names, encoding, and SHA-256 derivation. Before timing, the native study
tool must emit the complete derived mapping for all target, rho-walk, scheduling,
and warm-up streams. That mapping is committed and hash-pinned; duplicate
derived seeds are a hard refusal. Bootstrap selections are too numerous for a
line-per-draw manifest, so the native seed emitter writes, per curve and panel,
the SHA-256 of the complete ordered little-endian selection-index stream. The
analyzer and independent verifier must reproduce those digests before reading
timings.

Target scalars are `1 + (u64_le(digest[0..8]) mod (n - 1))`. Rho seeds use the
same extraction without reduction. Different stream labels make target and walk
seeds independent. Scheduling is fixed by parity, not drawn at run time.

## 7. Execution and admission

- Rust release build with the committed lockfile; `RAYON_NUM_THREADS=1`.
- One admitted isolated x86-64 CPU, pinned by the repository's isolation
  runner; no concurrent build, test, agent benchmark, or profiler.
- Record CPU model/features, OS, memory, cgroup limits, compiler and Cargo
  versions, binary hash, affinity, load/PSI, context switches, faults, and
  contention status.
- Preferred backend is IsoLab EKS Spot native-L2 under repository policy. A
  finite request will cap the job at 2 vCPU, 4 GiB RAM, 60 minutes, two total
  attempts, and USD 2.00. If authenticated admission, checkpoint publication,
  or benchmark-record storage is unavailable, only compile/correctness work may
  proceed; local timing is smoke data and cannot support the primary conclusion.
- Before the first timed row: push the exact study commit, complete and push the
  repository checkpoint, open a `bench.record/v1`, and verify that raw-result
  storage is writable. Do not time first and reconstruct these records later.

## 8. Analysis fixed before outcomes

The primary estimand at each size is

`R = sum(rho cold correctness-checked total) / sum(BSGS cold correctness-checked total)`.

Also report the arithmetic mean paired difference and `T / sqrt(n)` for both
arms. A native analyzer performs 10,000 percentile-bootstrap resamples of whole
pairs using the frozen bootstrap stream and reports a two-sided 95% interval.
Medians and geometric means are secondary diagnostics only.

For bootstrap replicate `b = 0..9999` and draw `d = 0..N-1`, apply the seed
manifest derivation with `panel = "analysis"`, `replicate = b`, `item = d`, and
the stream label `bootstrap-primary`, `bootstrap-bsgs-aa`, `bootstrap-rho-aa`,
or `bootstrap-amortized`; select whole observation `derived_u64 mod N`. Sort the
10,000 resampled statistics and use the type-1 quantiles at zero-based indices
249 and 9749. A/A uses ordered invocation-2/invocation-1 ratios. Amortized
resampling selects whole 64-target batches.

- A positive timing result is reported only if every scalar passes the evaluator
  check, no row is censored,
  the primary interval's lower endpoint is greater than 1.0, and both A/A
  intervals contain 1.0.
- A gain of at least 10% is reported only if the lower endpoint exceeds 1.10.
- Otherwise the row is unresolved or a loss as the interval dictates.
- Report all five sizes separately. Do not pool sizes into a single conclusion.
- Report exact table bytes, process peak RSS where available, rho DP counts,
  operation counters, caps/failures, and the cold/amortized distinction.
- No fitted asymptotic exponent claim is permitted from this panel.

An independent verifier must reparse raw rows, regenerate every target from the
committed derived-seed mapping, check every scalar in independent affine group
arithmetic, recompute summaries and intervals, and sign or otherwise retain its
report before any result is promoted. Producer output and Daybreak review alone
remain `[pcc:unverified]`.

## 9. Stop and falsification conditions

Stop the timed job after 60 minutes, 4 GiB resident memory, or the declared
compute budget. Preserve partial rows. Stop and reject the study before timing
on any source/manifest/binary mismatch, duplicate seed, invalid curve, unequal
target coordinates, thread-count mismatch, or unwritable evidence store.

The hypothesis “paired blocked BSGS has lower elapsed time on a curve under this protocol”
is falsified by `R <= 1`, by an interval reaching 1, by a failed A/A gate, or by
censoring that prevents the required estimate. A secondary amortized result does
not rescue a failed cold one-target hypothesis.
