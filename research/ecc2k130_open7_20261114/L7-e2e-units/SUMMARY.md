# L7 — end-to-end units for binary/Koblitz IC against matched rho

**Verdict: CLOSED-BY-PRIOR for the method; coverage gaps remain.** This lane re-read committed
data and made no new measurement. All claims `[pcc:unverified]`. The subagent could not write
Markdown reports, so the main session saved its narrative here.

Main-session checks: [pcc:unverified]
- `raw/SHA256SUMS` verifies.
- `results.json` has 147 rows and 8 exclusions.
- The quoted slopes (0.047 for FC w=0, 0.113 for LB-A) appear in `results.json`.
- The PROTOCOL freeze hash `e4aa1d63…` is the one committed in `73990ef46e` before data.
  After its dated Amendment 1 the hash is `3c126ec3…`.

## Prior work already covered every part of the item
ecbench's S = GAE/√r covers every phase, runs matched rho in the same session, and applies the
§5a band. The fitted exponents are in `ic_frontier_campaign_20261015` (IC α 0.589 against rho
0.436), `ecbench_frobenius_orbit_vs_rho_20261014` and `docs/ic/leaderboard.json`. Large-prime
rows are in `ic_koblitz_dlp_tuning_20261023` and `gaudry_k4_char2_crossover_20261015`. Phase
shares are in leaderboard Table B and `ic_koblitz_descent_routing_20261026`.

## 1. Consolidated table (`table.csv`, `results.json`)
- 147 rows. Each is read from committed JSON/JSONL and cites its source.
- 8 rows are excluded with reasons, among them FC `ic-shared` (unpriced work not counted) and
  the pair claw (a BSGS, not index calculus).
- Nearest complete prime-degree Koblitz rows:
  - LZ lazy orbit, n=29 (log2 r = 15.4): 1.23×, 1.34× and 2.33× at w = 0, 0.1 and 1. The
    source's own figure is 1.27× [0.77, 1.80]. These arms use 1–3 columns and about 2
    relations, so they are close to a collision walk.
  - At log2 r ≥ 36: the validated LB-A row at n=47 is 4.08× [3.76, 4.44]; FC at n=37 is
    5.20×, 7.40× and 18.0× across the band.
- The only rows below 1 are GK (k=4, composite degree, m = 17–18). They depend on the
  subfield, use one target against theoretical rho, and were never replayed, so they do not
  transfer to n=131.

## 2. Scaling
Slope of log2(IC/rho) per bit of r, within each source:

| series | slope [95%] | P(cross by log2 r = 129) | ratio at 129 |
|---|---|---|---|
| FC ic-compact, w=0 | 0.047 [0.006, 0.088] | 4% | 190 (8–4,027) |
| LB-A (validated) | 0.113 [0.056, 0.169] | 0.1% | 5.9·10³ |
| LZ lazy | 0.115 [0.080, 0.151] | — | 1.6·10⁴ |
| PX pipeline | 0.107 [0.066, 0.148] | — | 1.1·10⁴ |
| DT 2LP | 0.150 [0.09, 0.21] | — | — |
| GK k=4 (composite) | −0.246 | — | crosses near log2 r ≈ 66 |

- **Every prime-degree Koblitz arm moves away from rho as r grows.**
- Fitting on n instead of r is confounded (n=61 has log2 r = 47). The two-variable fit removes
  that: the n coefficient is about 0, the r coefficient is +0.09 to +0.16, and the predicted
  ratio at (n, log2 r) = (131, 129) is 460× to 2·10⁴×.
- The direction is robust; the size at 129 is very uncertain (leverage 22–88, 4–8 sizes
  below 47 bits).

## 3. Large primes (measured, no projection needed)
- **DT Koblitz 2LP against base, at w=0:** 1.001, 0.985, 1.183 and 1.095 at n = 37–47. The
  order never holds across the band.
- **GK k=4:** 1LP costs 1.48–1.60×, 2LP 1.08–1.51× relative to no large primes; LA falls to
  0.52–0.78×.
- LA is at most 0.1% of S (counted) or 1.2% (timed) in every binary row, so large primes can
  save at most that much on the matrix side.

## 4. What dominates S
Relation search is 50–92%, table or oracle build 5–45%, LA ≤ 1.2%. Trials per relation grow
from 7 to 2·10⁴. **Only cost per trial, trials per relation or table build can move S.**

## Controls
- Positive control: the native `ecbench band` re-read reproduces the frozen FC file exactly at
  w=0 (16/16 cells).
- At w > 0 the re-read differs because of the 2026-11-01 byte-counter rule. Amendment 1
  records this; it corrects the 1e-9 tolerance PROTOCOL §7 asked for.
- A/A: rho-strong-aa equals rho-strong in 16/16 files.
- `results.json` is byte-identical across two runs.

## Residual gaps
1. No validated Koblitz row in counted GAE with a real §5a band.
2. No complete m=83 counted IC row against matched rho, so the §8a gate is unmet.
3. No Koblitz 1LP row, and no validated large-prime row.
4. No GF(2^n) w_cal instrument (`ecbench wcal` covers prime fields only).
5. The DT, DR and PX band files predate the November counter rules, and their records are in
   S3 only.

## Files
`PROTOCOL.md`, `table.csv` (in the artifact store), `results.json`,
`fig1_log2cost_vs_log2r.svg`, `fig2_ratio_vs_log2r.svg`, `raw/`, `run.sh`, `tool/` (Rust).
