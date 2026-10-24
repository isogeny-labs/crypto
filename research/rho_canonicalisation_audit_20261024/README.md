# Rho canonicalisation audit (2026-10-24)

Workstream F of `/Volumes/SSD990-2/ic-regime-review-20261024/PLAN.md`, answering REVIEW.md §3 item 6 and §5.2 M1 and M5. Desk audit: no solver was run, nothing was pushed. Labels: **measured** = read from a committed file (path and line given); **derived** = inference from measured items; **proposed** = recommendation.

The N83 cold-base panel directory `research/n83_cold_base_confirm_20261021/` exists only on `ci/*` branches, not on `origin/main`; it was read from commit `753b622349`. All other `crypto` inputs were read from `origin/main` (`5cb553810e`), and `cryptanalysis` inputs from its `origin/main`.

## 1. Which rho the N83 panels and the leaderboard use

**N83 K1103/K1107 cold-base panel.** measured:
- Rho producer: `examples/n83_rho_optimization_eval.rs`, named by the workflow (`.github/workflows/n83-cold-base-confirm.yml`) and by `run.py` (`run_rho`, lines 101-109), which passes `cold.RHO_CONFIG` and seed `2026102100 + point_index`. Source hash recorded in `results/source.json`.
- Config: `research/n83_dense_root_20261013/rho_config.json` = `{"backend":"fold","lanes":64,"dp_bits":6,"escape_fruitless":false,"step_cap_factor":2000}`.
- The example builds `StrongRhoG` (lines 174-190) and calls `rho.solve` (line 288). `solve` dispatches to `solve_dispatch::<false>` (`src/cryptanalysis/koblitz_strong_rho.rs`, `solve` near line 1434), which calls `canonicalize` (line 1372), not `canonicalize_legacy` (line 1339).
- Receipt field `"quotient": "signed_frobenius"` (example line 381). The receipt does not name the canonicalisation basis; that gap is what REVIEW §3.6 points at.
- Same-point rho online geometric mean over 16 points: 2,334.964 ms (`RESULT.md` line 10). This is the 2,335 ms in REVIEW.

**Canonicalisation used by the N83 rho.** measured, `src/cryptanalysis/koblitz_strong_rho.rs` header lines 10-13 and `canonicalize` (line 1372): the canonical representative of `{±φ^k(P)}` is the least rotation of the x normal coordinates (`least_rotation`, line 780), sign chosen by y, ties resolved by `choice_exhaustive`, orbit multiplier `±λ^k` from a table. So it is **normal-basis signed-Frobenius least rotation**: not plain negation, not a polynomial-basis orbit minimum. The N83 rho is therefore already the canonicalised walk that cryptanalysis calls R2.

**Leaderboard Table A.** measured:
- `docs/ic/LEADERBOARD.md` lines 21-27: reference = "one-target rho on the same point ... A = 2n by signed Frobenius and negation".
- `docs/ic/boundary_targets.json`: rho reference entries name `koblitz_rho_fixture <n> <a> signed_frobenius 1 strong` (library `crypto_lib::cryptanalysis::koblitz_strong_rho`), and `rho_r2_normal_coordinate_canonicalization` as a recorded rung.
- `src/cryptanalysis/ecbench/methods.rs` line 2186: reference = `koblitz_strong_rho::StrongRho`, "the single-target reference of docs/ic/boundary_targets.json since 2026-10-01".
- `research/ecbench_rho_optimized_20261014/README.md` lines 44-46: accepted canonicalisation is normal-basis per-coordinate linear maps.

**Derived:** the N83 panel and Table A reference run the same canonicalisation (`StrongRhoG::canonicalize`, normal basis). Neither uses plain negation or the polynomial-basis orbit minimum. Some boundary rows do use A=1 (`boundary_targets.json`: "reference walk uses A=1 (no negation map)"); those are not the N83 or Table A reference and were not audited row by row.

## 2. Does any crypto rho implement normal-basis canonicalisation?

**Yes.** measured:
- `src/cryptanalysis/koblitz_strong_rho.rs`: `canonicalize` (line 1372), `canonicalize_legacy` (line 1339, "pre-optimization canonicalization ... within-binary performance control"), `x_orbit` (line ~1135). The `StrongRho` family is "rung 3 of examples/koblitz_rho_batch_ks_strong.rs" (header).
- Other normal-basis users: `examples/n83_rho_optimization_eval.rs`, `examples/koblitz_rho_batch_ks_strong*.rs`, `examples/koblitz_rho_batch_ks_v3.rs`, `examples/ic_equal_advice_wide.rs`, `src/cryptanalysis/ecbench/methods.rs`.
- The polynomial-basis reference is `src/cryptanalysis/koblitz_index_calculus.rs` line 11244, `canonicalize_signed_rho`: a `BigUint` minimum over `curve.frobenius` orbit members, one `frobenius` per step, `n` steps per canonicalisation. This is the kind of canonicalisation cryptanalysis's R1 measured.
- `examples/koblitz_rho_batch_ks.rs` (the vs_rho comparator) uses squaring-walk canonicalisation with Fermat inversion (per `boundary_targets.json`); it is the older polynomial-basis path.

**What cryptanalysis measured.** measured: `papers/frobenius-orbits/sec-benchmarks.tex` (table `tab:rho-ladder`, around lines 160-190, n=53, 1024 targets, Callgrind Ir): R0 371.1e9, R1 (library field, hardware clmul, still polynomial-basis canonicalisation) 281.0e9, R2 (+normal-coordinate canonicalisation) 68.9e9, R3 (+32 lanes, batched inversion) 46.4e9. R1 to R2 is 4.08x, the "4.1x" in `sec-abstract.tex`. The abstract's "reverses an apparent index-calculus win" refers to IC/rho going from 0.317 (R1) to 1.294 (R2).

**Derived:** the 4.1x is the step from the old polynomial-basis reference to the normal-basis one, and crypto's `StrongRho` already contains that step (crypto boundary record `rho_r2_normal_coordinate_canonicalization`). It does not show that the N83 rho is weaker than the best available rho. Its real content is that IC/rho ratios measured against a polynomial-basis rho are inflated about 4x. Any such ratio in crypto that cites `koblitz_rho_fixture ... strong` or `StrongRho` is already normal-basis; any that cites `koblitz_index_calculus::canonicalize_signed_rho` or `koblitz_rho_batch_ks` is not.

**What a port would touch.** Nothing for the N83 or Table A arms. A polynomial-basis control arm, if the review wants the cryptanalysis comparison on N83 itself, would need: a new example or flag that runs `canonicalize_signed_rho` (or the batch_ks walk) at n=83 with the same `rho_config.json`, seeds `RHO_SEED_BASE + point_index`, and the same 16 points; its receipt must name the basis; and the rho binary hash in `results/source.json` (`rho_binary_sha256` 4ed1335f...) would change for that arm only. A new basis label in the receipt (`"canonical_basis": "normal"`) would be the minimal fix for REVIEW §3.6 for the existing arm.

**Proposed:** M5 as written ("port from cryptanalysis") is already satisfied in crypto. Replace it with (a) a receipt label for the existing arm and (b) an optional polynomial-basis control. Backlog card `backlog/XP-20261024-rho-canonicalisation-control/` drafts (b).

## 3. Readiness of the equal-advice control (M1)

Sources: `research/equal_advice_wide_20261018/{README.md,PROTOCOL.md,run.sh}`, `research/20261016-n83-precomputed-rho-control/README.md`.

**Status (measured):** the instrument is built and checked at n=61 (identical counters on 24/24 rows); the n=73 and n=83 session is "frozen and NOT run" (README).

**Memory (measured, PROTOCOL.md lines 132-141, table columns n=73 | n=83):**
- Resident at once: IC about 5.6 GB + BL about 5.8 GB at n=73; IC 13.9 GiB + BL 15.4 GB at n=83.
- Minimum RAM (preflight rejects below): 24 GiB at n=73; 48 GiB at n=83. Address-space cap per process: 16 GiB; 32 GiB.
- Disk: 2 GiB free (`MIN_DISK_KIB`, run.sh line 110).

**Time (measured, PROTOCOL.md lines 139-141):**
- n=73: IC rank stage 18,398 s single-thread (5.1 h) recorded; BL table walks 40 to 100 core-hours; session 3 to 5 h projected at 32 threads.
- n=83: IC setup 580 to 694 s with 4 rank threads; BL walks 27 to 67 core-hours; session 2 to 3.5 h projected at 32 threads.
- All of these are projections: the wide step cost is unmeasured (the n=61 check measures the narrow backend, 0.2 to 0.3 us).

**Host requirement (measured):**
- Protocol: "The host must be otherwise idle during the online passes" (PROTOCOL.md lines 144-146). One cpuset (`taskset`), one address-space cap (`prlimit --as`), one process group (`setsid`), `nice 0`, `RAYON_NUM_THREADS=1` for online (lines 100-106).
- `run.sh` line 199 writes `"isolation": {"required": "L2", ...}`. Its default is `ISOLATION_LEVEL=L0` (line 49). PROTOCOL.md lines 145-147 and 180-181: any level above L0 needs an isolab receipt; claim check fails `independent_validation` and `independent_replay_pointer` on this session by design.
- `docs/ecbench/README.md` §6: L2 "quiet" requires a quiet preflight (PSI and other-CPU thresholds), and L1 requires a reserved cpuset with the run CPU read back as pinned.

**Admissibility of this Mac at load 70 (derived):** not admissible.
1. Load is not idle. `uptime` at 14:56 showed 106.8 / 127.8 / 123.1 (1, 5, 15 min). The plan's figure of 60 to 70 is not idle under any reading of "otherwise idle".
2. The script is Linux-only. `run.sh` reads `/proc/meminfo` (line 108), and its preflight requires `taskset`, `prlimit`, `setsid` and `nice` (line 115), which are not present on macOS. It would exit 2 with "missing taskset". This was read from the script, not run.
3. L2 cannot be earned under this load, and no isolab receipt exists, so the claim would be L0 at best. Operation counts (IC probes, BL steps) would still be valid at L0 per `docs/ecbench/README.md` §6, but wall time would not.
4. Mac RAM was not measured in this audit (`sysctl` returned nothing in the sandbox). n=83 needs 48 GiB minimum.

**Launch note for an idle host (proposed):** run on a Linux host with at least 48 GiB RAM, 32 threads, and 2 GiB free disk, with `taskset`, `prlimit` and `setsid` from util-linux, GNU `time`, and bash 4.4 or later. Check that the 1-minute load is below the host's idle threshold before starting, and record `/proc/pressure` figures. Run `CARGO_TARGET_DIR=target THREADS=32 research/equal_advice_wide_20261018/run.sh n73 n83` from the repository root, with `ISOLATION_LEVEL` left at L0 unless an isolab receipt is supplied. Expect about 3 to 5 h of session wall for n=73 and 2 to 3.5 h for n=83, as projected in the protocol; the recorded n=73 IC rank stage alone was 18,398 s single-thread (5.1 h). Treat the result as an operation-count and accounting audit; no wall-clock or claim-level result follows without an L2 receipt and a second-host replay.

## Consequences for REVIEW

- M5 is largely already done in code: the N83 rho and Table A rho are normal-basis. The residual work is a basis label in the receipt and an optional polynomial-basis control (card `XP-20261024-rho-canonicalisation-control`).
- M1 cannot run on this Mac. It needs an idle Linux host with 48 GiB RAM.
- The N83 K1107 2,335 ms rho figure is measured with the normal-basis canonicalisation; the review's concern about the rho's strength is answered in code, not by measurement.
