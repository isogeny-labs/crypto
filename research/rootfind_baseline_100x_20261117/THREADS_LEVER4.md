# PLAN lever 4 — threads over targets (`rootfind-floor` + kmitm)

Date: 2026-11-22. `[pcc:unverified]` (shared 4-vCPU host). Public synthetic
prime fixtures only. Engineering throughput claim; charged `S` unchanged.
No security / sub-rho / deployed-curve claim.

## Change

`RootfindFloorOracle` now shares prepared tables behind `Arc` and exposes
`fork_worker` / `decompose_threaded` (same shape as
`RootfindKMitmOracle::decompose_threaded`, already on this branch).

## Falsifying measurement (PLAN gate)

PLAN: run 1 / 2 / 4 threads at large A; **below 2.5× at 4 threads ⇒ memory
bandwidth binds**.

| Arm | A | targets | thr1 ns/tgt | thr2 | thr4 | 2× / 4× speedup | Gate ≥2.5× @4 |
|---|---:|---:|---:|---:|---:|---:|---|
| `rootfind-floor:m=3` (rep 1) | 256 | 512 | 4107 | 2731 | 2163 | 1.50× / **1.90×** | **bandwidth-bound** |
| `rootfind-floor:m=3` (rep 2) | 256 | 512 | 4027 | 2584 | 2752 | 1.56× / **1.46×** | **bandwidth-bound** |

Replay: `cargo test --release --lib floor_threaded_wall_panel_a256 -- --nocapture`.

Correctness: `floor_threaded_matches_single_m3` (verdicts match single-thread
decompose at 1/2/4 workers).

## Verdict

Land the threaded API (engineering). On this host the 4-thread floor probe
is **bandwidth-bound** relative to the PLAN 2.5× gate — same class of
outcome the SURVEY already flagged (0.4–6.2× mixed). Lever 4 does **not**
multiply the CPU stack toward 100× here; remaining wall gains are IFMA
lanes (already A-grade) or GPU (lever 7).

## Claim hygiene

- Throughput-only; charged additions unchanged by design.
- Fail-closed on the speedup gate: recorded as bandwidth-bound, not as a
  pass.
- No PROTOCOL / G3 / ledger promotion.
