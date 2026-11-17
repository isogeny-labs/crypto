# Curve25519-family guessing optimum on the isogeny form: protocol

Frozen 2026-11-02 before any data in this directory. Lead B, follow-up to
`../curve25519_guessing_20261102` (lead 4), where every F4 cost series fell
monotonically in G and every argmin sat at the grid edge.

## Question

Where is the guessing optimum G* on the isogeny form, does cost turn up before
G reaches t-1 (each branch system then covers `2^(t-G)` base elements, so the
method tends towards enumerating the factor base), and does the optimum
persist as k grows?

## Code and binary

No code change: `pkm-semaev` `guess=G` on the isogeny form already accepts any
`G < t`, and `f4cost` accepts k = 24 and 28 (smoke-checked with 2 targets:
`f4cost 24 isogeny 6 5`, `f4cost 28 isogeny 6 5`, `f4cost 20 isogeny 10 9`
all returned JSON, rc 0). Binary built from the unmodified worktree
(main 56c3e282ee), copied inside the cargo flock to
`/home/user/frozen-bin/curve25519_family_leadB`,
sha256 `fc72092faa52e34f6fa1fb5eeea459b466b4b780416c6724724134374e057311`
(also in `analysis/binary.sha256`). Only this copy is run.

## Exact command line (one cell)

    ( ulimit -v 3500000; timeout 1000 /home/user/frozen-bin/curve25519_family_leadB \
        f4cost <k> isogeny <t> <G> 100 <seed> <cap_s> 2 full 0 )

i.e. 100 random subgroup targets, m = 2, mode full, lifts 0. Cells run
sequentially, one at a time, by `analysis/run_cells.sh` (shell + jq). Per cell:
JSON line, status line (rc, elapsed), stderr file, `uptime` at cell start.
Wall-clock caps: stage 1 cap_s = 600; stage 2 cap_s = 900. A capped cell
reports only the targets it finished and is flagged as a truncated sample. Wall
times are affected by other agents' load on this shared 4-core box and are not
compared.

## Stage 1 (k = 20)

Isogeny form, m = 2, mode full, lifts 0, t in {6,7,8,9,10}, G from 4 to t-1:
20 cells (t=6: 4,5; t=7: 4..6; t=8: 4..7; t=9: 4..8; t=10: 4..9).
Seed `2026111600+t` for all of them (targets are the same across G at a given
t, so G-comparisons within a t are paired on the same 100 targets).

Reproduction check: G = 4 at t = 6, 7, 8 re-run with lead 4's seeds
`2026111400+t` (exact replay). Expected: F4 multiply-adds identical to lead 4's
`f4cost_guess.jsonl` (7.1e5, 9.7e6, 8.64e7 per target), since the source and
the code path are unchanged. Any difference is reported as a failure of the
replay.

## Stage 2 (k = 24, k = 28): rule frozen now

Let (t*, G*) be the stage-1 cell with the lowest per-relation cost
PR(cov) = muladds per target / `coverage_exact`, among cells that are not
capped (ties: smaller t, then smaller G). Let r* = G*/t*.

- 2a, fixed t: for k in {24, 28}, t = t*, G in {G*-1, G*, G*+1}, restricted to
  1 <= G <= t-1. If G* = t*-1 (edge), the window is {G*-2, G*-1, G*}.
- 2b, coverage-matched t: for k in {24, 28}, t' = t* + (k-20)/2 (t' = t*+2, t*+4;
  the same t - k/2, hence similar coverage), G' = round(r* t'), window
  {G'-1, G', G'+1} restricted to 1 <= G <= t'-1 (edge rule as in 2a).
- If a 2b cell would need t' > 12 it is run with the same rule but expected to
  cap; capped cells are reported, not dropped.

Seeds: `2026111700+t` for k = 24, `2026111800+t` for k = 28 (t is the cell's t).
The rule is applied mechanically to the stage-1 table; no cell is added or
removed after seeing stage-2 data. (If the stage-1 table makes 2a and 2b
identical for some k, that is stated.)

## Reported per cell

F4 multiply-adds per target (all branches charged, but a target stops at its
first verified hit, so mean cost is below the full-refutation cost wherever
coverage is high, notably t = 9, 10 at k = 20); calls per target; hits and
verified hits; solving degree; undecided; wall and uptime.

Per-relation cost on three bases, always labelled:

- **PR(lam)**: muladds per target / lambda, lambda from
  `<frozen binary> support <k> isogeny <t>` (available for every cell,
  k = 20, 24, 28, t = 6..10 recorded in `analysis/support.jsonl`; for a stage-2 t outside 6..10 the same command is run once the rule has fixed the cell). Lambda is
  the cofactor-aware 8-projected yield and is about 8x the yield of the
  subgroup-target oracle (lead 4, Amendment 1), so PR(lam) is a lower figure.
- **PR(cov)**: muladds per target / `coverage_exact` (exact fraction of the
  order-r subgroup that is a sum of 2 base points). Primary basis for stage
  selection and all argmins.
- **Observed**: total muladds / hits, only where hits >= 1, with the hit count.
  Per-relation figures are modelled (cost / frozen yield) unless this column
  is given; hit counts at low coverage (a handful in 100 targets) cannot rank
  cells.

Argmin G per series is the minimum of muladds per target (ties: smaller G) and,
separately, of PR(cov); the two coincide because coverage is constant within a
(k, t). "Bracketed" means a smaller-G neighbour and a larger-G neighbour are
both measured and both higher. Per-target costs are not emitted by the tool, so
no per-cell interval is available. An ordering of two cells of one series is
called **resolved** only if the ratio is at least 1.25 and neither cell is
capped; otherwise inconclusive.

## Predictions (frozen)

- **P1 (U-shape).** For t = 8, 9, 10 the argmin G is interior and equals t-3 or
  t-2, and cost at G = t-1 exceeds cost at the argmin by a resolved ratio
  (>= 1.25). Judged true if it holds for at least two of the three t; false if
  fewer.
- **P2 (stable ratio).** The argmin G/t of stage 2 (both 2a and 2b where
  bracketed) lies within 0.1 of the stage-1 value for the same series family,
  i.e. the optimum scales with t, not with k. Judged only on series that are
  bracketed and uncapped.
- **P3 (falsifier).** If muladds per target falls strictly (resolved) from G = 4
  to G = t-1 for t = 8, 9 and 10, there is no turn-up in the measured range: the
  optimum is the edge G = t-1 and lead 4's monotone picture extends to
  t = 10. P1 is then false.
- **P4 (reproduction).** G = 4 at t = 6, 7, 8 with lead 4's seeds reproduces
  lead 4's muladds per target exactly.

## Scope

Every statement is for k in {20, 24, 28} on these curves, m = 2, mode full,
lifts 0, the t and G measured, 100 targets (fewer if capped). No claim about
other k, t, m, other forms, or a distance to Pollard rho (lead A, different
unit). No replay certificate, validator report or matched rho is attached, so
nothing here is a leaderboard or `benchmarks/` row. Uncharged native work
(`pkm_system_fp_mults_uncharged`, `pkm_grid_fp_mults_uncharged`) is recorded per
cell; its share of charged work is reported (lead 4 found it below 0.05%).
Run locally on the agent host, not on EKS: a development measurement inside the
lead.
