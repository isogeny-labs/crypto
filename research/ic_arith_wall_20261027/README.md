# IC arithmetic wall-clock pass (2026-10-27)

Goal: lower wall time under the IC pipeline while every counter and answer stays byte-identical.
Host: local L0 sandbox (4 shared cores, other agents compiling on the same host), `--cpus none --allow-busy`.
All timings are **local L0, indicative**; no EKS, no isolab, nothing pushed.

## Changes
- `ic_boundary.rs`: `Barrett` per-modulus reducer (thread-local cached, one lookup per group operation) behind
  `CountedFp::mul/sqr` in `PrimeCurve::add_raw/double_raw`. Word path only (2 <= m < 2^32, reduced operands);
  every other case calls the unchanged `mulmod`. `CountedFp` call tallies are unchanged. `PrimeCurve` is untouched
  (no new field, so its JSON is unchanged). `mulmod` itself and `IncrementalGauss`/`calibrate_row_ops` are not changed.
- `ic_framework/fiber.rs`, `large_prime.rs`: SipHash `HashMap<u64,_>` -> `FastMap` (the three maps `FiberPairTable.map`,
  `FiberFrobeniusPairTable.map`, `LargePrimeCollector.membership`). Checked: only `entry/get/insert/contains_key/len`
  are used, never iteration, and the build uses `entry().or_insert` (first wins), so the hasher cannot change a result.
- `Cargo.toml`: opt-in `[profile.release-lab]` (fat LTO, `codegen-units = 1`). Default release unchanged. Not built or timed.

## Tried and dropped (measured, no gain on this CPU)
- Binary extended GCD for `invmod` (odd modulus): 59 -> 137 ns (m=53117), 120 -> 535 ns (m~2^61). Hardware-division Euclid
  wins; a 32-bit-only Euclid measured the same as the 64-bit one (51 vs 52 ns). `invmod` is therefore unchanged.
- 128-bit Barrett for m >= 2^32: 13.3 -> 15.5 ns, slower than `mulmod` (`__umodti3` is already ~13 ns). Removed.

## Verification
- Property tests (`ic_boundary::tests`): `Barrett::new/for_modulus(m).mul(a,b) == (a*b) % m` over 200k random rounds
  with the edge moduli (1, 2, 3, 7, 53117, 2^32-5, 2^32-1, 2^32, 2^32+15, 2^63-25, 2^63+11, 2^64-59, 2^64-1) and operands
  0, m-1, m, unreduced, random; exhaustive for every m < 400, all a < m+3, b in {0,1,a,m-1,m,m+5}. Also a standalone
  rustc harness over 12M random cases (not committed). `cargo test --release --lib` filters
  (word_sized barrett invmod mulmod prime_group prime_curve fiber large_prime ic_boundary ic_framework incremental):
  214 passed, 2 failed. The 2 failures are missing files of the sparse checkout (`docs/ic/LEADERBOARD.md`, an archived
  n37 source), not arithmetic. `sat_cdcl_parity_modes_are_equivalent_and_reported` failed once during the heavy parallel
  run and passed alone (it uses neither changed code path).
- Byte identity: `specs/prime-p18.json` (R-p18 arms rho, plan, res4; 8 targets x 3 rounds = 72 records) and
  `specs/koblitz-lp.json` (K_0 n=23: rho, `mitm-frobenius:m=3,large_primes=1` (large_prime.rs), `mitm-frobenius-fiber`
  (fiber.rs); 36 records). Baseline binary = main 0e5689117, candidate = this branch.
  `python3 -I normalize_records.py records.jsonl` removes isolation/time/host/binary-hash/session/record/run id fields.
  Result: **identical** (prime 72/72 records, normalized sha256 3e2ea001...; Koblitz 36/36, sha256 2508b43e...), including
  `field_ops` (e.g. invs 1110, muls 2508, sqrs 1206 on record 0) and all counters.
- Micro-benchmark (`arithmetic_micro_benchmark`, ignored test), ns per `mulmod`->`Barrett::mul`: m=53117 11.4->11.1;
  m=2^32-5 12.1->10.7; m=2^61-1 13.2->13.1; m=2^64-59 13.1->13.0 (the last two delegate to `mulmod`).
  An earlier run, noisier host, read 14.1->11.5 and 18.1->11.4 for the two word moduli.
- Wall (whole `ecbench run` sessions, base/candidate interleaved, 4 pairs each, `wall.tsv`):
  prime-p18 median 3.537 s -> 3.476 s (ratio 0.983); koblitz-lp median 1.989 s -> 1.979 s (ratio 0.995).
  Spread within a binary is about 5%, so both ratios are inside the noise.

## Reading
The word-modulus multiply is ~10% cheaper, and the group law is dominated by the inversion, which I could not
speed up. The measured session wall ratio is not distinguishable from 1 on this host. This is not a verified speed-up and
claims nothing for the leaderboard; it keeps counters and answers identical. Commands: `ecbench run --spec <spec>
--out <dir> --cpus none --allow-busy --quiet`.
