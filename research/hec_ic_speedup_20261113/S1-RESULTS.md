# S1 results: native genus 2/3/4 engine against the BigUint engine

2026-11-13. Protocol: [PLAN.md §6](PLAN.md#6-slice-s1-native-engine-protocol-frozen-before-measuring),
frozen in commit `a4db7afc2d` before any timing below.

**Class: infrastructure.** The algorithm is unchanged: full degree-1 base, factor-base walk
(64 places, `d_step_in = 4`), parity filter then `gcd(u, x^p − x)`, Markowitz elimination mod `ℓ`.
No `S` improvement is claimed.

## What ran

| | binary | built from | sha256 |
|:--|:--|:--|:--|
| BigUint engine | `examples/hyperelliptic_ic_large_prime.rs` (round eight, frozen) | `main` `8839290e5f`, rustc 1.99.1 | `eef942ef…f556c` |
| native engine | `crates/hec-native-ic/examples/hec_native_bench.rs` | `a4db7afc2d`, rustc 1.99.1 | `23e0cafc…1292a` |

- **Host and threads.** One arm64 macOS laptop, single thread. The CPU model was not recordable:
  `sysctl` is blocked in the sandbox. The two binaries ran back to back with nothing else
  scheduled.
- **What each column averages.**
  - BigUint `wall_ic_ms` is its own mean over seeds `20260916 + t`, `t = 0..4`. It includes that
    engine's op-counting instrumentation, which is part of how it runs.
  - The native column is the mean of `total_ms` over the same seed numbers: factor base, walk,
    oracle, rows and solve.
  - Both exclude instance selection.
- **Raw output:** [`s1/`](s1/). The table comes from `examples/s1_compare.rs`.

## Gates

| gate | test | result |
|:--|:--|:--|
| G1 group law | `jac::tests::group_law_g1`: genus 2–4 at `p ∈ {13, 17, 101, 1009, 65537}`, 60 random triples each. Checks commutativity, associativity, inverse, `2D`, `5D`, and the degree-1 fast path against generic Cantor, including `u(x0) = 0` and the opposite place. | **holds** |
| G2 oracle | `oracle::tests::oracle_matches_bruteforce`: every monic polynomial of degree 1–3 and a 1-in-7 sample at degree 4, `p ∈ {11, 13, 17, 19}` (both residues mod 4). Decision and root multiset are compared against brute-force division. | **holds** |
| G3 instances | `(ℓ, m)` equals round eight's on all 11 rows. | **holds** (asserted by `s1_compare`) |
| G4 answers | Every run returns `k` with `k·D₁ = D₂`. | **holds**: 55/55 native and all BigUint runs |

All 8 unit tests also pass in a debug build, with every `debug_assert` active: exact divisions,
consistency of the eliminated system, points on the curve.

## Table

| g | p | ℓ | BigUint ms | native ms | native min–max | **speedup** | native solve share | smoothness BigUint / native range | native ops |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 101 | 4663 | 2.76 | 0.337 | 0.291–0.368 | **8×** | 60% | 0.455 / 0.400–0.505 | 123 |
| 2 | 251 | 61667 | 6.19 | 1.175 | 1.024–1.239 | **5×** | 77% | 0.520 / 0.465–0.506 | 270 |
| 2 | 503 | 65179 | 13.76 | 3.128 | 2.883–3.466 | **4×** | 82% | 0.500 / 0.460–0.569 | 512 |
| 2 | 1009 | 241259 | 26.87 | 7.385 | 6.900–8.187 | **4×** | 75% | 0.477 / 0.455–0.486 | 1035 |
| 2 | 2003 | 2001787 | 50.15 | 13.353 | 12.621–14.838 | **4×** | 82% | 0.507 / 0.494–0.511 | 2008 |
| 2 | 4001 | 16050337 | 102.94 | 26.092 | 24.721–26.973 | **4×** | 79% | 0.501 / 0.485–0.507 | 4028 |
| 3 | 61 | 124459 | 7.72 | 0.590 | 0.467–0.834 | **13×** | 33% | 0.168 / 0.163–0.248 | 203 |
| 3 | 211 | 4620611 | 22.10 | 2.929 | 2.671–3.411 | **8×** | 58% | 0.166 / 0.134–0.188 | 669 |
| 3 | 401 | 31525763 | 40.75 | 5.593 | 5.355–5.765 | **7×** | 55% | 0.155 / 0.149–0.171 | 1298 |
| 4 | 31 | 239753 | 19.07 | 1.057 | 0.736–1.783 | **18×** | 8% | 0.049 / 0.036–0.068 | 533 |
| 4 | 61 | 16790591 | 21.61 | 1.633 | 1.542–1.819 | **13×** | 25% | 0.081 / 0.057–0.110 | 610 |

Group operations match the BigUint engine's `relation_ops` to within seed noise on every row. For
example, genus 2 at `p = 4001` is 4028 native against 4025 BigUint, and genus 4 at `p = 61` is 610
against 570. This is expected: it is the same walk with a different random stream.

## Verdicts on the registered expectations

- **E2 (≥ 10× on every shared row): falsified.** It holds at genus 4 (18×, 13×) and at genus 3
  `p = 61` (13×). It fails at all six genus-2 rows (4–8×) and at genus 3 `p = 211` and `401`
  (8×, 7×).
- **E1 (native smoothness inside round five's 0.49–0.56 / 0.17–0.27 / 0.05–0.08): falsified as
  registered.**
  - Per-seed native minima fall below those bands at nine of the eleven rows.
  - The band was the wrong yardstick, and choosing it was my error: the BigUint engine's own mean
    on the same rows is outside it too (0.455 at genus 2 `p = 101`, 0.049 at genus 4 `p = 31`).
  - The like-for-like reading: on 10 of the 11 rows the native per-seed range contains the BigUint
    five-seed mean. At genus 2 `p = 251` the BigUint mean (0.520) sits just above the native
    maximum (0.506).
  - Nothing here suggests a yield difference, and none is expected from the same oracle on the same
    walk. A proper test would compare yield over many seeds on both engines.

## Why E2 failed: the native solve

**The solve is about 24 ns per entry update.**

- The two solvers do the same algebra. At genus 2 `p = 4001` that is 0.89 M multiplications mod `ℓ`
  native against 1.00 M BigUint (`la_modmuls`).
- The native solve spends about 24 ns per entry update. Row merges go through freshly allocated
  `Vec`s, holder lists are re-sorted, and the heap carries stale entries.
- At genus 2 the solve is 75–82% of native wall time.

**The native relation phase is small by comparison.** At genus 2 `p = 4001` the walk, oracle and
row building take 4.3 ms of the 27 ms. The BigUint engine's per-phase wall time is not reported, so
the relation-phase speedup alone is not measured here and is not claimed.

**The rows are near-dense in the step columns.**

- Each walked row carries the cumulative count of every step place taken so far (≤ 64 columns),
  plus the decomposition and `k`.
- Elimination therefore works on a near-dense 65-column block at the end.
- Storing consecutive differences instead was tried, and is recorded in a code comment. It makes
  rows sparse but chained, and elimination fill-in made the genus-2 `p = 4001` solve 4× slower. It
  was reverted before the measurement.

**Genus is the deciding factor.** At genus 4 the oracle dominates (`m` is small and the yield is
1/24), so arithmetic speed shows directly. At genus 2 the solve dominates.

## What this changes in the plan

1. **The native solve comes before T1.** Round seven's reading still holds: the solve outgrows the
   relation stage below genus 4. At native speed it already dominates genus 2 and genus 3 at toy
   sizes. A planned T3 item moves ahead of T1.
   - Hold the step columns (and `k`) as a dense block per row.
   - Eliminate the sparse factor-base columns first.
   - Solve the residual ≤ 65-column system densely.
   - Gate it against the current solver on the same rows, exactly the same `k`.
2. **T1 genus 4 next.** The genus-4 rows already show 13–18× with the solve at 8–25%. It is the
   genus where a double-large-prime baseline can be built and pushed to sizes the BigUint engine
   cannot reach (`p ≈ 2^8 … 2^12`).
3. **Beyond round eight's sizes.** `pick_instance_with_budget` (BSGS budget via `HEC_BSGS_MAX`) was
   added after this measurement. The full-base walk at larger `p` is descriptive only until the
   solve is fixed.

## Scope

- Full base only, so these are toy sizes (`p ≤ 4001`).
- One host, single thread, five seeds.
- No matched rho is run here: this slice compares two implementations of the same IC. The rho gap
  for these rows is round eight's, unchanged, because the counted operations are unchanged.
