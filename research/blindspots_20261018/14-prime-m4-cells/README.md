# Item 14: first prime-field arity m = 4 cost cells

## Verdict

**REGRESSES, level primitive (the oracle exponent), first measured m = 4 cells.
No arm ADVANCES.** The frozen rule (`PROTOCOL.md`, Decision rule 5 and 6) gives,
per arm:

| arm | rule outcome | measured basis |
|--|--|--|
| (a) enumeration / MITM over a triple table | reference (rule 1: cannot ADVANCE against itself) | T_dec = \|F\| exactly, fitted p-exponent 0.2359 [0.2223, 0.2496] over 4 cells (checks the base sizes only) |
| (b) F4 on S5 + membership polynomials | **REGRESSES** at w = 0.1 and w = 1 (lower bound of the exponent above 0.25 at both); unresolved at w = 0 (F4 performs no group operation, T_dec(0) = 0) | 16-bit ladder b_x = 3..8: log2 madds on log2 \|F\| slope **11.031 [11.009, 11.053]** (6 points, 4 df); p-exponent = slope/4 = 2.76 [2.75, 2.76] if \|F\| = p^(1/4). Threshold slope 1. |
| (c) PKM tower oracle (reused, not re-measured) | **MATCHES by the letter of the rule, and uninformative**: 3 points (\|V\| = 4, 8, 16), 1 df, slope 9.545 [0.452, 18.639], e_c = slope/4 = 2.39 [0.11, 4.66], which contains 0.25. Both measured increments are steep (x1764 for \|V\| 4 to 8, x316 for 8 to 16, i.e. 10.8 and 8.3 per doubling). The interval spans the threshold only because it has one degree of freedom; the point estimate is far above it. | committed rounds 2/3 |
| (d) Coppersmith / determinant condition | **BLOCKED** by reach (a mathematical limit of the determinant condition, not a tool limit): p^(delta_max) < 2 at every design cell | S5 delta_max 0.0081 at level 6 (asymptote 0.0125) against the 0.25 that B = p^(1/4) needs |

Item verdict (rule 6): no arm ADVANCES at the primitive level, so the strongest
statement is the best-resolved arm: (b) REGRESSES, with a tight interval. The
frozen question, "does any m = 4 oracle's T_dec grow slower than p^(1/4)", is
answered **no** at the measured scale: the only oracle that is both new and
measured here (arm b) grows like \|F\|^11, and the two it could not beat (a:
\|F\| by construction; c: committed rows) are not faster. Nothing here is an
improvement over the index-calculus baseline; there was no m = 4 baseline, and
these cells are it (CLAUDE.md "Success criterion": the first measured cells
become the baseline; the distance to rho remaining is reported below, never as
pass/fail). Toy scale (16 to 28 bit primes); a statement about this engine and
this instance family, not about m = 4 in general.

## Distance remaining to matched Pollard rho

Matched rho measured in the same unit (16 independent logarithms per cell, same
curves, one thread, distinguished points theta = floor(log2 p / 4), jump-table
setup included). S is total GAE (every phase) plus w x native operations, w in
{0, 0.1, 1} (AGENTS.md section 5a). Ratios are S / rho (>1: m = 4 IC costs more).
Derived by dividing two measured means (`raw/summary.md`).

**Arm (a) full index calculus, completed 16 of 16 runs at every design cell**

| log2 p | b_x | mean attempts | S(0) | S(0.1) | S(1) | rho(0) | rho(0.1) | rho(1) | S/rho at 0 | at 0.1 | at 1 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 8 | 155.7 | 3651 | 3997 | 7108 | 566.5 | 602.6 | 927.8 | 6.45 | 6.63 | 7.66 |
| 20 | 15 | 368.4 | 16481 | 18139 | 33060 | 1119 | 1208 | 2006 | 14.73 | 15.02 | 16.48 |
| 24 | 29 | 719.0 | 76401 | 84242 | 154800 | 3708 | 4053 | 7157 | 20.61 | 20.79 | 21.63 |
| 28 | 63 | 1816.6 | 568573 | 627636 | 1159000 | 18913 | 20775 | 37538 | 30.06 | 30.21 | 30.88 |

The gap grows with p: the fitted growth of log2(S/rho) on log2 p is 0.179
(derived, four points, no interval; an indicative number). Measured S(a) scales
like p^0.60, rho like p^0.42 here (rho at these sizes is below its asymptotic
0.5 because of setup and small-n effects). The 16-bit ladder shows S(a)
falling with the base (b_x 3: 15489 down to b_x 8: 3651 at w = 0) and still
falling at the design base, so arm (a)'s own optimum base is larger than the
section 2.4 fixed point; no larger base was run. The ranking "m = 4 IC loses to
rho" holds at all three weights at every cell.

**Arm (b) (own F4)** full IC completed only at b_x = 3 and 4 (16 of 16 each):

| bits | b_x | S(0) | S(0.1) | S(1) | S/rho at 0 | at 0.1 | at 1 |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 3 | 2431 | 1.368e8 | 1.368e9 | 4.29 | 2.27e5 | 1.47e6 |
| 16 | 4 | 1353 | 1.439e9 | 1.439e10 | 2.39 | 2.39e6 | 1.55e7 |

Order between arms (a) and (b) on the full solve at b_x = 3, 4: (b) is lower at
w = 0 (its oracle is free there), (a) is lower by four to six orders at
w = 0.1 and 1. The order does not hold across the band: **unresolved at the
boundary** (w = 0 is the degenerate end). At the 16-bit design cell b_x = 8 the
full solve was not run (about 2.4 CPU-hours); the MODEL oracle-phase row
(attempts 155.7 x 2.7487e10 madds, a lower bound on S) is 4.28e11 at w = 0.1
and 4.28e12 at w = 1, i.e. 7.1e8 and 4.6e9 times rho. At 20, 24, 28 bits F4 did
not finish (below).

**Arm (c) (committed PKM rows, MODEL lower bound at p = 2013265921)**: at least
1.26e20 multiply-adds for the oracle phase at t = 4 (attempts at least 4.7e6 at
2.70e13 each) against rho 5.6e4 GAE, i.e. at least 2.2e15 times rho at w = 1.
The same row for t = 2 is 8.4e15 madds (1.5e11 times rho). Arm (d): no S exists
(BLOCKED).

## Method

Instrument `examples/blindspots_14_prime_m4.rs` (std-only Rust, built by `rustc`
in each card; sha256 f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376;
the `crypto_lib` library does not compile at this base). Four prime-order cells
(`PROTOCOL.md` table), base = first b_x = ceil(p^(1/4)/2) abscissae on the curve
(\|F\| = 16, 30, 58, 126 signed points), 16 random targets per cell, planted
4-sum positive controls. 12 frozen command cards (`card-01..12.yaml`,
`raw/receipt-card-*.json`); arm (c) reused by sha256-bound copy
(`raw/reuse-pkm.jsonl`); arm (d) is the determinant computation plus the
committed S3 probe. Numbers are those printed by
`summarize --dir raw` (`raw/summary.md`); no number in this file comes from
anywhere else except the divisions marked derived.

Arm (b) measured results: 16-bit ladder b_x = 3, 4, 5, 6, 7, 8 (18 targets
each: 16 random + 2 planted), mean multiply-adds 5.48e5, 1.33e7, 1.56e8,
1.16e9, 6.32e9, 2.75e10 (per-target spread under 2%, e.g. b_x = 8: 2.7486e10 to
2.7488e10). The p-ladder at b_x = 5, 6 on the 20-, 24- and 28-bit curves gives
the same counts (1.5569e8 and about 1.159e9) as the 16-bit curve: the cost
depends on \|F\|, not on p, as the protocol assumed. At the design cells F4 was
stopped by its count cap, not finished (status `budget-nnz`, all 16 targets,
deterministically): 20 bits b_x = 15 at step 21 (degree 39, 3.7e7 nonzeros,
4.5e8 madds), 24 bits b_x = 29 at step 16 (degree 53, 2.9e7 nonzeros, 6.7e7
madds), 28 bits b_x = 63 at step 12 (degree 87, 3.6e7 nonzeros, 2.3e7 madds).
Those counts are lower bounds on T_dec and are not comparable across cells (the
cap hits earlier in the elimination at larger b_x); no p-exponent is fitted
over the design cells for arm (b), as Decision rule 2 states.

Arm (d) determinant reach (exact): S5 (d = 8) delta_max 0.000010, 0.002455,
0.004804, 0.007372, 0.008117 at levels 1, 2, 3, 5, 6; asymptote 0.0125;
B_max at 28 bits 1.17 at level 6, so no level reaches a base of two abscissae
at any design cell; p must exceed 2^123 for level 6 to give B >= 2. The control
reproduces the committed S3 values (1/18 = 0.05556, 0.1, 0.119).

## Controls

* `selftest --deep` passed (card 01, `SELFTEST OK`): curves recounted by a full
  character sum, n G = O, S3 vanishes on sums, interpolated S5 equals the
  resultant, full IC and rho recover planted logarithms, reach control matches.
* Arm (a): planted 4-sums found 4 of 4 at every cell; 0 mismatches against the
  independent pair-table cross-check on every random and planted target.
* Arm (b): quotient dimension equals the brute-force root count 18 of 18
  targets at every finished cell (dimension 0 for random targets, 24 or 12 for
  planted); the planted point vanishes on the basis 2 of 2.
* Rho: 16 of 16 logarithms recovered at every cell. Full IC: 16 of 16 at all
  completed cells, arms (a) and (b).

## Deviations and caveats

* All protocol deviations 1 to 14 stand (`PROTOCOL.md`), notably: the base is
  defined by size; arm (b) at design cells is a count-capped lower bound;
  arm (c) is at its own sizes and prime; the arm-(a) fitted exponent has 2 df;
  the F4 is a plain implementation (counts are elimination multiply-adds only)
  and a better engine would shift the constant, not the observed |F|^11 shape
  at this scale; no symmetry reduction (each root appears 24 times).
* Arm (b)'s exponent is a slope in |F| over |F| = 6 to 16 (b_x = 3..8); the
  mapping to a p-exponent assumes |F| = p^(1/4) and is an extrapolation beyond
  these sizes. The F4 in the committed PKM work and the framework were not run
  here, so no claim is made about them beyond their committed rows.
* Arm (c) rows are committed measurements at a different prime and base sizes;
  the interval is 1 df and uninformative (reported as MATCHES only because the
  rule's wording is that the interval contains 0.25).
* Toy scale; host is a shared 4-vCPU container with at most two lanes running
  at once, one thread per run, `prlimit --as=4 GiB`; **host isolation level L0**
  (no PMU, no pinning). Wall time is not a metric; every cost is a count.
* Arm (a)'s T_dec is a count (|F| group additions and lookups per miss), so its
  exponent is a property of the base sizes; its exponent below 0.25 (0.236)
  comes from the base sizes being slightly above p^(1/4) at small p (16, 30, 58,
  126 against 14.8, 28.5, 56.9, 125.6).
* The full-IC target walk correlates consecutive targets (Deviation 11).
* The prime-order cells are one curve per size; across-curve variance is not
  measured.
* Pre-freeze probe runs are disclosed in `PROTOCOL.md`; none is data here.
* **Relay.** `tools/task_card.py check-relay` was run on the saved texts of
  cards 2 and 3 (relay_verbatim true, no verify findings). Card 1's executor
  returned an abridged text (it omitted the JSON row bodies of the stdout
  tails, saying so): check-relay reports **relay_verbatim false, 415 reference
  lines missing, all of them JSON stdout-tail rows** (`raw/relay-1.txt` is
  reconstructed from the saved tool output by dropping those row lines; the
  executor's command and digest lines all matched). The relay texts of cards
  4 to 12 were not saved as files in this stage (the executor texts exist only
  in the orchestration message), so check-relay was not run for them; instead
  every receipt was read: status completed, every command succeeded, and every
  output file's sha256 on disk equals its receipt digest (checked for all 12
  cards). The post-run `lint` of every card now fails ("names repository file
  ... that binds does not pin") because the output files exist; this is the
  linter seeing the run's own outputs, and `check-relay` was therefore called
  through its Python function. Conductor is not installed here; no task was
  obtained.

## Files

`PROTOCOL.md`, `card-01..12.yaml`, `raw/` (records, receipts, `summary.md`),
`plot.py` and `graph.svg` / `graph.png` (laid out from `raw/summary.md`),
`REPORT.pdf` (from `reports/report.typ`), `benchmarks/20261018-blindspots-prime-m4-cells/`
(the cost result's copies).
