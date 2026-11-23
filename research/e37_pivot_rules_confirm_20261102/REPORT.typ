#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E37: a pre-registered confirmation of E36's pivot rules F, S and H

*Run report and results, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.44 (pre-registration, commit `b7cb3327de`, before any code; amendment `3f8066270d`, before any build or run) and §9.45 (results). Code: `63c12fe6a6` (read probe and negative controls in `src/cryptanalysis/chain_pivot_rules.rs`, probe hooks in `klein_v4::chain_quotient`; E36's rules unchanged), `cdf486b90d` (target-stream salt, the v4 pipeline's solver), `a563d8c53c` (examples `e37_stage_a`, `e37_checks`, `e37_walk_e24`, `e37_tables`; `run_e2e.sh`, `identity.sh`). Binaries pinned by sha256 in `experiments/44_pivot_rules_confirm/binary_sha256.txt`. Records: `experiments/44_pivot_rules_confirm/`. Host: the 4-vCPU Claude Code container. isolab-eks is not reachable from it (no `aws`, no `kubectl`), and `conductor` is not installed. No isolation tier is earned: *operation counts only, no wall-time claim.*

*This is a confirmation run of a hypothesis E36 generated.* E36 measured F (each bucket's first-arrived row), S and H ahead of A and E25 at 0, 0.1 and 1 GAE on 20 rows. Its kill criterion fired, though, because its S/H control corrupted rows the solve never reads. E37 re-tests the same rules on fresh target streams and on E24's 12 curves. Its controls carry a positive check, a read probe, showing that a row the normal forms read was altered.

== Outcome
- *The registered kill criterion (G2.165) fired.* The F-specific control C_F (a misfiled arrival) failed its positive check on 320/320 calls. Every corrupted read log diverges structurally (another column), never at a same-column read row. Its raw detection was 316/320, but a call without a positive check counts as not detected. *No rank is claimed.*
- C_SH (S/H inverse permutation wrong at two read rows) passed: positive 320/320, detected 0.841 (S) and 0.863 (H) of all calls, and 0.990 and 0.995 of the non-empty calls. E36's control defect is repaired.
- Identity (G2.164) holds everywhere: 90,000 Stage A solves, 128 end-to-end records against A's, and 11/11 existing configurations byte-identical.
- As measured (descriptive), F, S and H are ahead of A and E25 at every weight on all 32 rows, including under σ1–σ3. Against B″ the order is unresolved. Rho is ahead of every row at every weight.

== What ran
- *Controls* (`e37_checks`, run first): E25's ten GLS curves, 32 calls each from the stream `0xE37C0001`. C_SH on S and H, C_F on F, C_0 (calibration, one entry of a read row moved by one) on F, S, H, L; E31's perturbed coefficient; F/L digests.
- *Stage A* (`e37_stage_a`): the same ten curves and factor bases as E33/E36, 1,000 *fresh* targets per curve (salt `0xE37A0001`), one thread, nine solvers per call (existing, A, A counted, B″, A's rule through E36's echelon, F, S, H, L): 90,000 solves, 0 failures.
- *End to end* (`run_e2e.sh`): six arms (A as `chain-counted`, E25 `macaulay`, F, S, H, B″), all on the same fresh stream (`--stream-salt E37E0001`) with Karatsuba products. Rows: E25's 10 GLS curves × 2 arms (`quartic_pipeline_gls`) and E24's 12 curves (`quartic_pipeline_v4`, $p <= 521$). 132 runs, 0 failures, every log recovered, every walk correct.
- *Matched rho:* walk (iii) of E27 for the GLS rows; `e37_walk_e24` for E24's curves: 192 walks, identical across (i), (ii), (iii), with (i) replaying E24's records exactly; (i)/(iii) cost 1.4524–1.4581.

#figure(image("figures/controls.svg", width: 90%), caption: [Negative controls, 320 calls (199 with a non-empty clean solution set). Positive check: the corrupted run's read log first differs from the clean run's at the same phase and column with a different row hash. C_F has no positive check on any call (all 320 divergences structural; raw detection 316/320). C_0 is the detector's calibration. Source: `experiments/44_pivot_rules_confirm/checks.json` via `summary.json`.])

*Why C_F failed* (post-hoc diagnostic `examples/e37_cf_diag.rs`, not registered, 16 calls): the aimed column is $c^* = 382$, among the last, on every call. The misfiled row keeps 384 pivots and $dim = 64$ but moves which late columns carry pivots. The border targets change, so the reachability pass diverges at its second read before any altered row is read, and the solve falls back on every diagnosed call. The control corrupts the pivot structure the solve reads, not a read row's contents. The registered check did not admit that case.

== Measured (descriptive: G2.165 fired)
#figure(image("figures/stage_a_ratios.svg", width: 100%), caption: [Stage A, per-call means over 1,000 fresh calls per curve: $S_A\/S_X$ at 0, 0.1, 1 GAE. Source: `summary.json` (from `stage_a.json`, sha256 in `stage_a.json.sha256`).])

Stage A (ten curves): $C_A\/C_X$ F 1.465–1.548, S 1.409–1.463, H 1.504–1.564; $C_L\/C_A$ 1.380–1.429; $C_(B'')\/C_F$ 0.948–0.985. Selection per call: F, L 0; S 511.8–512.7; H − S 333.96–333.99. $S_A\/S_X$ at 1 GAE: F 1.0115–1.0129, S 1.0078–1.0086, H 1.0077–1.0083. σ1–σ3 change no F/S/H order. σ2 puts the negative control L ahead of A at 1 GAE (1.0076–1.0087), an accounting effect on A seen in E36 too.

#figure(image("figures/e2e_band.svg", width: 100%), caption: [End to end, paired on the fresh stream: $S_A\/S_X$ per row at 0, 0.1, 1 GAE for F, S, H, B″ and E25 (Macaulay). Left of the dashed line: E25's 20 GLS rows; right: E24's 12 curves. Source: `experiments/44_pivot_rules_confirm/e2e/*.json` via `summary.json`.])

#table(columns: 6, align: (left, left, right, right, right, left),
  [*X*], [*vs*], [*0 GAE*], [*0.1 GAE*], [*1 GAE*], [*order, 32 rows*],
  [F], [A], [1.466–1.539], [1.0944–1.1065], [1.0116–1.0130], [F ahead everywhere],
  [F], [E25], [3.672–3.952], [1.526–1.581], [1.0469–1.0680], [F ahead everywhere],
  [F], [B″], [0.951–0.988], [1.046–1.056], [1.0671–1.0704], [unresolved],
  [S], [A], [1.408–1.450], [1.0830–1.0909], [1.0078–1.0086], [S ahead everywhere],
  [S], [E25], [3.528–3.725], [1.510–1.559], [1.0429–1.0635], [S ahead everywhere],
  [S], [B″], [0.914–0.932], [1.035–1.041], [1.0631–1.0658], [unresolved],
  [H], [A], [1.506–1.543], [1.0963–1.1029], [1.0076–1.0083], [H ahead everywhere],
  [H], [E25], [3.773–3.966], [1.529–1.576], [1.0428–1.0632], [H ahead everywhere],
  [H], [B″], [0.978–0.991], [1.048–1.052], [1.0630–1.0655], [unresolved],
  [B″], [A], [1.539–1.559], [1.046–1.048], [0.946–0.948], [unresolved],
)
Values are $S_"ref"\/S_X$ ranges (above 1, X is cheaper); every order also holds under σ1–σ3. A/A across streams (GLS rows): $S_A\/S_F$ differs from E36's by at most 0.0070 (0 GAE), 0.0006 (0.1) and $< 5 dot 10^(-5)$ (1 GAE).

#figure(image("figures/rho_distance.svg", width: 100%), caption: [Distance remaining to the matched rho (walk (iii)), $S_X\/"rho"$ per row for A, F and E25 at 0, 0.1, 1 GAE (log scale). Every row is above 1: rho is ahead. Source: `summary.json`.])

Distance remaining for F: $3.93 dot 10^3$–$1.32 dot 10^5$ (GLS) and $5.17 dot 10^3$–$4.78 dot 10^4$ (E24) times rho at 0 GAE, and $8.40 dot 10^4$–$3.20 dot 10^6$ at 1 GAE. The best row (GLS $p = 397$ seed 1 arm B) is $3.93 dot 10^3 times$ rho for F against $6.00 dot 10^3 times$ for A at 0 GAE, and $8.40 dot 10^4$ against $8.51 dot 10^4$ at 1 GAE. Per-row values: §9.45 and `tables.md`.

== Verdicts
G2.164 holds · *G2.165 falsified (kill)* · G2.166–G2.168 hold · G2.169 holds as measured (not a rank) · G2.170 falsified on its ≤ 1 % fallback clause (E24 $p = 101$ seed 2: 1.13 % for every chain arm, the same calls), with the orders holding · G2.171 holds (unresolved vs B″) · G2.172: the order holds on 32/32 rows, but the rank is void · G2.173–G2.175 hold.

== Limits and next step
Local container, operation counts only. The σ2/σ3 end-to-end pricing is a Stage A per-call mean times the row's calls (pooled over the GLS curves for E24's rows). The validation gate, leaderboard and `benchmarks/` are not touched. *Next:* E38, registering a positive check that also admits a changed pivot set or border-target set (state `chain_quotient` reads), or an F control aimed at a mid-matrix contested column that keeps the pivot set.
