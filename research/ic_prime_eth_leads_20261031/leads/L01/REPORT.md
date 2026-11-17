# L01 report: characteristic-2 Fourier gadget, edge/degree census and XOR-layer SAT arm

Status: measured (census and identities complete; SAT arm complete for l = 3..6, **l = 7 gadget arm censored**). Verdict: **undecided** for the slope rule (see 5).
Label key: [M] measured, number seen in a file in this directory; [D] derived from a measured/construction count; [P] plan.

## 1. What was built (all under this directory)
| file | purpose |
|---|---|
| `src/gadget.py` | char-2 Schonhage (triadic) multiplication as an XOR-layer circuit, one split N = N1*N2, four-step DFT, 2*N2 pointwise products in B_{N1}, carry-less recombination, linear reduction mod f (no quotient guess exists in char 2) |
| `src/test_gadget.py`, `src/census_test.py` | circuit == F_{2^n} multiplication on random inputs (n = 5, 9, 15, 17, 19); principal-root, four-step == direct DFT, no-wraparound checks -> `raw/identity_checks.json` |
| `src/census.py` | edge/degree census n = 13..131 -> `raw/census.json` |
| `lean/L01Index.lean` | Lean (core, no Mathlib, kernel `decide`/`omega`) for no-wraparound, four-step exponent identity (all naturals), inverse block read-off for N2 = 3, 9. Output `raw/lean_L01Index.out`, 0 errors, axioms propext/Quot.sound only |
| `src/gen_instances.py` | instance generator: standard ANF (`pdp_anf.py` copy), standard XOR-CNF (`anf2cnf.py` copy), gadget XOR-CNF |
| `src/build_wdsat.sh` | builds pinned WDSat sources (copied from `ml-ic-20261031-data/p4/wdsat/vendor/src`) with only `__MAX_ID__` fitted per l (155, 300, 502, 767, 1101 = values WDSat itself prints), `__MAX_ANF_ID__` 72, `__MAX_XEQ__` 120, `__MAX_XEQ_SIZE__` 3000, buffer/eq sizes enlarged |
| `src/run_arms.py`, `src/analyze.py` | runner (per-child rusage via wait4, fresh process per run) and aggregation |
| `raw/arms_*.jsonl`, `raw/analysis_output.txt`, `raw/slopes.json` | every run record |

Not used / blocked: `/usr/bin/time -l` (instructions retired) fails in this sandbox (`sysctl kern.clockrate: Operation not permitted`), so **instructions retired were not measured**; primary metric is solver conflicts, secondary child CPU seconds. The host had load average 60-135 on 14 cores, so CPU seconds are noisy (conflicts are not). WDSat `-x` on the gadget was **not run** (gadget is emitted as XOR-CNF for CryptoMiniSat, not as WDSat ANF). CryptoMiniSat 5.14.7 at /opt/homebrew/bin was used.

## 2. Census (a): edges and degree of the char-2 gadget [M: `raw/census.json`; model [D]]
Model (explicit): edge = one bit wire between layers, counted once (inputs 2n; forward mid and out layers of both operands; pointwise outputs; inverse mid and out; recombined product bits; reduced output n). Vertices: one row/column of each four-step stage, one pointwise product (black box, 4*N1... in, 2*N1 out), N1-bit blocks of recombination and reduction. Degree = distinct input wires + output wires of the vertex. Theorem 3 compared with d+1 := n (assumption: Theorem 3's d+1 is the operand bit width). One split only; N = 3^nu >= n, so counts jump at n = 28 and n = 82 (padding).

| n | N | N1xN2 | L=PxQ | edges | edges/n | edges/(1100n) | max deg | deg/(136 n^0.75) | pointwise ANDs | schoolbook n^2 |
|---|---|---|---|---|---|---|---|---|---|---|
| 13 | 27 | 9x3 | 9=3x3 | 605 | 46.5 | 0.042 | 108 | 0.116 | 832 | 169 |
| 15 | 27 | 9x3 | 9=3x3 | 643 | 42.9 | 0.039 | 108 | 0.104 | 1044 | 225 |
| 17 | 27 | 9x3 | 9=3x3 | 682 | 40.1 | 0.036 | 108 | 0.095 | 1264 | 289 |
| 19 | 27 | 9x3 | 9=3x3 | 714 | 37.6 | 0.034 | 108 | 0.087 | 1411 | 361 |
| 23 | 27 | 9x3 | 9=3x3 | 770 | 33.5 | 0.030 | 108 | 0.076 | 1595 | 529 |
| 27 | 27 | 9x3 | 9=3x3 | 830 | 30.7 | 0.028 | 108 | 0.067 | 1746 | 729 |
| 28 | 81 | 9x9 | 27=9x3 | 2241 | 80.0 | 0.073 | 324 | 0.196 | 5096 | 784 |
| 31 | 81 | 9x9 | 27=9x3 | 2296 | 74.1 | 0.067 | 324 | 0.181 | 5160 | 961 |
| 40 | 81 | 9x9 | 27=9x3 | 2463 | 61.6 | 0.056 | 324 | 0.150 | 5363 | 1600 |
| 54 | 81 | 9x9 | 27=9x3 | 2699 | 50.0 | 0.045 | 324 | 0.120 | 5704 | 2916 |
| 64 | 81 | 9x9 | 27=9x3 | 2777 | 43.4 | 0.039 | 324 | 0.105 | 5733 | 4096 |
| 81 | 81 | 9x9 | 27=9x3 | 2906 | 35.9 | 0.033 | 324 | 0.088 | 5797 | 6561 |
| 82 | 243 | 27x9 | 27=9x3 | 6683 | 81.5 | 0.074 | 972 | 0.262 | 45560 | 6724 |
| 100 | 243 | 27x9 | 27=9x3 | 7015 | 70.2 | 0.064 | 972 | 0.226 | 46755 | 10000 |
| 121 | 243 | 27x9 | 27=9x3 | 7408 | 61.2 | 0.056 | 972 | 0.196 | 48267 | 14641 |
| 131 | 243 | 27x9 | 27=9x3 | 7582 | 57.9 | 0.053 | 972 | 0.185 | 48451 | 17161 |

Extra [M]: n = 243 gives 8720 edges (35.88 n, 0.0326 of 1100n, degree 972); n = 729 gives 26202 edges (35.94 n, degree 972) (printed by `census.census`, not saved in census.json).
Result [D from M]: edges are linear in N, about 35.9 N when n = N and at most about 3x that just above a power of 3, i.e. **between 0.026 and 0.074 of Theorem 3's 1100(d+1)** for n = 13..131; degree is at most 0.262 of 136(d+1)^{3/4}. So the char-2 gadget meets O(n) edges with a constant well below the integer one **under this wire-counting model** (Bochum's own accounting may count additional edges, e.g. canonical-form checks; I did not recompute their 1100). Caveat that matters for the SAT arm: the pointwise products are schoolbook inside black boxes, and their AND count (about 8 N^{1.5}) exceeds the schoolbook n^2 for every n in the table below n = 81 (e.g. n = 19: 1411 vs 361; n = 131: 48451 vs 17161). At the pilot sizes the gadget adds nonlinear work.

## 3. Identity checks (a) [M: `raw/identity_checks.json`, `raw/lean_L01Index.out`]
- x is a principal 3N-th root of unity in B_N for N = 3, 9, 27, 81 (Python, exhaustive character sums); Lean `Char2Fourier.lean` already has N = 3, 9.
- Four-step (Cooley-Tukey) DFT == direct DFT over B_{N1}: true for nu = 3, 4, 5 (L = 9 = 3x3, 27 = 9x3).
- No wraparound: carry-less product == ring product for all n <= N (N = 3, 9, 27); a wrap is observed for operands of n > N bits (found within 200 random draws for N = 3, 9, 27). Lean: `no_wrap` (2n-1 <= 2N from n <= N), all n.
- Lean: `fourstep` (all naturals) `(Q i1 + i2)(k1 + P k2) = Q i1 k1 + i2 k1 + P i2 k2 + PQ (i1 k2)` and `fourstep_mod`; inverse block read-off (c from r = c(y^{2M}+y^M) mod y^{3M}-1) by `decide` for M = 3, 9.
- End-to-end: the emitted XOR/AND circuit equals F_{2^n} multiplication on random inputs for n = 5, 9, 15, 17, 19 (`src/test_gadget.py`, passes). Not in Lean. Parsimony of the reduction is trivial in char 2 (a deterministic linear map, nothing guessed); not formalised.

## 4. SAT arm (b)
Instances [M: `work/inst/manifest.json`]. **These are not the frozen corpus files**: generated fresh with `pdp_anf.py` (curve y^2+xy=x^3+x^2+1, m = 3, S4 system) and my own seeds `random.Random('L01-n-l-mode-seed')`. Cells (n, l): (11,3), (13,4), (15,5), (17,6), (19,7); (19,7) is not a corpus cell (corpus has (19,6)). Moduli x^11+x^2+1, x^13+x^4+x^3+x+1, x^15+x+1, x^17+x^3+1, x^19+x^5+x^2+x+1 (irreducibility checked). 8 uniform-target and 8 planted-target instances per cell. UNSAT/SAT label = WDSat `-b` verdict; all finished runs of all arms agree with it (0 disagreements, `raw/analysis_output.txt`). l = 3, 4 are context only (advisor-flagged fixed-overhead region); the decision cells are l = 5, 6, 7.
Arms: wd_b = WDSat `-b` (symmetry on, GE off, frozen-contract flags), wd_bx = `-b -x`; cms_std = CryptoMiniSat on the standard descended ANF converted to XOR-CNF (`anf2cnf.py --fmt xor --sym`); cms_gad = same linking equations, same symmetry breaking, same variables x,e, but the four field products of S4 (E3*c, E3*b, E3*a, a*c) replaced by the gadget circuit (`gen_instances.py: gadget_s4`); linking equations stay schoolbook. cms_*_gj = same with `--maxmatrixcols 100000 --maxmatrixrows 100000 --autodisablegauss 0`. **Default CMS does not use Gauss-Jordan on the gadget CNF** (log: "Too many columns in matrix: 4030 -> set usage to NO"; std gets 1 matrix); the `_gj` arms force it on.

Mean solver conflicts per instance [M: `raw/analysis_output.txt`] (finished/total in brackets, all finished):
| arm | l=5 UNSAT | l=6 UNSAT | l=7 UNSAT | slope l5-7 (95% CI, bootstrap over instances) |
|---|---|---|---|---|
| wd_b | 5522 (4) | 43884 (5) | 351264 (3) | **2.996** [2.992, 2.998] (atlas: 2.9985) |
| wd_bx | 3406 (4) | 31834 (5) | 284485 (3) | 3.192 [3.186, 3.198] |
| cms_std | 14659 (4) | 104503 (5) | 1168246 (3) | **3.158** [2.948, 3.444] |
| cms_std_gj | 14659 (4) | 90540 (2) | 868085 (2) | 2.944 [2.741, 3.231] |
| cms_gad | 39294 (4) | 183582 (5) | **not finished** (3 of 3 stopped at 165 CPU s / 630 s wall kill) | l5->l6 only: 2.22 bits |
| cms_gad_gj | 55206 (4) | 157790 (2) | not run to completion (not attempted: out of budget) | l5->l6 only: 1.52 bits (n=2) |

Planted-SAT cells, conflicts (mean): wd_b 1834 / 15840 / 194862 (n=12/11/1; slope l5-7 3.37 [3.14, 3.73] on a single l=7 instance); cms_std 8412 / 42634 / -- ; cms_gad 25983 / 66016 / --; cms_std_gj 3874 / 51452; cms_gad_gj 20349 / 48541 (n=4/6). No planted l=7 was run for the CMS arms. Planted cells are heavy-tailed (number of true solutions varies), treat the SAT-cell slopes as indicative only.

Paired gadget/standard ratio under CMS (same instances, `raw/analysis_output.txt`):
| cell | conflicts ratio l=3 / 4 / 5 / 6 | CPU ratio l=3 / 4 / 5 / 6 |
|---|---|---|
| UNSAT | 11.3 / 4.3 / 2.7 / 1.76 | 27 / 21 / 9.7 / 4.35 |
| planted SAT | 7.9 / 7.4 / 3.1 / 1.55 | 8.9 / 48 / 10.2 / 5.1 |
| UNSAT, GJ on both | -- / -- / 3.8 / 1.74 (n=2) | -- / -- / 21 / 6.7 |
At l = 7 [M]: cms_std UNSAT needs 140 CPU s on each of 3 instances (1.19M, 1.15M, 1.16M conflicts), while cms_gad had not finished at 165 CPU s on any of them. So the gadget arm is slower than the standard arm in CPU at every size measured, and **at l=7 its conflict count is unknown (censored, no conflict count was recorded at kill)**.

## 5. Decision rule applied
Rule: slope >= 3 bits per unit l at m = 3 -> close as a constant; slope with CI < 2.5 -> reopen/extend.
- Reference arms behave as the atlas says: WDSat `-b` 2.996 [2.992, 2.998] reproduces 2.9985; CMS on the standard encoding 3.16 [2.95, 3.44] at l = 5..7 (the all-l slope 2.69 is pulled down by the small-l region, which is why l = 5..7 is used).
- Gadget arm: the rule's three-point slope at l = 5..7 **could not be computed** (l = 7 not finished). Available: l5->l6 slope 2.22 (UNSAT conflicts, n=4,5), 1.34 (planted, n=12,11), 1.52 (GJ, n=2). Against the standard arm's 2.83 on the same step. The gadget/standard conflict ratio falls with l (UNSAT 11.3 -> 4.3 -> 2.7 -> 1.76), and at l = 6 with Gauss-Jordan forced on, planted SAT is already at parity (0.94).
- Confound that I cannot remove here: n is tied to l (n = 2l+5) and the gadget circuit is padded to N = 27 for every cell, so its size is nearly constant (4040 to 6207 variables for l = 5..7 against 510 to 1113 for the standard CNF) and a fixed overhead floor flattens the curve at small l. The floor is about 6.7k conflicts at l = 3, i.e. 17% of the l = 5 value, so it cannot explain the whole difference, but it is not excluded as the cause of the slope deficit.
- Not claimed: no cell shows the gadget faster than the standard encoding in conflicts or CPU, so there is no absolute gain at pilot scale; the observable is only a lower growth rate at l = 5->6 in conflicts.

Verdict: **undecided** (not "closed", not "reopen"). The pre-registered slope CI is unavailable; the one measured step is below the standard arm's, which is the reopen direction, but with 2-5 instances, one step, a padding confound, and a CPU ratio still >= 4 in favour of the standard encoding.

## 6. One command a skeptic should re-run
```
cd /Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/L01 && python3 src/test_gadget.py && python3 src/census_test.py && python3 src/census.py | tail -3 && python3 src/analyze.py | sed -n 1,12p
```
(gadget==field multiplication, identity checks, census, then the reference-arm slopes from the saved runs). To reproduce a SAT-arm cell: `python3 src/gen_instances.py work/inst2 8` then `python3 src/run_arms.py work/inst2 out.jsonl cms_std,cms_gad 300 'n15l5-' 3` (WDSat binaries via `src/build_wdsat.sh work/wd_l5 502` etc.).

## 7. Next step
Decide the slope cleanly: (1) l = 7 and l = 8 gadget UNSAT cells with `--maxconfl` caps (so a censored run still yields a conflict count) and >= 600 CPU s, GJ forced on for both arms, >= 6 instances each, on an idle host; (2) remove the padding confound by using the same N for all cells and also a std-arm control that pads variables to the same count (a "same-shape null"), or add n-fixed cells (vary l at fixed n = 19); (3) if the gadget slope CI stays < 2.5 at l = 5..8, freeze a protocol and extend to n = 23, 31 as the lead specifies; otherwise close into `LEAD-ICB-decomposition-encoding-and-sat` with the census constants recorded.

## 8. Limits stated plainly
- Instructions retired not measured (sandbox); CPU time on a loaded host.
- Instances are fresh, not the frozen 60-input corpus; cell (19,7) is outside the corpus.
- Only the four S4 products use the gadget; linking equations are schoolbook in both arms; e variables remain free in both.
- Census edges follow the stated wire model, not Bochum's accounting; "d+1 = n" is an assumption.
- Lean covers index arithmetic only; gadget correctness is numerical.
- Old no-GJ gad job continued running in the background after I could not kill it (no process list in the sandbox); `raw/arms_l67_gad.jsonl` holds whatever finished and may keep growing.
