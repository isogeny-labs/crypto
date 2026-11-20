# Validator report: P1 and P2 genus 6-10 hyperelliptic index-calculus measurements

Validator run 2026-11-20. All numbers below are `[pcc:unverified]`: this report is
a validator check, and no pcc claim exists for these results.

## Overall verdict: validated-with-errata

The validator's own code reproduces every counted-cost number in `P1/RESULTS.md`,
`P2/RESULTS_crossover.md` and `P2/RESULTS_F.md` from the committed JSONL.
- Largest P1 difference: 0.38%. Largest P2 crossover difference: 0.33%.
- Group orders: all 32 instance records are consistent with #J computed independently.
- Determinism: a sample replay is bit-identical.
- Mechanism: an independent Monte Carlo and an exact count reproduce the claimed
  smoothness-probability gap.

The errata are listed under "Discrepancies and errata" below.
- Four are documentation or reporting errors: E1, E3, E4 and E5.
- One is a ranking that the producer did not band and that does not hold across
  the validator's band: E2, at g=6 p=211.
- One concerns rounding: E6.
- One is a minor protocol note: E7.

**Not validated: discrete-log certificates.** The run records carry only the
`certified` and `ok` booleans. There is no G, T or recovered t. So no solve
certificate could be re-verified with code independent of the solver. Under the
validator's certificate rule, this requirement is `insufficient_evidence`. It
blocks admission to the leaderboard or to `VALIDATION.json`. It does not
invalidate the counted costs. The remedy is under C1 below.

## Frozen inputs

- Evidence: `P1/` and `P2/` and `P2_instrument.patch` as committed. The last commit
  touching them before dispatch is `430c1a7676`.
- Validated from HEAD `2d770709f0e2bea10944351120832000cb467e0f` (the dispatch).
  The shared working tree moved twice during validation:
  - `a44907ffc8`, an unrelated directory.
  - `bfccf86e8a` (2026-11-20 21:24Z). This commit **appended an erratum to
    `P2/RESULTS_crossover.md`**. It withdraws the p*(8) >~ 10^4 estimate and the g!
    scaling, and cites a new g=8 p=73 cell in `P4_followup/`. The same commit also
    committed this validator's in-progress scripts and outputs, before this report
    existed.
- **Scope of this report.** It validates `P2/RESULTS_crossover.md` as it stood at
  dispatch: sha256 `a0587a6e…`, which is the git blob at `2d770709f0`. The current
  file is `da98c3e7…`; its appended erratum and the `P4_followup/` data are **not**
  validated here.
  - Spot check: the erratum's premise holds in the validated records. The g=8 p=53
    curve-11 runs have `fb_cols` = 19 for arms c and E.
- Every other validated file is byte-identical from `430c1a7676` through `bfccf86e8a`.
- SHA-256 of every validated file, taken from the git blobs at `2d770709f0`:
  `validation/EVIDENCE_SHA256.txt`.
- Binaries, hashed at dispatch:
  - `/home/user/bin/genus_ladder_g16d` = `6cf78101b1370e2c466e8460abe917362e07e62e25e593f8c64edff6579b7f3e`.
    The prefix matches `P1/main/CELLS.log`, `P1/supp/CELLS.log`,
    `P2/crossover*/CELLS.log` and `P1/RESULTS.md`.
  - `/home/user/bin/genus_ladder_g16e` = `1c0cd167beb048166169a96f9befd3213fbe80584d56b10b0c515798aa9cb001`.
    The prefix matches `P2/pilot_phi*/CELLS.log` and `P2/RESULTS_F.md`.
- Validator tooling:
  - Python 3.13.
  - A scratch venv with cypari2 2.2.7 (PARI/GP 2.19.1) and numpy 2.5.5.
  - Single core, with every heavy step run sequentially.

## Per-check verdicts

| # | Check | Verdict | Evidence |
|---|---|---|---|
| 1 | Frozen-file integrity | **PASS** | `sha256sum -c P1/frozen.sha256` gives OK for all 4 (PROTOCOL.md, analyze.py, run_cell.sh, instrument.patch). No commit touches these files after the freeze commit `9caadc9575` (2026-11-17 18:38:06Z). The first main cell ended 18:39:29Z after 25 s wall, so it started after the freeze. |
| 2 | Group orders, independent | **PASS** (32/32 checked, none skipped) | Records: 53 instance records, 32 unique (p, g, curve_seed), no conflicting duplicates. #J was computed by PARI `hyperellcharpoly` and, separately, pinned by the validator's own code in 32/32 (`validation/summarize_orders.out`). The two agree in 32/32. **`order` = #J in 18; `order` != #J in 14** (#J/order = 2 in 8 instances, 4 in 3, 8 in 3). In all 14 of those: r is prime, r divides #J, r² does not divide #J, [order]D = 0 on 8/8 random D, and the lcm of own-verified element orders divides `order`. So `order` is a multiple of the exponent, as the patch's shared-r rule allows. In all 32: `order` = h·r and r is prime. |
| 3 | Certificates (flags) | **PASS on the flags**; independent re-verification **NOT-CHECKED** (no payload) | Run records checked: 224 in the P1 table, 24 in the P2 crossover table, 60 in the F pilot. Exhausted runs: arm `C_double_lp+pre` only, 22 in P1 and 2 in P2. Exceptions: **0** non-exhausted runs lack `certified: true` and `ok: true`. `bad_relations` = 0 in every IC run. No D, E or F run is exhausted. Every R run is certified (`validation/tables.out`, `[certs]` lines). |
| 4 | Tables re-derived | **PASS** (no discrepancy over 1% beyond print rounding) | `validation/tables.py`, output in `validation/tables.out`. It reads the records with `[]`, independent of `analyze.py`/`analyze_p2.py`. P1: 66 values compared, max relative difference 0.38%. P2 crossover: 10 values, max 0.33%. RESULTS_F: 60 values. Two F values differ by 1.34% and 0.99%, which is exactly print rounding: 2.762 printed "2.8", and 47.53 printed ">=48". `>=` markers sit exactly on the cells with exhausted `c` runs. Side claims are re-derived further below. |
| 5 | Determinism replay | **PASS** (determinism only, not correctness) | All 3 curves of each requested cell were replayed, one at a time with `--threads 1`: g=6 p=31 (11, 29, 33), g=7 p=17 (15, 19, 49), g=8 p=11 (7, 29, 36). That gives 9 instance and 72 run records, **2457 fields identical** (`wall_ns` excluded) (`validation/replay_diff.out`, `validation/replay_out/`). This shows the binary is deterministic on these inputs. It does not show the counts or the solves are correct. |
| 6 | Core mechanism, independent | **PASS** | Instance g=6 p=31, curve 33, 200,000 walk samples (`validation/mc_split_g6p31c33.out`). P(u splits into distinct degree-1 places) = 0.00149 ± 0.00009 (MC) against 0.00160 (exact count). P(squarefree, all factors of degree ≤ 2) = 0.1173 (MC) against 0.1169 (exact). Ratio: 73 exact, 79 MC. Producer trials/relation on the same curve: c 775, D 9.11, so 85×. Curves 11 and 29 (exact count): 92 against producer 90, and 994 against producer 726. Cell geometric mean: exact 188 against producer 177. The identity Σ_{n≤g} A_n = L(1) holds on all three curves. |

## Commands run

All validator scripts are in `validation/`. `$V` is the scratch venv python
(`/tmp/claude-0/.../scratchpad/venv/bin/python`, cypari2 2.2.7 plus numpy).
```
# 1 frozen files
cd P1 && sha256sum -c frozen.sha256
git log 9caadc9575..HEAD -- P1/{PROTOCOL.md,analyze.py,run_cell.sh,instrument.patch,frozen.sha256}   # empty
sha256sum /home/user/bin/genus_ladder_g16d /home/user/bin/genus_ladder_g16e
# own-code self-test (21 tiny curves, g=2..4): brute-force #J = own L(1) = PARI; Hasse-Witt = L mod p; group law
$V -I validation/selftest_hyperell.py                       > validation/selftest_hyperell.out   # SELFTEST PASS
# 2 group orders
$V -I validation/check_orders.py --offset 0  --max-instances 11 --json validation/orders_chunk0.json  > validation/orders_chunk0.out
$V -I validation/check_orders.py --offset 11 --max-instances 11 --json validation/orders_chunk11.json > validation/orders_chunk11.out
$V -I validation/check_orders.py --offset 22 --max-instances 11 --json validation/orders_chunk22.json > validation/orders_chunk22.out
#   ^ stopped at index 28: PARI stack overflow (8 MB default) at p=211 g=6; results 22-27 are kept in the .out.
#     After adding PARI.allocatemem(1<<30):
$V -I validation/check_orders.py --offset 28 --max-instances 4  --json validation/orders_chunk28.json > validation/orders_chunk28.out
$V -I validation/own_jorder.py 17,8,8 11,10,7 11,10,12 11,10,27     > validation/own_jorder.out
python3 -I validation/summarize_orders.py                           > validation/summarize_orders.out
# 3, 4 certificates and tables
python3 -I validation/tables.py                                     > validation/tables.out
# 5 determinism replay (single core, sequential)
validation/replay.sh <scratch>/replay 6 31 11 7 17 15 8 11 7
validation/replay.sh <scratch>/replay 6 31 29 6 31 33 7 17 19 7 17 49 8 11 29 8 11 36
#   each invocation: genus_ladder_g16d run --primes P --genera G --curves 1 --curve-seeds P=S --seeds 2
#   --arms c,D,E,R --cc 2.0 --qc 3.0 --max-h 8 --max-trials 5000000 --seed-search 1 --threads 1 --out OUT.jsonl
python3 -I validation/replay_diff.py validation/replay_out P1/main/g6_p31_cDER.jsonl P1/main/g7_p17_cDER.jsonl \
    P1/main/g8_p11_cDER.jsonl > validation/replay_diff.out
# 6 mechanism
$V -I validation/mc_split.py 6 31 33 --samples 200000 > validation/mc_split_g6p31c33.out
$V -I validation/mc_split.py 6 31 11 --samples 50000  > validation/mc_split_g6p31c11.out
$V -I validation/mc_split.py 6 31 29 --samples 50000  > validation/mc_split_g6p31c29.out
```

## How #J was pinned without trusting PARI or the Rust code (check 2)

- **Own code.** `validation/hyperell.py` is written from scratch: F_p[x] arithmetic,
  the Cantor law, and random prime divisors via Tonelli-Shanks in F_p[x]/u. It
  computes:
  - N_k by enumerating F_{p^k} for every k with p^k ≤ 10^7, which gives exact
    a_1..a_K;
  - the Cartier-Manin matrix, which gives every a_i mod p and #J mod p;
  - exact element orders on 8 random elements, which give e', a proven divisor of
    the exponent. PARI's #J served only as a hint for which primes to strip, and
    each order is re-verified by own arithmetic.
- **Unique candidate (28 instances).** The integers in the Hasse-Weil window that
  satisfy all of these constraints reduce to a single candidate, equal to PARI's #J.
- **Sylow closure (the other 4).** For g=8 p=17 curve 8 and g=10 p=11 curves 7, 12
  and 27, 2 to 8 candidates remain. `own_jorder.py` computes |S_l| for every prime l
  dividing the lcm of the candidates, by explicit subgroup closure. The product equals
  PARI's #J in all four.
- **Self-test.** `selftest_hyperell.out` checks the own code against a brute-force
  count of reduced Mumford pairs on 21 small curves.

## Re-derived side claims (check 4)

| Claim (file) | Reported | Recomputed | Status |
|---|---|---|---|
| P-a min pair c/E at g = 6, 7, 8 (P1) | 1.08 at g=6 p=211 | 1.078 at g=6 p=211 curve 28 seed 2; no pair < 1 | ok |
| P-a c/D at g=6 p=101, p=211 (P1) | min 0.56; all pairs 0.18-0.27 | 0.557 (2/6 pairs < 1); 0.179-0.273 (4/4) | ok |
| P-a g=10 p=11 E/D, min pair c/E (P1) | 50x; 0.88 | 51.4; 0.880 | ok |
| P-b decay slope g=6 (P1) | -1.05 | endpoint slope -1.055; least squares -1.035 | ok (method not stated) |
| P-b E crossover g=6 (P1) | p ~ 270 | 269 (LSQ slope) / 271 (slope -1) | ok |
| P-c D_g2/E_g2 at g=6 (P1) | 6x (53), 117x (211) | 6.38 (53), 21.7 (101), 116.6 (211); min pair 6.17 | ok |
| P-d E/D at g ≥ 8 (P1) | 2.3-51x | 2.27 (8,31), 6.73 (8,17), 9.58 (8,11), 51.4 (10,11); min pair 2.14 | ok |
| g=6 lab arm steps/relation (P1) | "about 1100-1700" | trials: 1714, 1084, 1145, 881; walk_ops: 1748, 1109, 1176, 909 (p = 31, 53, 101, 211) | **E5** (p=211 below range) |
| D, E steps/relation at g=6 (P1) | ~10, ~13 | D 9.7-10.3, E 11.9-13.1 | ok |
| E/c relation count at g=6 (P1) | 10-30x | 8.2, 12.9, 22.6, 32.3 | ok (approx.) |
| D LA share at g=6 (P1) | 6%, 11%, 27% | 5.6%, 11.0%, 27.2% (58% at 211) | ok |
| Distance to rho (P1) | as in table | all reproduce | ok |
| Decay slopes (P2 crossover) | g6: -1.48, -0.95, -0.84; g7: -4.13, -0.56 | -1.480, -0.945, -0.842; -4.133, -0.554 | **E6** (-0.56 from rounded gains) |
| p* at slope -1 (P2 crossover) | ~270, ~830, >~10^4 | 271, 831, 1.03e4 | arithmetic ok. The g=8 value was later withdrawn by the producer's post-dispatch erratum (not validated here). |
| E control, 30 reruns (RESULTS_F) | 30/30 field for field | 30/30, all fields except wall_ns | ok |
| F monotone in phi; 14/15 worse than the better endpoint (RESULTS_F) | as stated | holds; the exception is 1.009 at g=7 p=53 phi=0.3 | ok |

## Discrepancies and errata

- **E1 (RESULTS.md, "What this does not show").** The claim is that "D's dense column
  table and E's large-prime bookkeeping (DSU operations, counted in `la_add`) are
  charged". This is incorrect for the reported unit:
  - `FPM_1 = cantor_mul + 10 cantor_inv + rel_mul + la_mul` excludes `la_add` and
    `rel_add`.
  - The patch counts the DSU operations into `la_add` (`res.la.add += dsu.ops`).
  - So they are counted but not priced. D's degree-2 column-table setup is priced,
    in `rel_mul`.
- **E2 (no §5a band; ranking at g=6 p=211).**
  - The producer reports no §5a pricing band. The canonical `band.rs` / `ecbench band`
    reading does not apply: these are genus_ladder records in FPM units, not ecbench
    sessions.
  - The validator therefore priced the counted-but-unpriced work on every arm at
    w = 0, 0.1 and 1 FPM per operation. For IC that is `rel_add + la_add`. For rho
    it is `compares_uncharged + hash_uncharged + coef_adds_uncharged`, scaled to
    expected steps, plus a variant that also adds `native`.
  - **Every arm-versus-rho order holds across this band.** Arms below rho stay below;
    arms above stay above.
  - **The best degree-2 arm over `c` holds in every cell except g=6 p=211:** 1.286x
    at w=0, 1.243x at w=0.1, **0.995x at w=1**. That cell's "1.29x (E)" speed-up, and
    the last point of P-b, is therefore not a band-robust ranking.
  - Cell values at w = 0 / 0.1 / 1:

    | g | p | c / min(D,E) | best-arm / rho |
    |---|---|---|---|
    | 6 | 31 | 10.25 / 9.96 / 8.08 | E 0.425 / 0.441 / 0.582 |
    | 6 | 101 | 2.39 / 2.32 / 1.88 | E 0.072 / 0.075 / 0.104 |
    | 8 | 11 | 691 / 678 / 585 | D 1.03 / 1.06 / 1.31 |
    | 10 | 11 | 201 / 197 / 170 | D 0.72 / 0.74 / 0.92 |

    The full table is in `validation/tables.out`.
- **E3 (undisclosed omission, g=10 p=11).**
  - The timed-out main cell admitted **3** curves (seeds 7, 12, 27); see its `cell`
    record and three instance records in `P1/main/g10_p11_cDER.jsonl`.
  - Only seeds 7 and 12 were run, in `P1/supp/`. Seed 27 was never measured.
  - RESULTS.md deviation 2 says the reruns used "the curve seeds the timed-out cell
    had admitted" and does not mention that seed 27 was dropped. The row has 4
    pairs, not 6.
  - The validator verified curve 27's #J (21754609204 = 4r, `order` = 2r).
- **E4 (DESIGN.md order rule).**
  - DESIGN.md says "an ambiguous window retries with K+1, up to K = g (exact L(1))".
  - The patch escalates at most once, and not at all when p^K > 2×10^8. It then
    falls back to a shared-r rule that stores an exponent multiple in `order`.
  - That fallback applies to 14 of 32 instances (#J/order ∈ {2, 4, 8}). Neither
    DESIGN.md nor RESULTS.md discloses it.
  - It has no effect on any table: only r enters the analysis, and r is a verified
    prime divisor of #J in all 32.
- **E5 (RESULTS.md, Mechanism).** "About 1100-1700 walk steps per relation" at g=6
  leaves out p=211, where the value is 881 trials or 909 walk_ops per relation. The
  per-pair values span 422 to 11,710. The spread across curves is real, and check 6
  reproduces it from n1, the number of degree-1 places: on curve 29, n1 = 12 gives
  an exact ratio of 994.
- **E6 (rounding).** RESULTS_crossover g=7 slope 53→101: reported -0.56, which comes
  from the rounded gains 11.8 and 8.2. The unrounded value is -0.554 (1.1%). P-b's
  -1.05 is the two-point endpoint slope; it does not say so.
- **E7 (minor, P2 crossover).** "Same arms, units and analysis", yet the cap is
  `--max-trials 2e7` in both crossover cells, against 5e6 in P1. The cap is disclosed
  only for g=8 p=53. It is immaterial at g=7 p=101: the largest `c` trial count there
  is 2.11×10^6.

## Contract observations (no numeric error)

- **C1 Certificates.** Run records carry only the `certified` and `ok` booleans. To
  allow independent re-verification, emit per run:
  - G (u, v) and T (u, v);
  - the recovered t.

  `validation/hyperell.py` (`HCurve.mul`, `is_valid`) can then check [t]G = T and
  [r]G = 0.
- **C2 Records.** No `bench.record/v1`, pcc claim or `VALIDATION.json`. RESULTS.md
  says the tools were not on the branch base. That was true at `430c1a7676`, which
  lacks `tools/benchrec.py`, `scripts/checkpoint.sh` and `tools/pcc/pcc.py`. All
  three are present at `2d770709f0`. The leaderboard and benchmarks gate remains
  unmet.
- **C3 Not ecbench (§12).** The comparison against rho uses genus_ladder's
  executed negation-map rho in the same F_p-multiplication unit. That is a matched,
  priced reference, but not an ecbench `vs_rho` claim.
- **C4 Binary-to-source binding** was not re-established by a rebuild: the dispatch
  forbade rebuilding in the worktree. `/home/user/crypto-c0b657b` is `fffa1b1243`
  plus exactly `P2_instrument.patch` (byte-identical diff, ignoring index lines).
  `P2_instrument.patch` differs from `P1/instrument.patch` only by adding arm F.
  The binary hash prefixes match the cell logs.
- **m=83 / ECC2K-130 disclosure rules:** not applicable (genus 6-10, p ≤ 211).
- **Lesson L-99ee952350b2.** `analyze.py` reads `rel_mul`/`la_mul` with `.get(..., 0)`.
  Every C/D/E/F record has both keys (`tables.py` indexes them with `[]` and
  succeeded), so nothing was silently zeroed.

## What this validation does not establish

- **Solve correctness.** The DLP solves are not shown to be correct beyond the
  solver's own flags; see C1.
- **Replay scope.** The replay shows determinism of the producer's binary, not the
  correctness of its counters.
- **Interpretation.** Extrapolations in `RESULTS_crossover.md` (p* for g ≥ 10, and
  "g!/3 to g!/6") are interpretation, not measurement, and were not validated. The
  measured g=6 point, p* ≈ 270, is about g!/2.7.
