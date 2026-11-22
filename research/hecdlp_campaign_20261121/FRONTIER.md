# Frontier after campaign round 1 (2026-11-21)

Best measured index-calculus arm against the matched negation-map rho, by genus and curve kind,
with the stack each lead added. Every figure is `[pcc:unverified]`; all runs on the shared 4-vCPU
container; FPM primary reading (`cantor_mul + 10·cantor_inv + rel_mul + g²·la_mul`, the quartic arms
with `pic_*`). "Below rho" holds in the FPM band; the §5a GAE band stays unresolved (the LA price
decides). Replays by C5 reproduce the genus-3 composition rows field for field (`VALIDATION_REPORT.md`
in the two studies); everything else is a single producer's measurement.

| genus / kind | best arm | over rho | at p | change this round | source |
|---|---|--:|--:|---|---|
| 2, hyperelliptic | 2LP + prefilter | 3–5 | 2^24 | none (no lead targeted it) | `genus_ladder_scale_20261115` |
| 3, hyperelliptic | 2LP + prefilter + SGE(16,3) | 0.255 | 32771 | C1: w_max 16 (was 0.283 with SGE(8,3)); the LA drift 0.32→0.39 stops at 0.31–0.32 | `genus_ladder_sge_depth_20261121` |
| 3, hyperelliptic | same, cc 1.5 | 0.368 | 16411 | C2: cc 1.5 gains 0.8–3.7 % over cc 2.0; not a lever | `genus_ladder_cc_sweep_20261121` |
| 3, plane quartic (random) | Diem 2LP + algebraic decomposition + SGE(8,3) | 0.0070 [0.0037, 0.0133] | 8209 | C3: decomposition by conic resultant (`rel_mul` 0.20 of the scan), FPM 0.54 of the old arm (was 0.013 of quartic rho); arm now LA-bound | `quartic_diem_alg_20261121` |
| 4, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.0198 | 4099 | C2: first SGE point at genus 4, p = 4099 (was 0.0375 without SGE); C1: w_max 16 is worse at genus 4 | `genus_ladder_cc_sweep_20261121` |
| 4, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.061 | 2053 | C1 (p = 1031: 0.068) | `genus_ladder_sge_depth_20261121` |
| 5, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.004 | 1031 | C1 (was 0.0084 without SGE) | `genus_ladder_sge_depth_20261121` |

## What the round established
- **Merge depth is not a lever beyond w_max 16** (C1). Non-zeros grow 1.3×, 2.0×, 3.0× for
  (8,3), (16,·), (32,3) at genus 3, p = 32771, and Wiedemann pays for the density; the merge's own
  work is negligible. fill_max 3 beats 10. The Wiedemann multiplication ratio tracks
  n·(generic entries + n) within 0.016 in all 30 rows (descriptive, six cells).
- **The base constant is not a lever for the SGE arm** (C2). cc = 2.0 is within a few percent of
  the optimum at genus 3 and 4; SGE moves the optimum back toward 2.0 relative to the unmerged arm
  swept on main (PR #4852: cc 1.5 at κ ≈ 2). At genus 4 the base saturates at the p/2 cap near cc 4.
- **Diem on quartics is now linear-algebra bound** (C3). At q = 8209, 9·la_mul is 58 % of FPM_mid;
  SGE(8,3) removes only 11 % of la_mul because the cores are small (about 290 × 205). Three of five
  predictions failed (ratios 0.542 and 0.0070 against 0.5 and 0.005; exponent 1.31 [0.94, 1.61]).
- **Genus 3 at p = 32771 is replayed** (C5): 36 IC rows, 18 unmerged, 18 rho rows, 9 instances with
  0 mismatches; the analyzer double-counted the unmerged arm (intervals too narrow, means unchanged;
  errata in both studies).
- Pending: C4 (exact orders by BSGS from N_1 alone; genus-3 extension to 65537 and 131101).

## Distance remaining and next levers
At genus 3 the arm sits at about a quarter of rho in the FPM reading with exponent 1.40 against rho's
1.41: no exponent gain, a constant-factor gain. The linear algebra after the merge is now the whole
lever at genus 3 and on quartics: block Wiedemann with a wider block, or a sparse solver that
exploits the two-large-prime structure before the merge (the core's generic entries dominate the
cost, per C1's model check). The relation phase's Cantor steps are the lever at genus 2, where no
arm is below rho.
