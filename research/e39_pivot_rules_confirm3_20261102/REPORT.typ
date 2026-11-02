#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= E39: the third pre-registered confirmation of the pivot rules F, S and H

*Run report and results, 2026-11-02.* Research note `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: §9.48 (pre-registration, commit `16d17d2462`, pushed before any E39 code or run) and §9.49 (results). Code: `31f2a2999e` (the control hook `RuleCheck::bump_post` in `src/cryptanalysis/chain_pivot_rules.rs`, off by default) and `e1cd726577` (examples `e39_checks`, `e39_stage_a`, `e39_tables`; `identity.sh`, `run_e2e.sh`). Binaries built from a clean `git archive` export of `e1cd726577` (4,678 files checked blob by blob), pinned by sha256 in `experiments/46_pivot_rules_confirm3/binary_sha256.txt`. Records: `experiments/46_pivot_rules_confirm3/`. Host: the shared 4-vCPU Claude Code container. isolab-eks is not reachable from it and `conductor` is not installed. No isolation tier is earned: *operation counts only, no wall-time claim.*

*This confirms a hypothesis that E36–E38 generated; it is not a new hypothesis.* E36 and E37 measured F (each bucket's first-arrived row), S and H ahead of A and E25 at 0, 0.1 and 1 GAE on every row they ran, but in each a registered control failed and the kill voided the rank. E38 repaired the check on E37's F control (`P_ext`) but its structure-preserving F control could never be aimed, so its kill fired at the pilot. E39 replaces that control with C_F^post and changes nothing else.

== Outcome
- *No kill fired.* Identity 21/21 byte-identical; pilot P1 passed by every primary control, so no fallback was used; the main control gate held; Stage A and all 154 end-to-end records satisfy the identity clauses.
- *F, S and H are each ahead of A and of E25 at 0, 0.1 and 1 GAE on all 32 rows* (20 GLS + 12 E24), and stay ahead under σ1–σ3. With G2.191 and G2.192 holding, G2.201 holds: *F, S and H are ranked ahead of A and E25 across the band on these rows*: a measured engineering improvement of the `k = 4` solve's echelon (a constant). It has not passed the validation gate yet.
- Against B″, every rule is unresolved: B″ is ahead at 0 GAE and behind at 0.1 and 1. Rho is ahead of every arm on every row at every weight.

== Controls
C_F^post: from the clean F run of the same call, `V` = the contested columns the left-looking normal forms read; `l*` = its lower median; after `l*`'s bucket updates (the pivot row's last use in the echelon) the pivot row's last-column entry (monomial 1, standard) is moved by one. `P_post`: pivot set and border targets unchanged, the first differing read is that row at `l*`, and a normal-form read of it changed. Before freezing, `V ≠ ∅` was verified on all 160 of E38's stored pilot calls (non-null `c_star`).

#figure(image("figures/controls.svg", width: 100%), caption: [Pilot P1 (80 calls, 56 non-empty) and the main control run (320 calls, 194 non-empty). Gates: positive check at least 0.95 and non-empty detection at least 0.96 (from the pooled E37 and E38 C_SH and C_0 rates, lowest one-sided 95 % bound 0.9668). Source: `pilot.json`, `checks.json` via `summary.json`.])

#table(columns: 6, align: (left, left, right, right, right, right),
  [*control*], [*check*], [*positive*], [*detected (all)*], [*non-empty detected*], [*pilot non-empty*],
  [C_F (E37's misfile)], [`P_ext`], [320/320], [316/320], [192/194], [55/56],
  [C_F^post (new)], [`P_post`], [320/320], [271/320], [193/194], [55/56],
  [C_SH (S)], [strict], [320/320], [276/320], [194/194], [55/56],
  [C_SH (H)], [strict], [320/320], [280/320], [193/194], [56/56],
  [C_0 (F, S, H, L), calibration], [strict], [320/320 each], [265–273/320], [191–193/194], [54–55/56],
)
C_F^post was aimed on 320/320 calls, with `|V|` 161–240 and `l*` in columns 255–302; pivot set and border targets were unchanged on all 320 and the first difference was the altered row's read on all 320. C_F changed the pivot set and the border targets on all 320 calls, and its strict check held on none, as in E38. The perturbed coefficient changed 194/194 non-empty sets under each of F, S, H, L; F's and L's digests differ from A's on 320/320. *Plainly:* C_F^post is mechanically close to the calibration C_0. It shows the detector sees a content corruption of a row F chose and the normal forms read; it does not test F's choice. C_F tests an F filing defect.

== Stage A (10 curves × 1,000 fresh calls, salt `0xE39A0001`)
#figure(image("figures/stage_a_ratios.svg", width: 100%), caption: [$S_A\/S_X$ per curve at 0, 0.1, 1 GAE (per-call means). Values from `tables.md` (computed by `e39_tables`).])
Every solver returns the existing solver's outputs on every call, and the same calls fall back. Selection per call: F, L 0; S 511.9–512.7; H − S 333.95–334.00. $C_A\/C_X$: F 1.471–1.548, S 1.411–1.463, H 1.503–1.563; $C_A\/C_L$ 0.710–0.721. $S_A\/S_X$ at 1 GAE: F 1.0116–1.0128, S 1.0077–1.0086, H 1.0076–1.0083; L 0.9856–0.9862.

== End to end (32 rows, paired on the fresh stream `0xE39E0001`)
#figure(image("figures/e2e_band.svg", width: 100%), caption: [$S_A\/S_X$ per row at 0, 0.1, 1 GAE. Left of the dashed line E25's 20 GLS rows, right E24's 12 curves. Per-row values: `tables.md`.])

#table(columns: 6, align: (left, left, right, right, right, left),
  [*X*], [*vs*], [*0 GAE*], [*0.1 GAE*], [*1 GAE*], [*order, 32 rows*],
  [F], [A], [1.457–1.537], [1.0937–1.1064], [1.0115–1.0130], [F ahead everywhere],
  [F], [E25], [3.626–3.945], [1.520–1.581], [1.0439–1.0676], [F ahead everywhere],
  [F], [B″], [0.952–0.988], [1.046–1.056], [1.0668–1.0704], [unresolved],
  [S], [A], [1.402–1.449], [1.0825–1.0908], [1.0077–1.0086], [S ahead everywhere],
  [S], [E25], [3.488–3.719], [1.505–1.559], [1.0400–1.0631], [S ahead everywhere],
  [S], [B″], [0.913–0.932], [1.035–1.041], [1.0628–1.0658], [unresolved],
  [H], [A], [1.497–1.542], [1.0957–1.1028], [1.0076–1.0083], [H ahead everywhere],
  [H], [E25], [3.725–3.957], [1.523–1.576], [1.0399–1.0627], [H ahead everywhere],
  [H], [B″], [0.977–0.991], [1.048–1.052], [1.0627–1.0655], [unresolved],
  [B″], [A], [1.530–1.558], [1.0459–1.0480], [0.9464–0.9482], [unresolved],
  [L (control)], [A], [0.710–0.725], [0.9006–0.9053], [0.9856–0.9863], [L behind everywhere],
)
Values are $S_"ref"\/S_X$ (above 1, X is cheaper), $S(g) = "total_muls" + 97 g ("entries" + "selection")$. Every F, S, H order holds under σ1–σ3. Under σ2 the control L moves ahead of A at 1 GAE on every row, an accounting effect on A that E36 and E37 also saw and E39 predicted. Health: every log recovered, every walk correct, unsolved at most 0.05 %, fallbacks at most 1.30 % (E24 $p = 101$ seed 2: 1.13 %, ceiling 1.63 %), the same count for every chain arm on each row. A/A against E37's stream: $|Delta S_A\/S_F|$ at most 0.0111, 0.0010 and $8 dot 10^(-5)$.

#figure(image("figures/rho_distance.svg", width: 100%), caption: [Distance remaining to the matched rho (walk (iii): E27's on GLS rows, E37's `walk_e24` on E24's, reproduced byte for byte by the E39 build), $S_X\/"rho"$ per row, log scale. Every row is above 1: rho is ahead.])
Distance remaining for F: $5.18 dot 10^3$–$2.32 dot 10^5 times$ rho at 0 GAE, and $9.40 dot 10^4$–$5.62 dot 10^6 times$ at 1 GAE. The closest row, GLS $p = 397$ seed 1 arm B, has F at $5.18 dot 10^3 times$ (A $7.91 dot 10^3 times$) at 0 GAE and $1.11 dot 10^5 times$ (A $1.12 dot 10^5 times$) at 1 GAE. On E24's curves the closest row is $p = 397$ seed 1: $5.37 dot 10^3 times$ and $9.40 dot 10^4 times$. At 1 GAE the Macaulay entries are almost all of $S$, and no pivot rule changes them.

== Verdicts
G2.190–G2.203 all hold. In particular G2.191 (identity) and G2.192 (controls) hold, so G2.201 holds: F, S and H are ranked ahead of A and E25 across the band on all 32 rows.

== Limits
Local shared container: operation counts only; no isolation tier, no wall time. The σ2/σ3 end-to-end pricing is a Stage A per-call mean times the row's calls (the ten curves' mean for E24's rows), an estimate. The rank covers these 32 rows at $p <= 521$ and the gain at 1 GAE is about 1 %. B″ remains unresolved against F, S and H. The validation gate (replay certificate, validator report, matched rho with the §5a band), the leaderboard and `benchmarks/` are left to the main session. PR #3307 merged while E39 ran (records up to Stage A); later commits are local.
