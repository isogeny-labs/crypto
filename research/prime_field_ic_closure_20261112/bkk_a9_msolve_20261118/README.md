# Route A9 (BKK / sparse-resultant presentation): direct msolve comparison   [pcc:unverified]

Date 2026-11-18. Label `bkk`. Every number below was produced in this run and is an
unverified assertion (`[pcc:unverified]`). Nothing was committed or pushed.

## Question and frozen rule

`PRESET.txt` (sha256 in `PRESET.sha256`) was frozen before any data. Two amendments
(`PRESET_AMEND1.txt`, `PRESET_AMEND2.txt`) change the instrument and add a secondary run.
The kill rule did not change.

- **System.** p has b bits, b ∈ {12, 14, 16, 18, 20}. The curve is random. The factor base V is L random x-coordinates with L = round(2^{b/m}), entered as the dense f(x_i) = Π(x_i − r). The target x_R is random.
- **Arms.**
  - D: dense S_{m+1}(x_1..x_m, x_R).
  - m=3: chain C3 / C3u, i.e. S3(x1,x2,u), S3(u,x3,xR), with u last or first.
  - m=4: chain C4, balanced B4 = S3(x1,x2,u), S3(x3,x4,v), S3(u,v,xR), and hybrids H4a = S4(x1,x2,x3,u), S3(u,x4,xR) and H4b = S3(x1,x2,u), S4(u,x3,x4,xR).
- **Picks.** The BKK pick minimises the mixed volume of the square subsystem, with ties broken by fewer variables. The oracle pick is the cheapest alternative per instance.
- **Kill rule.** Fit log2(metric_D / metric_X) = α + β·b on uncensored bit sizes. The route survives only if, for some m, both wall time T and Msum (Σ rows×cols over F4 rounds) give β ≥ 0.05 with a bootstrap 95% CI lower bound > 0 on ≥ 3 bit sizes. Otherwise it is KILL.

## Instrument

- **msolve.** /usr/bin/msolve with `-l 1 -g 2 -t 1 -v 2`, giving the reduced DRL GB only. Each run had a 900 s timeout and a 4.5 GB address-space cap. The m=3 ladder ran with 1 worker and the m=4 ladder with 2, at the same time on 4 cores.
- **Default solver crashes.** The default `-l 2` gives SIGSEGV on every Semaev input with p > 2^16. That includes L = 8, without a memory cap, while toy systems at p ≤ 2^31 run. This forced amendment 1. The `-l 2` rows for 12–16 bits are kept as a cross-check and show the same pattern.
- **Correctness checks.**
  - S4 and S5 vanish on 20/20 random sums and are nonzero on 20/20 random x_R. S5 with x_R fixed has 6561 terms, degree 8 in each variable and total degree 32.
  - Positive controls with a planted decomposition: every arm's reduced GB is satisfied by the planted point. This holds at m=3,4 with 12 bits, and at m=3 with 18 and 20 bits (L = 8, checking `-l 1` at p > 2^16).

## BKK (mixed volume) of each presentation

The m−1 base segments factor out, so MV = L^{m−1} · MV(projections onto x_m and the aux variables).

| m | arm | vars | BKK | Bézout |
|---|---|---|---|---|
| 3 | D | 3 | 4·L² | 12·L² |
| 3 | C3, C3u | 4 | 4·L² | 16·L² |
| 4 | D | 4 | 8·L³ | 32·L³ |
| 4 | C4, B4 | 6 | 8·L³ | 64·L³ |
| 4 | H4a, H4b | 5 | 8·L³ | 48·L³ |

- **Every presentation has the same BKK number,** 2^{m−1}·L^{m−1}, which is the fibre degree. BKK cannot rank the presentations.
- **The tie-break picks C3 (m=3) and H4a (m=4).** H4a is the slowest m=4 arm, 2–30x slower than D where both finish.
- **Bézout/BKK is 3, 4, 4, 8 or 6.** The L factors cancel, so the ratio does not depend on p. That is the red team's analytic bound.

## Results

Each cell gives the geometric mean of wall time in seconds over 3 instances, then the largest F4 step degree Dmax in brackets. "TO" means the run timed out at 900 s. "seg" means it stopped with SIGSEGV at the address-space cap.

| m | b | L | D | C3/C4 | C3u | B4 | H4a | H4b |
|---|---|---|---|---|---|---|---|---|
| 3 | 12 | 16 | 0.32 [24] | 0.93 [20] | 1.13 [20] | | | |
| 3 | 14 | 25 | 1.72 [33] | 10.5 [29] | 13.9 [29] | | | |
| 3 | 16 | 40 | 23.4 [48] | 86.0 [44] | 123.5 [44] | | | |
| 3 | 18 | 64 | 309 [72] | TO ×3 | TO ×3 | | | |
| 4 | 12 | 8 | 20.1 [32] | 2.27 [13] | | 0.77 [12] | 41.6 [18] | 0.76 [16] |
| 4 | 14 | 11 | 13.2 [35] | 19.7 [18] | | 4.15 [15] | 428 [22] | 6.39 [19] |
| 4 | 16 | 16 | 689 [40] | 588 [25] | | 25.5 [20] | TO ×3 | 36.6 [25] |
| 4 | 18 | 23 | seg ×3 (at deg 49) | TO ×3 | | 691 [27] | not run | TO ×3 |

The 20-bit cells were not run. By then D was censored, so they had no paired ratio (amendment 2, S2).

Saving factors R = metric_D / metric_X and the fitted slope β (R ∝ p^β), with bootstrap 95% CIs:

| m | pick | metric | R at 12 / 14 / 16 bits | β | 95% CI | survives |
|---|---|---|---|---|---|---|
| 3 | C3 (BKK) | T | 0.348 / 0.164 / 0.272 | −0.089 | [−0.316, 0.098] | no |
| 3 | C3 (BKK) | Msum | 0.153 / 0.096 / 0.090 | −0.191 | [−0.313, −0.050] | no |
| 3 | oracle | T | 0.353 / 0.164 / 0.272 | −0.095 | [−0.316, 0.092] | no |
| 3 | oracle | Msum | 0.159 / 0.104 / 0.094 | −0.191 | [−0.309, −0.064] | no |
| 4 | H4a (BKK) | T | 0.483 / 0.031 / censored | −1.98 (2 points) | | no |
| 4 | H4a (BKK) | Msum | 0.112 / 0.455 / censored | 1.01 (2 points) | | no |
| 4 | oracle (B4/H4b) | T | 27.6 / 3.19 / 27.1 | −0.007 | [−0.074, 0.060] | no |
| 4 | oracle (B4/H4b) | Msum | 2.24 / 39.3 / 56.8 | 1.17 | [0.77, 1.57] | yes |

At m=3, 18 bits, D finished on 3/3 instances in about 300 s while both chain arms timed out on 3/3. So R < 0.34 there, and the cell is censored on the alternative side.

**Secondary run S1** (amendment 2): m=4, 18 bits, instance 0, run alone with an 11 GB cap.

- D stopped with SIGSEGV after 4212 s at F4 degree 50. Its last matrix was 58004 × 223639, and its RSS was about 10 GB at 67 minutes.
- B4 finished in 453.5 s.
- That gives R_T(18) > 9.3, a lower bound.
- With that lower bound added, β_T = −0.081.
- β_T would reach the 0.05 threshold if R_T(18) ≥ 17, that is T_D(18) ≥ about 7.7k s. D had already run 4.2k s, was still at degree 50 and was growing. So the m=4 time verdict is **sensitive to the censored 18-bit cell**.
- H4b in S1 was stopped before it finished, to save the machine.

**Exploratory, not in the preset:**

- **F4 degree tracks L plus a constant that depends on the presentation.**
  - m=3: D has Dmax = L + 8 and the chain has L + 4, exactly, at 12–16 bits. At 18 bits D is L + 8 again, while the chain reached only a lower bound before timing out.
  - m=4: D has L + 24 at 12–16 bits, and reached L + 27 at 18 bits when it stopped.
  - B4 is L + 4 at 12–18 bits.
  - The degree gap D − B4 is 20, 20, 20, then ≥ 23.
- **Σ nonzeros (rows × cols × density), D / B4 at m=4:** 9.9, 7.1, 106.
- **Phase split for m=4 D at 16 bits:** symbolic preprocessing takes 334 s of 696 s. At 12 bits D's time is 85% linear algebra, at 20 s versus 13 s at 14 bits. So the 12-bit D cell, where L = 8 equals deg_{x_i} S5, is an outlier in D's own scaling. The m=4 β_T rests on it.
- **`-l 2` cross-check** (12–16 bits for m=3, 12–14 bits for m=4): R_T is 0.46 / 0.26 / 0.19 for m=3 and 29.8 / 2.1 for m=4. That reproduces the 14-bit dip.

## Verdict against the kill rule

**KILL under the frozen rule.** No (m, pick) passes on both metrics.

- **m = 3: KILL, and it is robust.** Each alternative presentation is slower than dense, by 2.9–6x at 12–16 bits and by more than 2.9x at 18 bits, where the chains time out and D takes about 300 s. The deficit does not shrink with p. The F4 degree gap is a constant 4 in the chain's favour, but the extra variable costs more than that.
- **m = 4: KILL by the rule, but not settled.**
  - The best alternative presentations, B4 (balanced S3 split) and H4b, are 3–28x faster than dense at 12–16 bits. That is a real constant-order saving.
  - The BKK-chosen presentation, H4a, is slower than dense.
  - The time ratio has no significant trend (27.6, 3.2, 27.1). The Msum ratio grows strongly, but Msum overstates D's work.
  - The rule-deciding 18-bit pair is censored on the dense side. Its lower bound (> 9.3) is compatible with both readings. If the 12-bit outlier is dropped, 14 → 16 bits gives 3.2 → 27.
- **BKK itself does not discriminate.** The mixed volume is the same for every presentation, so "BKK-chosen" carries no information beyond a tie-break. Any saving comes from F4 structure (the balanced split), not from the root count. That part of the route is closed.

## Open or blocked

- **One run would settle the m=4 time trend: D at 18 bits.** It needs more than 11 GB of RAM and more than 1.2 h on one core (it was still at degree 50). The 20-bit m=4 cell needs more still. Neither fits this container (about 14 GB shared). These are isolab-eks jobs and were not submitted, because this task ran local-only.
- **More instances per cell** would firm up the 14-bit dip and help decide whether to read Msum or T, especially at L ∈ {8, 11}.
- **msolve `-l 2` SIGSEGVs for p > 2^16 on these inputs.** It reproduces in /dev/shm/bkk/tmp/p18_L*.ms, regenerated by code/bkklib.py. This is worth an upstream report.
- **research-kb was unavailable.** `scripts/research-kb` found no executable, and no MCP server was available. Sources checked instead: closure-ledger.md row A9, prime-field-ideas.md #7, SOURCES_CHECKED.md (KR-IC-5a43e5), prime_ic_lead_heatmap_20261114/TABLES.md (LEAD-ICP-split-graph-secant-presentation), and factor_base_arity_review_20261024/lit_degree_reduction.md (Semaev 2015 and Karabina 2015, the S3-chain literature for binary fields).
- **Only the dense x-coordinate base was tested.** The coset base x^L = c and the PKM tower bases were not.
