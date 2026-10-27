# C1 toy ladder: standard base vs twisted all-lift line, n = 4

2026-10-27. Protocol (frozen before the run; hashes in `FREEZE.sha256`):
[`PROTOCOL.md`](PROTOCOL.md). Report: [`REPORT.pdf`](REPORT.pdf) (source
`REPORT.typ`). Run: [`reports/run-20261027T1502Z/`](./reports/run-20261027T1502Z/) (`summary.txt` is the source of every
number below). Figure: `fig_c1_ladder.svg` / `.png`. Seed `0xC1_7715_7ED_2610_27`.

**Execution route.** The run was local on the session host, not isolab-eks
(AGENTS.md §9a). The session had no AWS identity or CLI. This is local
evidence, and the §9a confirmatory EKS run is open; see
`../lift_dense_followups_20261027/README.md`, "Execution route".

## What was measured, and what was not

Measured, exact and host independent:
- **Relation yield.** Exact numbers of ordered decompositions
  R = P1+P2+P3+P4 per uniform target, standard base x in F_Q against the
  twisted line u F_Q, by a meet-in-the-middle pair table, k in {5,7,9,11}
  (N = 4k = 20..44), curves with a,b in F_16 (family A) and generic b
  (family B, extension).
- **Cost of the Weil-descended Gaudry system.** S_5 built symbolically,
  symmetrised (even exponents in e1,e2,e3 appear: system in E1..E4,
  weights (2,2,2,1), 105 monomials, weighted Bezout 512). One fixed solver
  for both bases: weighted Macaulay matrix at D = 26, forward elimination,
  sparsest pivots. Counted: F_Q multiplications of nonzero entries and
  inversions. 594 solves (198 per arm), k in {5,7,9,11,13,15}.

Not measured: wall-clock time (none claimed); a full F4; extraction of the
F_Q-rational solutions (multiplication matrices, eigen step); any end-to-end
cost or gap to rho; no leaderboard or benchmarks row. The cost verdict is for
this simplified solver only.

## Results (verbatim from `summary.txt`)

Yield, tau = 1 curves (A1 A2 B1 B2 B3), twisted/standard:
- `H-Y2 (tau=1 curves A1 A2 B*): R_pred inside the interval in 34/36; pooled (sum of means) obs 17.3254 / pred 16.9227 = 1.0238  => PASS`
- k = 5: A1 18.718 [16.229, 21.993] against R_pred 18.287; k = 7: A1 16.877 [15.026, 19.185] against 16.516; k = 9: A1 20.778 [15.946, 28.155] against 16.126; k = 11: A1 16.667 [8.909, 51.500] against 16.031. The prediction is the counting identity (about (2Q-2)^4 / (Q-2)^4); this is an instrument check.
- `H-Y1: tested rows 54, |z|<=3 on 48, |z|>5 on 5 (4 of them constant counts != pred, z infinite); finite z mean -0.240 sd 1.352  => FAIL`. All five |z| > 5 rows are the A0 control: k = 9, z = -5.17, and the four constant-zero rows (std and `tw_a`, k = 7 and 9, mean 0 against pred 15.47 and 15.87). The first scoring pass gave these four rows z = NaN and dropped them from the denominator (50 rows, 1 above 5); the scorer now keeps a constant row in the gate with z = -inf (Codex review on PR #2704). The verdict is FAIL either way. The first-pass output is kept as `reports/run-20261027T1502Z/summary_firstpass.{txt,json}`; `RAW.sha256` names it under that file name. Excluding A0, post hoc: 44 of 45 rows within |z| <= 3, the exception z = -3.60, B2 std at k = 5.
- `H-Y3 (A0, tau=0 control): R_pred inside the interval in 2/6; pooled ... = 1.5262  => FAIL`. A0 (b = 1, #E(F_16) = 16) has extremely clustered counts (sd 954 for mean 17 at k = 5; zero hits in 8000 and 1500 standard-base targets at k = 7, 9 although the prediction is about 15.5), so the control is uninformative, not a confirmation. The all-lift fact itself is exact: lifting abscissae 31/31, 127/127, 511/511 on both lines.
- `H-Y4: PASS`: Tr(a) = 1 gives 0 of Q-1 lifting abscissae on the twisted line at k = 5, 7, 9 and 0 relations (the standard base lifts 16/31, 64/127, 256/511).
- `H-Y5: interval contains 1 in 18/21; pooled ratio 0.9518 => PASS`. Caveat: on A2 the two valid u (4 and 6) give identical counts at every k (A/A ratio exactly 1.000), and on A0 `tw_a` is u = 1, the standard line itself; those A/A comparisons carry no information.
- Validation: `cells 12: ordered relations enumerated 16678, equation violations 0, abscissae with x/u not in F_Q 0, pair-table vs enumeration mismatches 0`.

Cost (counted F_Q multiplications, D = 26, same targets):
- Structure: matrix 4840 rows x 4200 columns (40.7 MB dense) for every solve; `H-C1: pairs (std,tw) 396; rank equal 396; HF equal 396 => PASS`; Hilbert function 512 (the weighted Bezout number) in all 594 solves; HF = 512 first reached at D = 25, standard monomials of degree <= 25.
- `all pairs, both closed [PRIMARY]  n=384  geo-mean 0.9606  95% [0.9279, 0.9913]  sd(log) 0.162  => UNCHANGED` (tolerance [0.8, 1.25]).
- Family A: geo-mean 0.9097 [0.8627, 0.9539]; family B: 1.0045 [0.9788, 1.0327]; A/A noise reference tw_a/tw_b: 1.0596 [1.0043, 1.1266]. At k >= 11 the spread collapses: A 0.9299 to 0.9357, B 1.0004 to 1.0069.
- Per-solve spread: `rho quantiles: min 0.476  5% 0.661  25% 0.928  50% 1.000  75% 1.007  95% 1.193  max 2.026; fraction inside [0.8,1.25]: 0.870`.
- `H-C3: closed fraction std 0.995, tw_a 0.975, tw_b 0.975 => PASS`. 11 of 594 systems are not closed at D = 26 (standard monomial of degree 26 remains; HF stays 512): 1 standard-base, 10 twisted. They are target-dependent coincidences with a common zero of the top-weighted forms at infinity, also present on the standard base in the pilot; no fixed D closes them (a real F4 with degree falls would be needed, not run).
- Mean mults: 1.5659e8 (std), 1.5573e8 (tw_a), 1.4475e8 (tw_b). Setup: 3968 F_Q products. Slope of ln(mean mults) per unit k: 0.0011.

## Verdicts

| hypothesis | verdict |
|:--|:--|
| H-Y1 yield identity, |z| <= 3 on 90 % and none above 5 | FAIL (48/54; five rows above 5, all A0 control: z = -5.17 and four constant-zero rows) |
| H-Y2 yield ratio, tau = 1 | PASS (34/36 covered; pooled 1.0238) |
| H-Y3 positive control A0 | FAIL (clustered, uninformative) |
| H-Y4 negative control Tr(a) = 1 | PASS (exact 0) |
| H-Y5 A/A yield | PASS (18/21; caveats above) |
| H-C1 same dimensions, rank, HF | PASS (396/396) |
| H-C2 per-trial cost unchanged in [0.8, 1.25] | UNCHANGED for this solver: 0.9606 [0.9279, 0.9913]; family A about 7 % cheaper at k >= 11 |
| H-C3 closure rate | PASS (0.995 / 0.975 / 0.975) |
| H-C4 k dependence | none (slope 0.0011) |

## Findings worth keeping

1. With a, b in F_16 and k odd, T(v) = Tr_{16/2}(v) on F_16, so tau = Tr_{16/2}(b)
   decides the standard line: tau = 0 and Tr(a) = 0 makes it all-lift already
   (A0: predicted ratio 1). Only two non-isomorphic tau = 1 curves exist, hence
   the generic family B. The upstream "C1 toy ladder" prescribed a, b in F_16.
2. S_5 over e1..e4 has even exponents in e1,e2,e3 in characteristic 2, so the
   descended system lives in weights (2,2,2,1), 105 monomials, with no use of
   the 2-torsion symmetry.
3. The pilot found, and the run confirms, systems whose top-weighted forms
   vanish at infinity on both bases (1 of 198 standard, 10 of 396 twisted).

## Not done

F_Q-rational solution extraction and the eigen step; a full F4 (degree falls)
for the non-closed systems; absolute yield predictions for family B at
k >= 9 (group order not counted; ratio test with (m_tw/m_std)^4 used);
`.resume/RESUME.md` checkpoint and artifact-store push (this session wrote only
under this directory; `yield_k5.jsonl` etc. exceed the 64 KiB data-file
threshold and need `tools/artifacts push`; hashes in `reports/run-20261027T1502Z/RAW.sha256`);
no git operations were run.

## Reproduce

```sh
cargo build --release
./target/release/c1_twisted_line_ladder selfcheck
./target/release/c1_twisted_line_ladder run <dir> validate
THREADS=2 ./target/release/c1_twisted_line_ladder run <dir> yield 5,7,9,11
THREADS=2 ./target/release/c1_twisted_line_ladder run <dir> cost 5,7,9,11,13,15
./target/release/summarize <dir>; python3 plot.py <dir>   # plot.py: layout only
```
`src/bin/summarize.rs` and `src/json.rs` were written after the data (they implement the frozen rules; one bug, a pred parsing error, was fixed after the first scoring pass); the other sources match `FREEZE.sha256`.
