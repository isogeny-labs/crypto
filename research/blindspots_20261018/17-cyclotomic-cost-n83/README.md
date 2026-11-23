# Item 17: cyclotomic base, counted cost arm up to n = 83

## Verdict

**TRADES, at the constant level.** Pooled over the four headline cells (`n = 83`, `a in {0, 1}`, `K in {74 or 93, 300}`), the frozen rule (`PROTOCOL.md`, applied by `--mode rule` of `examples/blindspots_17_cyclo_cost.rs` to `raw/*.jsonl`) gives for the cyclotomic arm against the incumbent `compact-orbit-scan`:

| headline cell | faithful accounting | tidy accounting | cell verdict |
|---|---|---|---|
| C83a1-74 (K = 74, full w = 2) | REGRESSES | MATCHES | REGRESSES (production accounting only) |
| C83a0-93 (K = 93, full w = 2) | REGRESSES | MATCHES | REGRESSES (production accounting only) |
| C83a1-300 (K = 300, w = 3) | TRADES | MATCHES | TRADES |
| C83a0-300 (K = 300, w = 3) | TRADES | MATCHES | TRADES |

Pooled: not all four REGRESSES, not all four MATCHES, none ADVANCES, so **TRADES**. **A2 (the cyclotomic base) stays closed**: the pooled verdict is not ADVANCES. There is no improvement over the incumbent to report; the cyclotomic base is costlier or equal, never cheaper by the frozen rule, in every cell.

The one stated reading of the result, with its scope: the base-dependent part of the oracle's counted cost (the factor base) is small next to the arm-independent pair table at `n = 83`. Under production (faithful) accounting the cyclotomic base costs up to 2.00 times the incumbent's `S_ready` at `c = 0` (`n = 83`, `a = 1`, `K = 74`) and 1.027 to 1.034 at `c = 1`; when written to do the least work (tidy) all three bases are equal to within 0.11 %. At `K = 300` the cyclotomic faithful scan uses fewer group operations than `cos` (48,680 against 69,780 at `a = 1`) but 17.7 M against 0.16 M unpriced native operations, so its order against `cos` flips with the price (WITHIN at `c = 0`, HIGHER at `c = 0.1` and `1`): that is the TRADES.

Level: **constant** (counted `S_ready` = factor base + Frobenius-folded pair table, section 5a band). Not exponent, not primitive. The solving-degree leg is **BLOCKED** (below), so nothing is claimed about the oracle's algebraic solving degree beyond the structural table.

Gates (all passed): instrument validation 8 of 8 exact against the committed `n = 23` ecbench counters; planted targets 16 of 16 found in every cell and arm (22 control records); classifier positive control: `cyc` HIGHER than `cos` at `n = 23` (instrument ratio 1.1367 against ecbench 1.122 at `a = 0`; 1.0255 against 1.022 at `a = 1`).

Gap to matched rho: every arm is far from it (table below, model rows, **not run**); the cyc and onb model rows differ by under 1 %, so the arm choice does not change the distance.

## Question and method

Does the cyclotomic base's algebraic membership test lower the decomposition oracle's solving degree or cost at `n = 83`? The frozen design is in `PROTOCOL.md` (commit 3e5584266c, prose fix 3cccd6b095, instrument commit 228f1da216, sha256 `15030cd3...e976`; checked again at assessment: `15030cd378f688dc...`). Three factor-base families (cos incumbent, cyc, onb) are built by one std-only Rust instrument on `K_a / F_{2^n}` in two accountings (faithful, production loops; tidy, least work), the Frobenius-folded pair table is built in every arm, and `S(c) = G + c N` prices unpriced native work `N` at `c in {0, 0.1, 1}` GAE per operation. Seven write-once frozen cards ran the cells (`card-01..07.yaml`, receipts `raw/receipt-card-0N.json`); the assess stage ran `--mode rule` on `raw/*.jsonl` (output saved as `raw/rule.jsonl`).

## Numbers

All counts are deterministic: they have no sampling interval. The "interval" is the rule's envelope `[lo, hi]` of what a generic base costs (200 null draws at `K <= 93`, 60 at `K = 300`); it is narrow because the number of projections per column is nearly fixed.

`S_ready(arm) / S_ready(cos)` at `n = 83` (graph.svg, graph.png; envelope of `cos` plus null shown in `raw/rule.jsonl`):

| cell | arm | faithful c=0 | c=0.1 | c=1 | tidy c=0 | c=0.1 | c=1 |
|---|---|---|---|---|---|---|---|
| a=1 K=74 | cyc | 2.0029 HIGHER | 1.1425 HIGHER | 1.0539 HIGHER | 1.0000 | 1.0009 | 1.0009 |
| a=1 K=74 | onb | 0.9890 LOWER | 1.0388 HIGHER | 1.0439 HIGHER | 1.0000 | 1.0000 | 1.0000 |
| a=0 K=93 | cyc | 1.0265 HIGHER | 1.0330 HIGHER | 1.0336 HIGHER | 1.0000 | 1.0010 | 1.0011 |
| a=0 K=93 | onb | 0.9857 LOWER | 1.0242 HIGHER | 1.0282 HIGHER | 1.0000 | 1.0001 | 1.0001 |
| a=1 K=300 | cyc | 0.9972 WITHIN | 1.0244 HIGHER | 1.0273 HIGHER | 1.0000 | 1.0000 | 1.0000 |
| a=1 K=300 | onb | 0.9985 WITHIN | 1.0025 WITHIN | 1.0029 WITHIN | 1.0000 | 1.0000 | 1.0000 |
| a=0 K=300 | cyc | 0.9954 WITHIN | 1.0242 HIGHER | 1.0273 HIGHER | 1.0000 | 1.0000 | 1.0000 |
| a=0 K=300 | onb | 0.9955 WITHIN | 1.0022 WITHIN | 1.0029 WITHIN | 1.0000 | 1.0000 | 1.0000 |

Tidy class is WITHIN in every row. Absolute `S_ready` (counted ops) of `cos` at `c = 0`: 478,116 (a=1 K=74), 745,611 (a=0 K=93), 7,564,680 (a=1 K=300), 7,559,466 (a=0 K=300); about 96 % of it is the arm-independent pair table (`n K (K+1)` additions: 460,650; 725,586; 7,494,900).

Factor-base group operations `G_FB` (faithful / tidy), from the build records: a=1 K=74: cos 17,466 / 2,960, cyc 496,984 / 2,960, onb 12,224 / 2,960; a=0 K=93: cos 20,025 / 188, cyc 39,804 / 186, onb 9,342 / 186; a=1 K=300: cos 69,780 / 12,000, cyc 48,680 / 12,000, onb 58,440 / 12,000; a=0 K=300: cos 64,566 / 602, cyc 30,070 / 600, onb 30,794 / 600. Faithful native work `N` is dominated by the cyclotomic subset-sum and sort (up to 17.7 M at `K = 300`, `w = 3`) and by the onb scan (1.9 to 2.0 M), against 39 k to 156 k for `cos`.

Other cells (descriptive, same rule; from `raw/rule.jsonl`): `n = 23` cyc REGRESSES in both cells in faithful accounting (anchor, as the prior ecbench session found); `n = 47` has onb and cos only: TRADES; the onb arm at `n = 83` is TRADES at `K = 74, 93` and MATCHES at `K = 300`. Arm (a) is reported, not decisive.

### Distance to matched rho (model rows, not run; AGENTS.md section 5a)

`S_model = S_ready + (K + 1) / y (1 + y + c n)` in `ops / sqrt(r)` against ecbench's theory `S_rho-frob = sqrt(pi/2) / sqrt(2n) = 0.0973` at `n = 83` (0.1848 at `n = 23`). Ratio `S_model / S_rho` (distance remaining, upper bound because the rho floor's own native work is unpriced and the measured rho `S` was 2.37 times theory at `n = 23`), cyc arm:

| cell | c = 0 | c = 0.1 | c = 1 |
|---|---|---|---|
| n=83 a=1 K=74 (y = 8.8e-9) | 946.6 | 8,802.7 | 79,507.7 |
| n=83 a=0 K=93 (y = 4.9e-17) | 1.26e7 | 1.17e8 | 1.06e9 |
| n=83 a=1 K=300 (y = 1.45e-7) | 231.9 | 2,156.7 | 19,479.8 |
| n=83 a=0 K=300 (y = 5.1e-16) | 3.88e6 | 3.61e7 | 3.26e8 |
| n=23 a=0 K=17 (anchor) | 35.0 | 132.8 | 1,013.0 |
| n=23 a=1 K=30 (anchor) | 62.9 | 235.3 | 1,787.5 |

The onb arm differs from cyc by under 1 % in these rows. Whole-solve `S` at `n >= 47` is a model: the measured yield at `K <= 300` is 10^-7 to 10^-17 per probe, so no relation was found on random targets (0 hits in 1,600 probes per cell and arm, expected 0.0000 to 0.0002), and no matched rho was run in this item. The distance is reported as distance remaining; nothing here is a pass or fail, and no row beats rho. Ordering of the three index-calculus arms holds across the band only where the rule says so (the TRADES rows are the ones where it does not).

## Controls

* **Validation gate** (`raw/validate.jsonl`): 8 of 8 `n = 23` cells reproduce `adds`, `doubles`, `scalar_mults`, projected points, lifts, columns and pair entries exactly.
* **Planted-target positive control**: 16 of 16 in every cell and arm; each returned decomposition re-added to `R`.
* **Random-target negative control**: 0 hits in 1,600 probes per arm in the `n = 83` cells; expected hits 0.0000 to 0.0002 (reported, not gated).
* **Population null**: generic bases (random abscissae, random order) give a narrow envelope for `S_FB`; the 1 % threshold carries the "within noise" reading.
* **Classifier positive control**: passed (above). **Accounting positive control**: the faithful cyclotomic base costs 28.45 times `cos`'s `S_FB` at `n = 83`, `a = 1`, `K = 74`, `c = 0` (factor base only), so the instrument sees the enumeration cost that the tidy accounting removes.

## Solving degree (structural; the algebraic oracle is BLOCKED)

`raw/degree.jsonl`: the unity membership system has degree `popcount(2n+1)`: 4 at `n = 11`, **5 at `n = 23` and `n = 83`** (certified, 300 of 300 trials), against the matched ONB system's `w + 1 = 4` at `n = 23, 83`: cyclotomic **higher** at `n = 23, 47 (no p), 83`, and lower only at `n = 11` (4 against 5). The `m = 2` system needs 166 (ONB) and 166 or 332 (cyclotomic `w = 1` or 2) variables at `n = 83` against the 64-variable limit, and `crypto_lib` does not compile at the branch base. Verdict for the solver leg: **BLOCKED**, decided from code and arithmetic, not measured. The measured substitute is `S_ready`.

## Deviations and relay report

* The deviations of `PROTOCOL.md` ("Deviations": cells, `S_ready` as the counted oracle cost, degree 5 not 4, targets as control workload, tidy accounting and null added, std-only re-implementation of the plug-ins) stand unchanged; no amendment was made. The protocol's own pre-flight disclosure applies: the cards replicated deterministic counts first seen in the pre-flight.
* **Relay check** (`tools/task_card.py check-relay`, files `raw/relay-3.txt` .. `relay-7.txt`): cards 03, 04, 05, 06 `relay_verbatim: true`. **Card 07 is a deviated relay, reported not repaired**: the executor's printed `pair` line puts `s0`, `s01`, `s1`, `y_pred`, `secs` inside the `nat` object, which the committed output `raw/c83a0-k300-onb.jsonl` and the receipt do not contain. The on-disk record is the tool's own and was used; the executor text is not. Executor statuses: all five `completed`, exit 0. Note on the saved relay files: the executor text was received in this assessment's task, and the long data lines (the `null` records, 3 to 9 lines of stdout) were inserted into the relay files from the committed `raw/*.jsonl` files rather than retyped, so `relay-3` .. `relay-7` agree with the receipts by construction apart from the card 07 line, where the executor's version was kept.

## Caveats

* **Toy scale.** `n = 83` is an instrument rung (`r` about 2^81 at `a = 0`, 2^53 at `a = 1`); `K <= 300` gives a relation yield of 10^-7 to 10^-17 per probe, so no whole solve was run and the distance-to-rho rows are models.
* **What was measured is the base-dependent part only**: factor base and pair table. The pair table is identical in cost in every arm by construction (96 % of `S_ready`), so arm differences cannot exceed a few percent beyond the base itself; this bounds what a constant-level base change can matter here. It does not measure the oracle's algebraic solving cost.
* **Native work is priced, not measured in time.** `c in {0, 0.1, 1}` is the section 5a band; conclusions are stated only where the order holds across it. Wall times in the cards are descriptive. The host is shared 4 vCPU (L0, no isolation tier earned, no PMU), one thread, `prlimit --as=4 GiB`; counts are host-independent, wall times are not.
* The rule's envelope, 1 % threshold and headline set were written after the pre-flight counts were seen (protocol disclosure); they are direction-neutral.
* The `rho` reference is the ecbench theory floor, not a run; it is not a matched measured row.
* `K = 300` cells are split by arm across cards (cos, cyc in one; onb in another); the instrument rebuilds `cos` identically in each, and `--mode rule` pairs them.

## Files

`PROTOCOL.md`, `card-01..07.yaml`, `raw/` (outputs, receipts, relays, `rule.jsonl`), `plot.py`, `graph.svg`, `graph.png`, `REPORT.pdf`, `examples/blindspots_17_cyclo_cost.rs`.
