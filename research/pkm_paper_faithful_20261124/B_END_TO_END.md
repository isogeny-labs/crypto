# Algorithm 1 end to end at `B > 2`

Date: 2026-11-25. Driver: `examples/pkm_mixed_ecdlp.rs`. Raw rows:
`p18_b11_full.json`, `p18_b2_rate.json`, `probe_p18_*.json`. All
`[pcc:unverified]`.

## What was built

The paper's Algorithm 1 needs only the relation oracle; the mixed-degree chain
already existed in `pkm_smooth_tower`. `examples/pkm_mixed_ecdlp.rs` is the
outer loop around `pkm_smooth_tower::algebraic_decompose`:

1. `search_aux_exact(p, S)` finds `E'/F_p` with a point of order `S = ∏ q_j`;
2. `isogeny_tower_on` builds the Vélu chain and `V = x(R' + ⟨Q⟩)`;
3. the factor base is every point of `E` with abscissa in `V`, one column each;
4. for random `a, b`: `R = aP + bQ`; `algebraic_decompose` builds the paper's
   system (4) — `S₃` plus the chain, denominators cleared — solves it with
   `f4_fp`, and returns every checked relation `P_1 + P_2 = R` with `x(P_i) ∈ V`;
5. Gaussian elimination mod `r` on `L_1 + L_2 − b·k = a` recovers `k`, checked
   by `[k]P == Q`.

This is not yet an `ic bench` plug-in: `prime-isogeny`'s membership still
assumes a 2-isogeny kernel (`ξ/ψ²` with `ψ` linear). The driver uses the same
system and the same solver, so the comparison below is engine-for-engine.

## Results

`p = 3907`, generator `(6,15)`, `r = 3889`, planted `k = 2683`:

| B | degrees | n0 | cols | targets | relations | rank | k | verified | wall |
|--:|:--|--:|--:|--:|--:|--:|--:|:--|--:|
| 2 | 2⁶ | 6 | 30 | 74 | 32 | 17 | 2683 | yes | 6.7 s |
| 5 | 3,3,5 | 3 | 21 | 87 | 22 | 12 | 2683 | yes | 2.0 s |
| 7 | 3,5,7 | 3 | 44 | 36 | 45 | 35 | 2683 | yes | 3.4 s |

`p = 261847`, roster curve `y² = x³ + 3x + 2`, `r = 261673`, planted
`k = 203179`:

| B | degrees | n0 | cols | targets | relations | undetermined | rank | k | verified | wall |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|
| 11 | 3,3,5,11 | 4 | 256 | 483 | 258 | 1 | 139 | 203179 | **yes** | 3 125 s |
| 2 | 2⁹ | 9 | 271 | 40 (capped) | 21 | 6 | 21 | — | rate probe | 1 526 s |

## Findings

1. **Algorithm 1 completes at `B > 2`.** Two fields, three arities of chain,
   five runs, every one recovering the planted scalar and passing `[k]P == Q`.
2. **At `p = 2^18` the mixed chain finishes and the 2-power chain does not.**
   `B = 11` needed 483 targets at 6.5 s each — 52 min end to end. `B = 2`
   costs 38.1 s per target, and 6 of its 40 probes exceeded the 60 s solver
   budget; at 271 columns it would need ~520 targets, about **5.5 h** — 6×
   longer — with ~15 % of targets lost rather than solved. The `B = 2` figure
   is an extrapolation from 40 measured targets, stated as such.
3. **The target pins well before full rank**, as in the harness runs: rank 139
   of 257 for `B = 11`, rank 12 of 22 at `p = 3907, B = 5`. Collecting to
   `cols + 1` is deliberate here (a stronger check than the pin needs).
4. At `p = 3907` the ordering is not monotone in `B` (6.7 s at `B = 2`, 2.0 s
   at `B = 5`, 3.4 s at `B = 7`): at a small field the 2-power chain is only
   12 variables and already cheap, so the smoothness bound buys little. The
   gain appears when `log2 p / m` makes the 2-power chain long.

## Caveats

* One auxiliary curve per rung (`seed = 7`), one planted scalar per rung;
  no seed panel.
* `S = 495` against `p^{1/2} = 511.7` (3 % small) at `B = 11`, and `S = 512`
  (= `p^{1/2}`) at `B = 2`, so the two are within 3 % of the same density.
* The solver is the generic `f4_fp` in both arms, so the comparison is
  engine-for-engine; the `ic bench` harness's tower rewriting would make
  `B = 2` cheaper than it is here and is the obvious reason not to read these
  numbers as an end-to-end speed claim against the harness.
* `B = 2` at `p = 2^18` was not completed. Its 38.1 s/target and 15 %
  undetermined rate are the measurement; the 5.5 h is arithmetic.

## Wall-clock note

The `p = 3907` table was taken on a quiet host. A repeat of the same three runs
under concurrent load gave 53.8 s, 15.5 s and 20.4 s instead of 6.7 s, 2.0 s and
3.4 s — the same rows, the same recovered scalars. Wall time on this host is
load-sensitive by up to 8×; the `p = 2^18` pair was measured back to back and
the 6× gap between `B = 11` and `B = 2` is larger than that swing, but the
absolute minutes are not portable.
