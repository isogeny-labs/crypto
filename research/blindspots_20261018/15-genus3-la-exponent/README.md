# Item 15: the linear-algebra exponent of genus-3 double-large-prime IC

**Verdict: ADVANCES at the primitive level only (arm d, automatic filter choice), TRADES for arm e
(committed round-4 records), REGRESSES for arms b and c. The LA exponent is not lowered by any arm
(level: primitive; exponent: no; constant: no).** The gap to the matched rho is not closed by this item.

Frozen protocol: [`PROTOCOL.md`](PROTOCOL.md) with Addendum A. Instrument:
`examples/blindspots_15_genus3_la.rs`. Records: `raw/`. Decision output: `raw/analysis.json`.
Graph: `fig-ratios.svg` / `fig-ratios.png`; PDF: `REPORT.pdf`.

## Result by arm (frozen rule, applied mechanically by `analyze`)

All numbers measured (counted operations at c_ell = 1, b = 16..20, curves 0 to 3), same relation sets
across arms, ratios paired to arm a = incumbent `r2:lpsgebw2` (SGE + block Wiedemann (2,1)). Intervals are
95 % bootstrap over curves (2000 resamples); with 4 curves they are intervals over curves, not over walks.

| arm | LA exponent (b=16..20) [95 %] | exponent vs a (paired) | LA ratio at b=20 [95 %] | whole ratio at b=20 [95 %] | verdict |
|---|---|---|---|---|---|
| a incumbent | 1.3846 [1.3797, 1.3904] | 0 | 1 | 1 | reference |
| d `lpsgebw2*` (auto cx, mw) | 1.3787 [1.3741, 1.3842] | -0.0059 [-0.0067, -0.0052] (MATCHES by rule: intervals overlap a's) | 0.9862 [0.9836, 0.9876] | 0.9949 [0.9940, 0.9954] | ADVANCES (primitive) |
| b `lpsgebw4x4@cx` | 1.3850 [1.3802, 1.3906] | +0.0004 [+0.0001, +0.0007] (MATCHES) | 1.4758 [1.4752, 1.4770] | 1.1752 [1.1744, 1.1761] | REGRESSES (constant, primitive) |
| c `lpsgebl4@cx` | 1.4019 [1.3971, 1.4073] | +0.0173 [+0.0167, +0.0180] (REGRESSES) | 1.7833 [1.7811, 1.7873] | 1.2884 [1.2865, 1.2906] | REGRESSES (exponent, constant, primitive) |
| e `lpsgebw2o70c@32` (reused, round 4) | 1.4658 [1.4605, 1.4713] vs its own a 1.3779 | +0.0879 [+0.0847, +0.0911] (REGRESSES) | 0.7111 [0.7076, 0.7153] | 0.9467 [0.9451, 0.9485] | TRADES |
| n `lpsgebw2!1` (null) | identical to a | 0 | 1 (b<=19) | 1 (b<=19) | MATCHES |
| aa (independent walk, null) | 1.3914 vs a 1.3879 | +0.0035 [-0.0056, +0.0115] | 1.0040 [0.9956, 1.0146] | 1.0023 [0.9993, 1.0065] | MATCHES |
| bd, cd, c0 (diagnostics, b<=18) | no exponent fit (3 sizes) | | LA 1.43 / 1.63 / 1.26 at b=18 | whole 1.18 / 1.26 / 1.11 at b=18 | INCONCLUSIVE by rule |

What this says, and no more:

* **Constant: no arm reaches the 1.2x whole-solve threshold.** The best is arm d at 0.995x (0.5 %), arm e 0.947x.
  Arms b and c are 1.18x and 1.29x more expensive.
* **Exponent: no arm lowers it.** Arm d's paired difference is -0.006 (CI excludes 0) but its interval
  overlaps a's, so the frozen rule says MATCHES (exponent). Arm e raises the LA exponent by +0.088 (it saves
  most at small b, 0.56x at 2^16, and the saving shrinks to 0.71x at 2^20), as round 4 found.
* **Primitive: arm d's LA cost is 1.4 % lower at b = 20** (0.9862 [0.9836, 0.9876]). This is a real counted
  saving on these instances, with the SGE candidates of all 16 `(cx, mw)` grid points charged to the arm
  (Addendum A1), but it is small. It is stated as ADVANCES (primitive), not as an exponent or constant advance.
* **Block Wiedemann (4,4) and block Lanczos (width 4) lose single-threaded**, as the round-2 argument said for
  symmetric blocking: matrix-vector products do not fall, the generator/dense products rise (about 12 and 21
  products per N^2 against 6). The width control c0 (Lanczos width 1) is 1.26x a at b = 18, so the loss is not the
  width alone; block Lanczos here is the GF(ell) conjugate-direction version, not Montgomery's GF(2) algorithm
  (Deviation 4), and the result says nothing about GF(2) block Lanczos with word-parallel vectors.

### Local exponents (counted cost against q, consecutive sizes; measured)

| arm | 16 to 17 | 17 to 18 | 18 to 19 | 19 to 20 | whole, 19 to 20 | whole minus 4/3 |
|---|---|---|---|---|---|---|
| a (LA) | 1.487 | 1.404 | 1.350 | 1.304 | 1.396 | +0.063 |
| d (LA) | 1.489 | 1.376 | 1.356 | 1.308 | 1.398 | +0.065 |
| b (LA) | 1.473 | 1.419 | 1.347 | 1.303 | 1.381 | +0.048 |
| c (LA) | 1.482 | 1.463 | 1.351 | 1.306 | 1.376 | +0.043 |

Reading: the local LA exponent of every arm falls from about 1.49 at 2^16 to about 1.30 at 2^19..2^20, below 4/3.
That is a small-size effect (the LA cost at these sizes is dominated by sparse and dense terms that change with the
matrix shape and `cx`), not evidence that LA scales as q^1.30; the five-size fit slope is 1.38 to 1.40. The distance
remaining to the relation-phase exponent 4/3 is therefore about +0.05 to +0.07 on the whole solve at 2^19..2^20.

## Distance to the matched rho (b = 20, curves 0 to 2, whole-solve cost over the measured rho record, geometric mean)

Rho reference: committed measured `rho` records of `research/ic_improve_20261014/04-genus3-2lp/results/run-20261014/`
(one run per curve). Priced at c_ell in {0, 0.1, 1} per AGENTS.md section 5a (the unpriced native work of the IC arms is
the mod-ell products of LA and SGE; the rho record is used as measured at every price).

| arm | c_ell = 0 | c_ell = 0.1 | c_ell = 1 |
|---|---|---|---|
| a (derived: d's value divided by d/a's ratio) | about 0.457 | about 0.484 | about 0.723 |
| d | 0.4573 | 0.4835 | 0.7197 |
| b | 0.4573 | 0.4966 | 0.8502 |
| c | 0.4573 | 0.5048 | 0.9321 |
| e (its own cell set, curves 0..2 of the committed records) | 0.4956 | 0.5146 | 0.6852 |

The IC whole cost is below the single measured rho run on these three curves at every price in the band, for every
arm. **This is not a claim of beating rho:** (i) rho is one measured run per curve, whose cost is a heavy-tailed random
variable (the rho model `sqrt(pi ell / 4)` steps is about 16 % below the measured value on curve 0), (ii) the cells are
toy scale (q about 2^20, group order about 2^60), (iii) the whole-solve exponent gap to 4/3 above is still positive
and a short-range fit says nothing about the crossover at cryptographic sizes, and (iv) at c_ell = 0 the arms tie by construction (equal relation phases).
Order between two arms is stated only where it holds at c_ell = 0.1 and 1: d < a holds (0.9992 and 0.9949, tie at 0); e < a
does **not** hold across the band (whole ratio 1.084 at c_ell = 0, 1.064 at 0.1, 0.947 at 1, 0.771 at 9), so no order
of e against a is stated; b and c are above a at every priced c_ell > 0.

## Method

Same relation collection per (b, curve) for every arm (round 4's R1-driven walk, cx per size as the incumbent, kappa
1.9027, 1.7319, 1.5763, 1.4348, 1.3059 for b = 16..20); arms differ only in the LA phase. Arm d runs the SGE filter for
16 `(cx, mw)` candidates per check and solves the one with the lowest modelled solver cost (an online rule reading only
the instance's matrix; Addendum A1). Cost unit: `M_q + ceil(log2 q) I_q + c_ell M_ell` over relation and LA phases.
Nine frozen cards (cards 01 to 09 in this directory), each builds its own binary with plain `rustc` under
`prlimit --as=4294967296`, one thread, gates against the committed round-4 records, deletes the binary. Cards 08 and 09
(b = 20, curves 2 and 3) ran in this final round; their receipts are `raw/receipt-card-08.json` and `-09.json`.
Total measured card time 2341 s (about 39 CPU-minutes, from the nine receipts), inside the 2 CPU-hour budget.

## Controls

* **Gate 0 (bit-for-bit against committed round-4 `r2:lpsgebw2@cx` records):** GATE OK in every card including
  `raw/card-08-gate0-b20.txt` and `raw/card-09-gate0-b20.txt`; `analyze` reports 0 duplicate mismatches.
* **Positive control, solvers:** `raw/selftest.txt`, 120 checks, 0 failures; planted solutions recovered by every solver,
  tampered right-hand sides change the value, `lpsgebw2x1` equals the existing (2,1) code value for value and counter for counter.
* **Positive control, detector:** arm e is below 1 in LA at every size (0.56, 0.59, 0.63, 0.67, 0.71), the committed
  round-4 values.
* **Negative control:** arm n has the same counted cost as a (identical algorithm; only projections change), so it
  ties exactly: it shows the pipeline does not invent a difference but cannot measure noise. The informative null is
  `aa` (independent walk): LA ratio 0.9875 to 1.011, CI at b = 20 [0.9956, 1.0146]; exponent difference +0.0035 [-0.0056, +0.0115]
  covers 0. Note the aa spread (about 1 %) is of the size of arm d's LA gain (1.4 %): d's interval is a paired interval on
  identical matrices over 4 curves and is narrow because the comparison is deterministic given the matrices, but it does
  not show that the 1.4 % persists on other walks; d's ratio is 0.98 to 1.00 across b = 16..20 and is not monotone.
* **Correctness:** every arm's logarithm was checked (`k phi(G) = phi(T)` and against the planted `k`); all records are
  `correct: true`; cells per size are 4 for a, b, c, d, n, aa (aa at b = 20 has 3).

## Deviations and how this relay was handled

* Size range 2^16..2^20 (not 2^22): card-time and memory rules (Protocol Deviation 1). No verdict at 2^21 or 2^22.
* Training stage withdrawn and arm d tuned online (Addendum A1). Arms b and c run at the incumbent's `cx(b)`; the
  auto-filter variants bd, cd are diagnostics for b <= 18 and give no exponent verdict.
* Arm e is reused (round 4 C3, tuned there). Its cell set is the committed cells (3 curves at b <= 17, 5 at b >= 18) and the
  `a` it is compared to is a over the same cells (its a exponent is 1.378, not 1.385).
* The `aa` and committed round-4 `aa` records are both read where present; duplicates agree (0 mismatches).
* **Relays:** the two executor texts were saved as `raw/relay-1.txt` and `raw/relay-2.txt` in condensed form (command
  argv lines for the run were shortened; every digest quoted was checked against the files on disk and the receipts).
  `python3 tools/task_card.py check-relay` for cards 08 and 09 reports CARD FAIL: the card's own lint now fails because its
  output files `raw/b20_c2_r2.jsonl` and `raw/b20_c3_r2.jsonl` exist in the repository and are not pinned by `binds`
  (expected after a run, the same as for cards 01 to 07). The relay was reported, not repaired. Receipt status for both is
  completed, the receipt files hash to the digests the executors quoted (884005ca..., d9d63707...), and the output files hash
  to def811ee... and e1f396b7....
* The analysis was run with a binary built from the committed instrument with `-C opt-level=3` in the scratch directory
  (not a card), as Addendum A3 allows.

## Caveats

* **Host:** shared 4-vCPU container, load about 3, no isolation level earned (no L0 pinning or isolab run); the metric is
  counted operations, so wall time is descriptive only (`la_wall` in the records) and no wall-time claim is made.
* **Toy scale:** q = 2^16..2^20 with a 5-point fit; the exponent estimates have an interval over 4 curves only and the local exponents
  show strong size dependence, so a fitted exponent is not an asymptotic one.
* **Scope of the null controls** is above. The CIs are paired bootstrap over curves and do not include walk-to-walk variation.
* Rho comparison is against single measured runs and the unpriced work of the rho arm is not itself repriced (section 5a
  band applied to the IC arms' mod-ell work).
* Arms b and c lose only in this single-threaded counted model; a multi-core or GF(2) word-parallel setting is not tested.

## Files

`PROTOCOL.md`, `card-01..09.yaml`, `orders.csv`, `raw/*.jsonl` (records), `raw/receipt-card-*.json`, `raw/card-*-gate0-*.txt`,
`raw/selftest.txt`, `raw/analysis.json`, `raw/relay-1.txt`, `raw/relay-2.txt`, `plot_report.py`, `fig-ratios.svg`,
`fig-ratios.png`, `REPORT.pdf`, `build_pdf.py`.
