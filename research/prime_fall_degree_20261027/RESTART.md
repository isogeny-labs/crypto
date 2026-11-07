# Restart guide: prime-field fall degree study

Written 2026-10-28 from the pushed branch only. The original worktree
(`prime-fall-degree-20261027` on the owner's Mac) held files that were never
pushed: `runs/*.jsonl|log`, `progress.txt`, `TABLE.md`, a draft `RESULTS.md`,
the report draft, and the scorer edits made after commit f1b7dfc03.
Treat all of those as lost until the Mac copy is uploaded.

## What the branch has (verified by reading it, 2026-10-28)

- `PREREGISTRATION.md`: question, definitions, gates G1-G4, cells A-D,
  decision rules H-LFD/H-FFD/H-gap/H-null/H-m, amendments 1-6.
- `DERIVED.md`: derived results D1-D8 (D3: ffd = 4 on Kummer m = 2; D4: the
  no-fall certificate 2t + 2; D8: the 1-D specialisation has d_F = 2).
- `src/cryptanalysis/hky_fp_tower.rs`: exact closure engine.
- `examples/pkm_tower_pilot.rs`: `--engine hky`, multiply-add counts.
- `examples/prime_fall_degree_tools.rs`: scorer/tools (committed version).
- `run.sh`: stages `g4 hky-A hky-B hky-C d-A d-B cap-A`.

## Last known state (from the session transcript; not reproducible from the branch)

- Gates G1-G4 passed (Amendment 3: 6 hand cases; 70 capped-F4 runs, 0
  disagreements; planted closures never contain 1).
- Stage A, Kummer m = 2, P1: d_F medians for t = 1..10 were
  3, 4, 4, 4, 4, 5, 5, 5, 5, 6. Reported by the session, not re-derived here.
- 1-D arm read "1-D flat": d_F(q) = 2 at every t = 2..24.
- Amendment 6 registered before any t >= 11 row: C-step, d_F = 4 + floor((t-2)/4),
  so 6 at t = 11, 12, 13 and 7 at t = 14.
- In flight when the session stopped: stage A (t >= 11), stage C, d-A, and an
  attempt at `--engine f4` (raw `f4_fp`, no 4-free-variable cap) on m = 7, t = 2.

## Restart order

1. `cargo build --release --example pkm_tower_pilot --example prime_fall_degree_tools`
2. `cargo run --release --example hky_fp_tower_check` (G1, 6 hand cases). PREREGISTRATION §4 names `cargo test --lib hky_fp_tower`, but that module has no `#[test]` functions and runs 0 tests; the cases are in the example.
3. `run.sh g4` (G4), then `run.sh hky-A` and `run.sh d-A`, then `cap-A`.
   Stage A restarts from t = 1, and its t <= 10 rows must agree with the
   medians above. Disagreement is a reproducibility failure (Amendment 2).
4. Score t >= 11 against the C-step prediction. It was registered before
   those rows existed, so a miss is a result, not a reason to amend.
5. `hky-C`, `hky-B`, `d-B`. Then P1-P3 (PREREGISTRATION §7) and RESULTS.

Memory: Amendment 2 assumed a shared 32 GB host and two chains at once
(7 rayon threads each). On a dedicated instance, rerun A and C separately.

## Execution route

Per the repository's rules, run on EKS Spot through `isolab-eks`, after
`scripts/checkpoint.sh init` and a filled `.resume/RESUME.md`. The task card is
frozen by the experiment director. Runs must keep op counts (`muladds`), which
the owner asked for on 2026-10-28.

## Owner requests that define done

- "Do what we need to do to prove disprove the ffd/lfd" (2026-10-28 04:31 UTC).
- "Measure operation or instruction count" (04:34 UTC).
