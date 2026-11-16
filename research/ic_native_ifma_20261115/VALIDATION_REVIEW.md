# Validation review: `research/ic_native_ifma_20261115/REPORT.md`

Validator (independent; did not write the code, the specs, the run or the report), 2026-11-15.
Standing of this review: `[pcc:unverified]` (no pcc claim is registered for it).

**Disposition: `validated` for the run, scoped as the report scopes it (one L0 run on a 4-thread VM,
wall-clock figures indicative), with one required correction: the report's sentence "Prediction 2
held at its threshold" is not supported by the artifacts and must be replaced (finding 1) before
the report or its record is cited.** [pcc:unverified]

What holds, recomputed with my own scripts and not with `analyze.py`:

- the provenance (protocol, specs and code pushed before the run; one binary throughout);
- all 96 discrete-log certificates, under my own elliptic-curve arithmetic;
- all 64 identity checks;
- every other number in the table, the bullets and the §5a band, except two `m = 3` cells that
  were computed from rounded medians (finding 2);
- the IFMA Montgomery kernel, on review and under a randomized u128 test and 13 mutations.

What does not hold is the threshold reading. At `m = 2`, log2 r = 27.5, four threads, the IFMA arm's
median relation phase is 1.4945× faster than the scalar native arm's. That is below the
preregistered 1.5×, so prediction 2 was missed at one of its four cells. The report's 1.497 and
its table cell 1.50× come from dividing medians already rounded to three decimals, and 1.497 is
itself below 1.5. Findings 2–10 are smaller corrections or disclosures. [pcc:unverified]

## What was validated

| item | value |
|---|---|
| validated objects | `research/ic_native_ifma_20261115/` and `benchmarks/records/20261115T2010Z-ic-native-ifma.json` at `d71bc17b0f` (the dispatch commit). These paths and `rootfind_gpu.rs` have no diff from `d71bc17b0f` to `b49a710717`, `3767aaca3f`, `fa5a380796` and `4926dbdc54`, the branch heads seen while I worked; the working-tree copies hash equal to the committed blobs. `REPORT.md` blob `fc154ae88a`, sha256 `6dfad63363d72f45…`; record sha256 `7115c4500ccc2f80…` |
| code under test | `src/cryptanalysis/ic_framework/rootfind_gpu.rs`, byte-identical at `1deabdf708`, `219b1378f8` and `d71bc17b0f` (sha256 `c973a84ca7bffb12…`); module `ifma` is lines 525–709 |
| raw tarball | `raw/out_ifma.tgz`: 82 072 B, sha256 `77afcb9475471416697dc20b0d77beae0b24e2f598c9a08d7c8cddd445827871`, equal to the record's `raw[0].sha256` and to the `77afcb94…` in REPORT.md; 31 entries, all under `out_ifma/`, no absolute or `..` paths. Extracted in scratch only |
| session bindings | For both sessions, `records.jsonl`, `plan.json` and `host.json` hash to `records_sha256`, `plan_sha256` and `host_sha256` in `session.json`. Each session has seq 0–47 contiguous, no warm-ups and 48/48 records `verified`. Every record is L0 with the single blocker "L1: no CPU reservation", affinity CPUs 0–3, env class `ECBENV3h54a8cfb9b559`. The order alternates by round (round 0 `cpu, native1, ifma1, native, ifma, rho`; round 1 reversed) |
| specs | `sessions/ifma2/spec.json` and `sessions/ifma3/spec.json` are byte-identical to `specs/ifma2.json` and `specs/ifma3.json` at `219b1378f8` and at `d71bc17b0f` (sha256 `db7d3892…`, `b56604f1…`). `PROTOCOL.md`, `analyze.py` and `run.sh` are unchanged since `219b1378f8` |
| binary | `binary_sha256 = 0afa6c10e2e4638f…` in all 96 records, both `session.json`, both audit receipts (`auditor_binary_sha256`), the tarball's `sha256.txt` and the local `target/release/ecbench` (mtime 20:10Z). That binary contains the IFMA refusal string (`simd=ifma needs AVX-512 IFMA on this host and p < 2^52`) and `ecbench_before_solve`, which entered the branch in the merge between `1deabdf708` and `219b1378f8` |
| band files | `band-ifma2.json` and `band-ifma3.json` in Git are byte-identical to the tarball's |
| tables | the producer's `analyze.py`, run on the extracted tarball, regenerates `RESULTS.md` and `results.json` byte for byte. My own scripts agree with them on every unrounded value (section 4) |

## Requirements

| # | requirement | result | evidence |
|--:|---|---|---|
| 1 | Provenance: protocol and specs pushed before the session started; specs equal; tarball hash; binary hash consistent | pass, with finding 3 | Section 1 below and the table above |
| 2 | Certificates: all 96 recovered logarithms re-verified with independent EC arithmetic | pass (96/96) | scratch `scripts/certs.py` |
| 3 | Identity: per workload and round, every IC arm against the one-thread arm | pass (64/64, 0 mismatches) | scratch `scripts/identity.py` |
| 4 | Every number in REPORT.md recomputed | pass for 25 of the table's 28 cells, the band values and the "against one thread" and solve ranges. **Fail** for the 4-thread `m = 2` low end (1.497 and 1.50×; it is 1.4945), with finding 1. Two `m = 3` cells and two ranges come from rounded medians (finding 2) | scratch `scripts/recompute.py`, `scripts/paired.py` |
| 5 | Code review of the IFMA kernel; randomized u128 test; mutation checks with `cargo test --release --lib rootfind_gpu` | pass, with findings 4 and 10 | sections "Code review" and "Tests and mutations" |
| 6 | Wording: L0 and indicative; the threshold result stated plainly | **fail** on the threshold (finding 1); pass on L0, indicative, `[pcc:unverified]` and no leaderboard row; findings 5 and 6 are wording | REPORT.md "Standing", "Results", "What this changes" |

### 1. Provenance

The remote-tracking reflog for `origin/claude/index-calculus-gpu-optimization-otouqv` gives the
push times:

| commit | what | committed | pushed |
|---|---|---|---|
| `1deabdf708` | the IFMA sweep (`rootfind_gpu.rs` only) | 20:02:08Z | 20:02:12Z |
| `219b1378f8` | `PROTOCOL.md`, specs, `run.sh`, `analyze.py` | 20:03:47Z | 20:03:50Z |
| `e96ec1b771` | the open benchmark record (record file only) | 20:11:16Z | 20:11:19Z |

The record was created at 20:10:44Z on `219b1378f8` with `commit_dirty: false`, and its single
command started at 20:10:52Z. The sessions ran 20:10:54Z–20:13:40Z (ifma2) and 20:16:30Z–20:17:38Z
(ifma3). The protocol, the specs and the code were therefore pushed about seven minutes before the
first session started. `1deabdf708` is an ancestor of `219b1378f8`.

### 2. Certificates

The two `prime_search` curves carry no `a` and `b`. For them I derived `a` and `b` from the
generator and the target. Every other target of the same curve then lies on the derived curve (24
records per curve, one `(a, b)` each). For the `prime_explicit` curves I read `a` and `b` from the
spec. For each curve I then checked the following with my own affine double-and-add:

- `p` and `r` are prime (Miller–Rabin with the first 13 prime bases).
- The cofactor is 1, and `p + 1 − r` equals the ICV1 trace and lies in the Hasse interval. With
  `r` prime and `r·G = O` below, `#E = r`: `r` is the only multiple of `r` in that interval.
- The discriminant is non-zero, and the j-invariant equals the ICV1 `j` field.
- `r·G = O`.

For all 96 records, rho included:

- `recovered·G = target`, with `0 ≤ recovered < r`;
- `recovered = planted`;
- `planted = 1 + derive_u64("uniform_scalar_sha256_v1", [model, r, target_seed, index]) mod (r − 1)`,
  recomputed with SHA-256 over the canonical JSON.

No check failed. [pcc:unverified]

| curve | p | a | b | r | logarithms |
|---|--:|--:|--:|--:|---|
| icv1-fp28-t7531-5f8c6474 | 187 834 783 | 178 200 012 | 134 334 653 | 187 827 253 | 153 372 092; 70 602 574 |
| icv1-fp30-tm15113-0b67ffcf | 1 062 943 379 | 804 230 764 | 730 864 256 | 1 062 958 493 | 484 743 770; 293 130 150 |
| icv1-fp34-t124571-a5bf1675 | 11 700 434 063 | p − 3 | 7 883 827 458 | 11 700 309 493 | 4 247 149 014; 1 398 758 306 |
| icv1-fp36-tm325323-0291967d | 39 012 606 793 | p − 3 | 22 067 677 030 | 39 012 932 117 | 17 181 862 920; 9 079 950 598 |

### 3. Identity

I grouped the records by (session, curve, target, round): 16 groups, each with all five IC arms.
In each group I compared every IC arm with `rf*-cpu-*` on the following, indexing every key with
`[]` so that a missing field fails:

- `counters.targets_tried` and `counters.relations_found`;
- the recovered logarithm and the status;
- `cost.s` and `cost.total_gae`;
- every phase's `(name, adds, doubles, scalar_mults, gae)`.

All 64 comparisons are equal. Beyond those fields I compared every leaf outside timing, isolation
and identifiers. The only other differences are the following:

- the five `gpu_*` relation-phase counters, which only the batched arms have (four at `m = 3`,
  where `gpu_cpu_fallback_targets` is absent);
- `fbm_id`;
- `work.complete` and `work.unpriced`, because the `bitop/v1` model lists the `gpu_*` counters as
  unpriced.

The `gpu_*` counters are equal across the four batched arms in every workload. Every phase native
and work counter, including `sweep_field_muls`, is equal to the one-thread arm's. `analyze.py`
this time reads the right phase keys (`adds`, `doubles`, `scalar_mults`, `gae`), unlike the
native-sweep study's script (that review's finding 2). [pcc:unverified]

### 4. Numbers

Ratios are over the one-thread `rootfind-subtract` arm's median relation-phase wall (median of 4).
"IFMA/scalar" is the scalar arm's median divided by the IFMA arm's, unrounded. [pcc:unverified]

| m | log2 r | one thread (s) | scalar ×1 | IFMA ×1 | scalar ×4 | IFMA ×4 | IFMA/scalar, 1 thread | IFMA/scalar, 4 threads | report |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|
| 2 | 27.48 | 2.0098 | 0.793 | 1.322 | 3.088 | 4.616 | 1.6675 | **1.4945** | 1.67×, **1.50×** |
| 2 | 29.99 | 11.4347 | 0.936 | 1.494 | 3.469 | 5.627 | 1.5966 | 1.6223 | 1.60×, 1.62× |
| 3 | 33.45 | 0.4360 | 0.516 | 0.746 | 2.092 | 2.630 | **1.4456** | **1.2570** | **1.44×, 1.25×** |
| 3 | 35.18 | 1.7361 | 0.946 | 1.291 | 3.493 | 4.536 | 1.3644 | 1.2987 | 1.36×, 1.30× |

The other cells, rounded to two decimals, equal the report's. The ranges in the bullets reproduce:

- IFMA ×4 against one thread: 4.616–5.627 at `m = 2` and 2.630–4.536 at `m = 3`;
- solve: 3.983–5.420 at `m = 2` and 1.182–2.010 at `m = 3`;
- the record's two metrics: 2.032 s against 3.297 s (1.622) and 7.654 s against 12.220 s (1.597).

Paired per (target, round), the IFMA/scalar relation-phase ratios are:

| m | log2 r | 1 thread | 4 threads |
|--:|--:|---|---|
| 2 | 27.48 | 1.551, 1.632, 1.636, 1.790 | **1.410**, 1.530, 1.587, 1.624 (median 1.558) |
| 2 | 29.99 | 1.568, 1.574, 1.595, 1.598 | 1.527, 1.602, 1.602, 1.681 |
| 3 | 33.45 | 1.258, 1.394, 1.567, 1.569 | 1.239, 1.271, 1.311, 1.432 |
| 3 | 35.18 | 1.250, 1.342, 1.388, 1.422 | 1.216, 1.295, 1.300, 1.310 |

My band follows band.rs's rule:

- `cost.unpriced` counts come from the top-level counters first, then from the phase native maps;
- memory-byte counters are exempt;
- counted `table_insert_attempts` is priced in the default reading.

It reproduces both band files' `mean_s` to a relative 1e-12 for every arm. The distances in the
report reproduce: `m = 2` 6 689.9 / 6 706.3 / 6 781.0 and 16 809.3 / 16 829.7 / 16 921.9; `m = 3`
213.0 / 295.5 / 668.4 and 150.2 / 170.9 / 264.4. They are the same on every IC arm. Without the
table-work term the `m = 3` distances at `w = 0.1 / 1` would be 287.8 / 625.7 and 169.0 / 253.6. The
report correctly uses the default reading. Every IC arm is above rho at every weight. IC at `w = 0`
over rho at `w = 1` is 3 391, 8 461, 106.9 and 75.2. [pcc:unverified]

## Code review of the IFMA kernel (`1deabdf708`)

- **Inputs below `p < 2^52`.** Every operand of the Montgomery product `mm` is below `p`:
  - Montgomery forms made by the u128 `mulmod`;
  - outputs of `mm`, `add` and `sub`;
  - `one_r = 2^52 mod p` and `one = 1`;
  - the output of `inv_lanes`.

  `Ifma52::new` refuses `p ≥ 2^52`, even `p` and `p < 5`, so `vpmadd52{lo,hi}uq`, which read the
  low 52 bits of each operand, see whole values.
- **Product and reduction.** The four multiply-adds compute the following:
  - `lo = ab mod 2^52` and `hi = ⌊ab/2^52⌋`;
  - `m = lo·pinv mod 2^52`, where `pinv = −p⁻¹ mod 2^52`. Six Newton steps from 1 give `p⁻¹ mod
    2^64`, which is then negated and masked;
  - `lo + (mp mod 2^52)`, a 64-bit accumulator. The sum is ≡ 0 mod 2^52 and below 2^53, so it is
    0 when `lo = 0` and exactly 2^52 otherwise.

  `c` (that sum shifted right by 52) is therefore exactly the carry out of the low halves. `u = hi
  + ⌊mp/2^52⌋ + c = (ab + mp)/2^52` exactly.
- **Bound and final subtraction.** `u < (p² + (2^52 − 1)p)/2^52 < 2p` because `p < 2^52`, so one
  conditional subtraction suffices. `min_epu64(u, u − p)` picks `u − p` exactly when `u ≥ p`
  (otherwise `u − p` wraps above 2^63). The output is in `[0, p)` and equals `ab·2^−52 mod p`.
- **`add` and `sub`.** `add` computes `s = a + b < 2p` and then `min(s, s − p)`. `sub` computes
  `d = a − b`, which wraps when `a < b`. In that case `d + p` wraps back to `p − (b − a) ∈ (0, p)`,
  the smaller value. When `a ≥ b`, `d < p ≤ d + p`. Both outputs are in `[0, p)`.
- **Batched inversion, per lane.**
  - Forward pass: `prefix[i] = d_0⋯d_i` in Montgomery form, where `d_i = x_R − x_i`, replaced by
    Montgomery 1 where it is 0. Every factor is non-zero mod the prime `p`, so the product is
    invertible.
  - `inv_lanes`: from `v = aR` it computes `v^(p−2) = a⁻¹R⁻¹`, multiplied by `r2 = R² mod p` with
    the plain `mulmod`. The result is `a⁻¹R`, the Montgomery form of `a⁻¹`.
  - Backward pass: at the entry to step `i`, `inv = P_i⁻¹`. The step sets `inv_i = P_i⁻¹·P_{i−1} =
    d_i⁻¹` and `inv ← P_{i−1}⁻¹`, then stores `prefix[i] = d_i⁻¹`. Step `i` reads only
    `prefix[i − 1]`, which no earlier step has overwritten. After the loop, `prefix[0] = P_0⁻¹ =
    d_0⁻¹`, which is correct for `k = 1` too.
- **Candidates and order.** For each `i`, the kernel computes the following:
  - `λ = (y_R + y_i)·d_i⁻¹` at position `2i` and `(y_R − y_i)·d_i⁻¹` at `2i + 1`;
  - `x = λ² − (x_R + x_i)`;
  - `mm(x, 1)`, which takes `x` out of Montgomery form into `[0, p)`.

  These are the slopes and positions of `NativeSweep::sweep_one` and of `decompose`'s
  confirmation. Each lane's hits are pushed in increasing `i`.
- **Degenerate rows.** `zm` marks the lanes with `x_R ≡ x_i`. Their `d` becomes Montgomery 1, and
  they are skipped in the probe loop, as `sweep_one` skips them. At the oracle level such targets
  never use the sweep: `decompose` hands a target with `x_R ∈ fb.x_index` to the CPU oracle whole.
- **Partial chunks.** Lanes `≥ n` repeat the last target. `contains` is still called for them, which
  wastes up to 7 lanes of probes in a batch's last chunk. `hits.truncate(n)` drops their hits.
  `par_chunks(8).map_init(…).flatten().collect()` keeps input order, so `sweep` returns one `Hits`
  per target in order. Each worker owns its `prefix` buffer (`map_init`).
- **Probe set.** With `simd` the set holds canonical abscissae: `from_mont` of the reps' `x` at
  `m = 2`, and the pair table's abscissae unchanged at `m = 3`. That is the form the kernel
  produces. The sentinel `u64::MAX` is never a canonical value.
- **Charges.** These are unchanged by construction. `decompose` charges misses up to each reported
  position and then re-derives and confirms the candidate on the CPU (`gpu_hits_confirmed_on_cpu`).
  A false negative would change the charge or the trials, and the identity checks would see it.

## Tests and mutations

I wrote two tests and added them, in my scratch copy only, as a module
`validator_ifma_tests` at the end of `rootfind_gpu.rs` (scratch `scripts/validator_ifma_tests.rs`).
Neither uses the producer's arithmetic as its reference.

- **A, the real kernel against u128.** This test calls `ifma::Ifma52::new` and `sweep_lanes` and
  records every value passed to `contains`. It compares that sequence, candidate by candidate, with
  `λ² − x_R − x_i` computed in u128 with extended-Euclid inverses. It also compares each lane's hits.
  - Moduli: 160 primes, namely 5, 7, 11, 13 and 1 000 003; the eight largest primes below 2^52
    (from 2^52 − 47 down); and three random primes of every size from 4 to 52 bits.
  - Inputs: 40 cases per prime, with 1–40 abscissae and 1–8 targets (every partial chunk).
    Values are drawn a third of the time from {0, 1, 2, p − 1, p − 2, p − 3, ⌊p/2⌋, ⌊p/2⌋ + 1}.
    A quarter of the targets have `x_R` equal to a base abscissa.
  - Duplicate abscissae occur, and one `prefix` buffer is reused across all calls.
  - Size: 6 400 cases, 2 035 324 candidates, 481 931 hits and 18 908 degenerate rows.
- **B, the arithmetic closures against u128.** This is a verbatim copy of `mm`, `add` and `sub`
  from `sweep_avx`. The test first asserts, via `include_str!`, that the source contains the same
  text (whitespace aside).
  - Moduli: 160 odd moduli, namely 5, 7, 11, 13 and 1 000 003; the 8 largest odd numbers below
    2^52; and three random odd moduli of every size from 4 to 52 bits. `pinv` comes from the
    extended Euclidean algorithm on 2^52, not from the producer's Newton loop.
  - Inputs: 400 × 8 lanes per modulus with the same edge values, plus every extreme against `p − 1`.
  - Checks: `mm = ab·2^−52 mod p`, `a + b mod p` and `a − b mod p` on 512 000 lanes, and the
    bound `((p − 1)² + (2^52 − 1)p)/2^52 < 2p` for each modulus.

On the unmodified code, `cargo test --release --lib rootfind_gpu` passes 12 of 12: the producer's
10, of which 6 GPU-worker tests print "skipped" and pass without `ICX_ROOTFIND_WORKER`, plus A and
B. A and B also pass on a standalone copy of the `ifma` module (lines 525–709, byte-identical). [pcc:unverified]

**Mutations.** I made one mutant schemata, `VALIDATOR_MUT=<n>` read once at run time (scratch
`scripts/mutate.py`, `mutation.diff`), and rebuilt the scratch library tests. For each mode I ran
`cargo test --release --lib rootfind_gpu -- --skip validator_ifma_montgomery_ops`. B is skipped
because the schemata rewrites the closures it is bound to; B ran on the unmodified build. As a
control, mode 3 was also applied as a literal one-line edit (`mutation-literal3.diff`) and rebuilt
(row "3, literal"). Nothing was changed in the repository's working tree.

| mode | mutation | `ifma_lanes_match_scalar_sweep` | `native_ifma_matches_rootfind_subtract` | validator A | reading |
|--:|---|---|---|---|---|
| 0 | none | ok | ok | ok | baseline, 11/11 |
| 1 | `mm`: carry `c` dropped from `u` | caught | caught | caught | |
| 2 | `mm`: `min` → `max` | caught | caught | caught | |
| 3 | `mm`: final conditional subtraction removed | **survives** | **survives** | caught at `p = 2^52 − 47` | wrong only for `p > 2^51` (below) |
| 3, literal | same, as a source edit | survives | survives | caught at `p = 2^52 − 47` | control: same result as the schemata |
| 4 | `add`: `min` → `max` | caught | caught | caught | |
| 5 | `sub`: `min` → `max` | caught | caught | caught | |
| 6 | backward pass: `(2..k)` for `(1..k)` | caught | caught | caught | |
| 7 | backward pass: `prefix[i]` for `prefix[i − 1]` | caught | caught | caught | |
| 8 | degenerate lanes not skipped | survives | survives | caught (extra `contains` calls) | equivalent for the oracle: such targets go to the CPU whole |
| 9 | partial chunk not truncated | caught (`len 31`) | survives | caught | equivalent for the oracle: `zip` with the batch drops the extra lanes of the last chunk |
| 10 | scalar inversion without `r2` | caught | caught | caught | |
| 11 | candidate left in Montgomery form | caught | caught | caught | |
| 12 | `pinv = +p⁻¹` | caught | caught | caught | |
| 13 | padding lanes take `pts[0]` | survives | survives | caught (call sequence only) | equivalent mutant; A records the padded lanes' probes |

The other producer tests (`native_sweep_matches_rootfind_subtract` and the skipped GPU-worker
tests) pass in every mode, as they should, because they do not use the IFMA path.

**Mode 3 by size of `p`.** In the standalone copy, under mode 3, I swept 4 096 abscissae × 16
chunks of 8 targets, 1 048 576 candidates per prime (scratch `mode3_by_bits.txt`,
`mode3_threshold.txt`). The changed candidates by size of `p`:

| `p` | changed candidates (of 1 048 576) |
|---|--:|
| random primes of 20–51 bits, and primes just above 2^50, 2^50.5 and 2^50.8 | 0 |
| just above 2^51 | 64 |
| 2^51.2 | 436 164 |
| 2^51.4 | 1 037 058 |
| 2^51.6 | 1 047 061 |
| 2^51.8 | 1 047 908 |
| 2^51.99 | 1 048 287 |

Without the final subtraction, an `mm` output can reach `2p`. Below 2^51 such a lazily reduced
value still fits in the 52 bits `vpmadd52` reads. The closing `mm(x, 1)` then maps it back into
`[0, p]`, because `(x + m·p)/2^52 < p + 1` for `x < 2^52`. Above 2^51, `2p` exceeds 52 bits, and
the high bit is lost. The final subtraction is therefore needed on `(2^51, 2^52)`. That range is
inside what `Ifma52::new` accepts and outside what the producer's tests (`p ≤ 2^20`) or the run
(`p ≤ 2^35.2`) exercise. [pcc:unverified]

**A defect in my own test B, found and fixed.** As first written, B's `include_str!` binding
searched the whole of `rootfind_gpu.rs`, and that file contains B's own copy of the closures.
The binding therefore held vacuously. The literal mode-3 control exposed this: B passed there.
The fixed B searches only the text before `mod validator_ifma_tests` and requires exactly one
match. On the standalone copy, the fixed binding passes on the original source and fails on the
mode-3 source ("the mm closure differs"). On the unmodified scratch copy, rebuilt with the fixed
B, `cargo test --release --lib rootfind_gpu` passes 12 of 12 again (`test-baseline2.log`). B's
arithmetic checks did not depend on the binding; they ran and passed in the first baseline as well.

## Findings

1. **Prediction 2 was missed at one cell; "held at its threshold" is wrong.** PROTOCOL.md's
   prediction 2 is "at least 1.5× faster ... at one thread and at all threads" at `m = 2`
   [pcc:unverified]. With the report's own statistic, the ratio of the median relation-phase walls,
   the four `m = 2` cells are as follows:
   - 1.6675 and 1.5966 on one thread;
   - 1.6223 at log2 r = 30.0 on four threads;
   - **1.4945** at log2 r = 27.5 on four threads (650.756 ms against 435.433 ms).

   The report's "the low end is 1.497, at the predicted 1.5×" and the table's 1.50× come from
   dividing RESULTS.md's three-decimal medians, 0.651/0.435. Even 1.497 is below 1.5. The plain
   statement is: prediction 2 held at three of the four `m = 2` cells and was missed at the fourth
   (1.49× against 1.5×, four threads, log2 r = 27.5). Paired per target and round, that cell reads
   1.410, 1.530, 1.587 and 1.624, median 1.558. The protocol did not fix the statistic. There are
   four runs per arm, L0, a loaded host (finding 8) and no A/A interval, so the measurement cannot
   place that cell on either side of 1.5×. It cannot support "held". The same correction is needed
   in the following places:
   - the table cell (1.49×);
   - the bullet's "1.50–1.62×" (1.49–1.62×);
   - "What this changes" (1.49–1.62×);
   - the record's `summary`;
   - REPORT.pdf.

   Commit `98337e2c6a`'s subject line has the same "1.50" and cannot be changed. [pcc:unverified]
2. **Two `m = 3` cells and two ranges come from rounded medians.** The `m = 3`, log2 r = 33.4 cells are
   1.4456 → 1.45× (report 1.44×) on one thread and 1.2570 → 1.26× (report 1.25×) on four. The
   `m = 3` range is therefore 1.26–1.45× (report 1.25–1.44×). The overall range in the title, the
   figure's description and the record is 1.26–1.67× (report 1.25–1.67×). These understate, and no
   reading changes. Prediction 3 holds either way: the `m = 3` ratios are below the `m = 2` ratios
   at each thread count.
3. **Provenance: a commit inside the run.** `e96ec1b771` was committed into the measured checkout at
   20:11:16Z, while ifma2 was running. ifma3's `session.json` and `host.json` therefore record
   `git_commit` `e96ec1b771`, not `219b1378f8` (both `git_dirty: false`). `e96ec1b771` adds only the
   benchmark record file, and all 96 records carry the same binary hash, so the code under test is
   unchanged. The report says only that the record was initialised on the clean pushed commit; it
   should say that ifma3 records `e96ec1b771` and why that does not matter. This repeats finding 1
   of the native-sweep review.
4. **The producer's IFMA tests never exercise `p` near 2^52.** `ifma_lanes_match_scalar_sweep` uses
   `p ≈ 2^20` and `2^14`. `native_ifma_matches_rootfind_subtract` uses curves of 14–20 bits, and the
   run's largest `p` is about 2^35.2. The kernel accepts any `p < 2^52`. Removing `mm`'s final
   conditional subtraction (mode 3) survives both producer IFMA tests. It is wrong only for
   `p > 2^51`: it changed 1 048 028 of 1 048 576 candidates at a 52-bit `p` and none for `p` up to
   2^50.8. So nothing in the evidence tests the kernel on `(2^51, 2^52)`. Validator test A catches
   mode 3 at `p = 2^52 − 47`. A test like A should be added to the crate, or the refusal bound
   lowered to the tested range. The run itself is unaffected, because its `p` is below 2^36.
5. **The `m = 3` explanation is a hypothesis.** "The probe of the pair table's abscissae (a 64 MiB
   set) is a random memory access per candidate, which the lanes do not speed up" is consistent with
   the counters. 4 194 304 `table_insert_attempts` imply 2^23 slots × 8 B = 64 MiB at load ≤ 1/2.
   But nothing in the run isolates the cause: `perf_event_open` failed in every record, and no arm
   vectorises or removes the probe. Word it as "consistent with".
6. **"What this changes" extrapolates.** "Narrows the GPU's relation-phase lead by that factor per
   CPU core" is not measured, for three reasons:
   - the quoted factor (1.49–1.62×) is the four-thread ratio, not a per-core one (one thread:
     1.60–1.67×);
   - whether the CPU hosts of the A100 rounds have IFMA is not established here;
   - no GPU arm is in this run.

   Label it as an extrapolation. The sentence that the IFMA sweep is the strongest CPU baseline is
   supported only "on this VM", which the paragraph does say.
7. **§5a: no calibrated reading, and a noisy rho reference.** The band is reproduced (section 4).
   The owner's 2026-11-12 `w_cal` reading is absent. It cannot change an order here, because IC at
   `w = 0` is at least 75× rho at `w = 1` on every curve. The matched rho is a four-run mean. At
   log2 r = 33.4 its per-run `S` is 0.172–0.955 and its mean 0.456, about half of the other curves'
   0.89–1.04. The 213× distance there therefore carries about a factor-2 sampling uncertainty. A
   distance to rho should be quoted with that caveat.
8. **Host load, pool size and kernel attestation are not in the evidence.**
   - Before each run the 1-minute load average was 1.81–2.71 (ifma2) and 1.36–1.97 (ifma3) on 4
     CPUs, and CPU PSI avg10 reached 7.09%.
   - The all-threads arms take `available_parallelism()`, and no record states the pool size.
     Affinity is CPUs 0–3 and the 1→4-thread scaling is 3.49–4.06×, both consistent with 4 workers.
   - No counter differs between the scalar and IFMA arms. That the IFMA kernel ran rests on the
     method parameter, the fail-closed refusal in `prepare` and the walls.

   A counter such as `ifma_lane_chunks` would make a record show which kernel ran. The report
   already calls the wall figures indicative.
9. **Record metadata.** `reports.pdf` names `REPORT.md`, not `REPORT.pdf`. `raw[0].uploaded` is
   `false`, and the 82 072 B archive sits in Git, above the 64 KiB data-file threshold of AGENTS.md
   §13. The report discloses the S3 waiver. `validation` is `null`, as expected before this review.
   (Both points repeat finding 9 of the native-sweep review.)
10. **Robustness outside the run.** Neither case below is reachable in this run.
    - `sweep_avx` writes `prefix[0]` after the backward pass, so an empty base (`k = 0`) would
      panic.
    - `inv_lanes` inverts by Fermat, which needs `p` prime, while `Ifma52::new` accepts any odd
      `5 ≤ p < 2^52`.

    The factor base is never empty, and the oracle passes only prime fields. A `debug_assert!` on
    both, or a refusal, would make the contract explicit.

Not findings, for the record:

- The IC arms' `S` is the CPU path's charge, re-derived in `decompose` from the hit positions, so
  "Counted S is unchanged by design and measured unchanged" is an accounting identity plus a check
  that no hit was lost. The arms' extra executed work (whole sweeps, unused targets, the `m = 3`
  probe set) is outside `S`, as in the native-sweep study.
- The replays in both audit receipts ran on the run's own host, with the same binary and
  environment class. The report claims no cross-host audit.

## Wording (requirement 6)

- **Within scope.** The standing is `[pcc:unverified]`. It is one local L0 run with `--cpus none
  --allow-busy`, the wall figures are labelled indicative, and no scoreboard or leaderboard row is
  added. "Counted S is unchanged by design and measured unchanged" is accurate (section 3).
- **Not stated plainly.** The threshold result is not stated plainly (finding 1). "Held at its
  threshold" turns a miss of 0.0055 below the threshold into a pass. The report should state the
  miss, the cell and the fact that L0 cannot resolve it.
- **Wording to soften.** Findings 5 and 6 ask for "consistent with" and "extrapolation".

## Not checked

- No independent replay with a binary built by me, and no replay on another host class. The audit
  receipts are same-host and same-binary.
- No wall-clock re-measurement, and no A/A run.
- No reproducible rebuild of `ecbench`. The binary's provenance rests on four facts:
  - its hash equals the local build's;
  - it contains the IFMA refusal string and `ecbench_before_solve`;
  - the build's mtime is 20:10Z;
  - every session records `git_dirty: false`, read from the worktree at run time
    (`git_commit_source: runtime_worktree`), not embedded at build time.
- `spec_sha256` (a canonical-form hash) was not recomputed. I compared the session specs with the
  frozen files byte for byte instead.
- The scalar `NativeSweep::sweep_one`, the `m = 3` pair table and the rest of `rootfind_gpu.rs`
  were not re-reviewed beyond the parts the IFMA path touches. The native-sweep review covered
  them at `2ec4187be0`.
- The push times come from the local remote-tracking reflog ("update by push"), not from the
  server.

## Scratch evidence (not committed)

`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-ifma/`:

- `scripts/`:
  - `certs.py`, `identity.py`, `recompute.py` and `paired.py`;
  - `validator_ifma_tests.rs` (tests A and B; the fixed binding);
  - `mutate.py` and `run_mutants.sh`.
- `work/`:
  - `mutation.diff` and `mutation-literal3.diff`;
  - `build-*.log`, `test-baseline.log`, `test-baseline2.log`, `test-mut{0..13}.log` and
    `test-literal3.log`;
  - `mode3_by_bits.txt` and `mode3_threshold.txt`;
  - `analyze_out/` (`analyze.py`'s regenerated tables).
- `raw/x/out_ifma/`: the extracted tarball.
- `mini/`: the standalone copy of the `ifma` module (`ifma_extract.rs`, sha256 `12f82c6a…`) and the
  tests.

The scratch source copy came from `git archive d71bc17b0f` of `Cargo.toml`, `Cargo.lock`, `src/`,
the path dependencies, `tests/fixtures/` and the files `src/` includes. The full tree is 12 GB
against about 3 GB free, so a worktree did not fit. The source copy and both build directories
were removed after the tests, so the logs and diffs are what remain. A first `git worktree add`
attempt ran out of space. Git removed the partial worktree itself, and `git worktree list` shows
only the main tree. Nothing in the repository was changed except this file.
