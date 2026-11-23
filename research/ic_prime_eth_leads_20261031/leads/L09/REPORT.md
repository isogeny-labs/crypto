# L09 -- BTN census tool and the D.1 cost row for every encoding (status: measured, tooling done)

All numbers below are labelled **[measured]** (produced by the tool or the Rust code in this directory, path given), **[derived]** (arithmetic on measured numbers), or **[plan]**. Nothing outside this directory was modified; patched copies of two Rust sources live in `rust-dump/` and `sweeper-dump/`.

## Headline

1. **[measured]** The tool runs on four lab encodings (descended ANF, prime PDP chained-S3 and quotient-witness/group-law CNFs, the Bochum m=2 hashed network from the Rust crate), plus the lab-format ANF, a CNF form of the descended instance, and two published 131-bit rows. Table below; raw rows in `table2.json`.
2. **[derived]** No row is D.1-admitted. Admission `u0 = lam/2^17 >= lam^(4/5)` is equivalent to `lam >= 2^85`, i.e. s >= 2^(2^85). The largest row here has lam = 21.96 (s = 4.08M); margin log2(u0/lam^0.8) = 0.2*log2(lam) - 17 lies in [-16.33, -16.11] for every row.
3. **[derived]** Even granting D.1 constant c = 1 and its best accounting (black-box vertices, "bb"), `s/sqrt(log2 s)` exceeds the enumeration exponent `(m-1)b/m` by **37x to 45x** for the descended ANF, by **227x to 370x** with quadratic equations split to arity 3, by **3.9e3 to 5.8e3x** for the crate's m=2 Bochum network (b = 5..16 and the published ECCp-131 row), and by **5.1e3 to 4.1e4x** for the prime PDP CNFs. No encoding comes within a factor 30 of enumeration even in the most favourable accounting.
4. **[measured]** Scaling of s with b (log-log slope, split mode): prime PDP CNFs 2.09 to 2.17 over b = 16..32 (two intervals 1.98..2.06 then 2.24..2.32, schoolbook multiplication); crate Bochum m=2 1.19 between b = 16 (128,448 edges) and the published ECCp-131 row (1,564,506 edges). The crate has a constant floor of ~58k edges at b = 5 (`graphs/bochum_m2_p17.json`), so b <= 16 is dominated by fixed Fourier-parameter overhead and its small-b slope (0.78 over b = 6..16, non-monotone: 0.30, 2.60, 0.14, 0.26) is not an asymptotic reading.
5. **[measured]** Interface k vs the lower bound b: k >= b on every row except the lab/Trimoska n=19, l=6 ANF where k = 3*6 = 18 < 19 = b. That instance family cannot have a generic expected solution (a planted "S" instance only); it is a corpus property, not a tool artifact.

## Tool

`btn_census.py` (Python 3, stdlib only). Usage:

```
python3 btn_census.py PATH [--format auto|dimacs|labanf|crate] [--mode split|bb] [--no-share] -b B -m M [--iface-first N]
```

Inputs: DIMACS CNF (also `.gz`) with CryptoMiniSat `x` lines and WDSat ANF lines (`x ... .d v1..vd ... T 0`, `T` = constant 1); the lab `p anf` polynomial format; the Rust crate dump (`rust-dump/examples/btn_dump.rs` writes JSON from `Network::census_json`, my patch). Output (JSON): s, V, Delta, nonlinear ports/vertices, EQ and free vertices, degree-2 invariant check, s/sqrt(log2 s), D.1 block (lam, u0, lam^0.8, admitted, margin), enumeration exponent, ratio, interface k and its source. `run_table.py` builds the whole table; `compact_table.py` formats it; `fits.py` the slopes. Unit test with a hand count: `tests/test_btn_census.py`; independent bb recomputation: `tests/independent_bb_anf.py`.

## Subdivision rule as implemented (read this before trusting any s)

The text of Bochum Remark 1 is **not on disk** (no `bochum.txt` found anywhere on SSD990-2); the rule below is reconstructed from LEADS.md section 2 (parsimonious ripple/equality/copy gadgets with constant degree) and the D.1 statement (wires of degree 2, vertices as black boxes). Treat it as "the standard equality-tree subdivision", not as a verified transcription of Remark 1.

- **R1 fan-out (always applied).** A variable is a wire. With d ports: d >= 3 gives a binary tree of d-2 degree-3 equality vertices and 2d-3 edges; d = 2 gives 1 edge; d = 1 gives 1 edge plus a unary all-ones "free" vertex; d = 0 is not a wire. After R1 every edge has exactly two endpoints (checked: sum of vertex arities == 2s on every row, `degree2_check`).
- **R2 arity split (cap 3, mode `split`).** XOR of a terms becomes a-2 XOR3 vertices (a-3 new wires). An OR clause of w literals becomes w-2 vertices of arity <= 3 (w-3 new wires, functional, so parsimonious). An ANF monomial of degree d becomes d-1 AND2 vertices; identical monomials and prefixes are shared across equations (`--no-share` turns sharing off); the equation is then an XOR chain with the constant folded into the tensor.
- **Mode `bb`** applies R1 only: every constraint stays one black-box vertex (D.1's reading; Delta = largest constraint arity).
- **Constants.** DIMACS variable 1 is absorbed (not a wire) when the first clause is the unit `1 0` (sweeper convention); literal polarity is part of the vertex tensor.
- **Crate rows.** The crate's graph is already a wire graph (each edge has two endpoints); its vertices are never split (their tensors are opaque Fourier/adder gadgets), so `split` == `bb`. Each vertex is classified by probing its predicate: `eq` (accepts exactly all-0/all-1), `xor0/xor1` (accepts exactly one parity), `pin` (arity 1), else nonlinear. Arity <= 12 is exhaustive; arity > 12 is probed (all-0, all-1, every single flip, 4096 random words) which can only err by calling a nonlinear vertex linear.
- **Nonlinear ports** = sum of arities of vertices that are not XOR, equality, pin or free (AND, OR/clause, ANF quadratic equations in bb mode, crate nonlinear vertices).
- **Interface k.** DIMACS/PDP: declared by the generator (`c iface`, the low l bits of each summand abscissa; Trimoska CNF: variables 1..15 by the corpus description). ANF: greedy functional-head search, accepted only after a propagation check, minimum over four head orderings, so it is a verified *upper bound* on the minimum interface. Crate: `Network::interface()` (infinity flag, x and y bits of each A_i, so k is about 2b+1 per summand, not b/m).

## Table (split / bb; `D.1` = admitted?; all "no")

| encoding | instance | b | m | s (split) | s (bb) | V (split) | Delta split / bb | nonlin ports (split) | s/sqrt(log2 s) [split] | enum (m-1)b/m | ratio | D.1 | k vs b |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| descended ANF (WDSat .anf) | Xn15l5-1-S.anf | 15 | 3 | 9,164 | 1,186 | 6,110 | 3 / 27 | 1,380 | 2,526 | 10.00 | 253x | no | 15 >= 15 |
| descended ANF (WDSat .anf) | Xn17l6-1-S.anf | 17 | 3 | 14,447 | 1,681 | 9,632 | 3 / 33 | 2,148 | 3,886 | 11.33 | 343x | no | 18 >= 17 |
| descended ANF (WDSat .anf) | Xn19l6-1-S.anf | 19 | 3 | 14,960 | 1,815 | 9,974 | 3 / 33 | 2,148 | 4,017 | 12.67 | 317x | no | **18 < 19** |
| descended ANF (lab p anf) | n15l5-1-S.anf (lab) | 15 | 3 | 8,195 | 1,188 | 5,464 | 3 / 27 | 1,380 | 2,273 | 10.00 | 227x | no | 15 >= 15 |
| descended ANF (lab p anf) | n19l6-1-S.anf (lab) | 19 | 3 | 13,151 | 1,815 | 8,768 | 3 / 33 | 2,148 | 3,555 | 12.67 | 281x | no | **18 < 19** |
| descended CNF (Trimoska) | Xn15l5-1-S.dimacs | 15 | 3 | 13,719 | 11,040 | 9,495 | 3 / 180 | 3,845 | 3,701 | 10.00 | 370x | no | 15 >= 15 |
| prime PDP chained-S3 m=3 plain CNF | b16_m3_bits-s3_xor0 | 16 | 3 | 906,764 | 906,712 | 627,220 | 3 / 16 | 542,930 | 203,830 | 10.67 | 19,109x | no | 18 >= 16 |
| prime PDP chained-S3 m=3 +native XOR | b16_m3_bits-s3_xor1 | 16 | 3 | 442,852 | 442,800 | 318,324 | 3 / 16 | 232,898 | 102,255 | 10.67 | 9,586x | no | 18 >= 16 |
| prime PDP quotient/group-law m=2 plain CNF | b16_m2_bits-grouplaw_xor0 | 16 | 2 | 346,561 | 346,535 | 239,768 | 3 / 16 | 207,536 | 80,786 | 8.00 | 10,098x | no | 16 >= 16 |
| prime PDP quotient/group-law m=2 +native XOR | b16_m2_bits-grouplaw_xor1 | 16 | 2 | 169,677 | 169,651 | 121,990 | 3 / 16 | 89,324 | 40,709 | 8.00 | 5,089x | no | 16 >= 16 |
| prime PDP bits-s3 m=2 plain CNF | b16_m2_bits-s3_xor0 | 16 | 2 | 418,425 | 418,412 | 289,396 | 3 / 16 | 250,493 | 96,826 | 8.00 | 12,103x | no | 16 >= 16 |
| prime PDP bits-grouplaw m=3 plain CNF | b16_m3_bits-grouplaw_xor0 | 16 | 3 | 576,361 | 576,296 | 398,782 | 3 / 16 | 345,246 | 131,753 | 10.67 | 12,352x | no | 18 >= 16 |
| prime PDP chained-S3 m=3 plain CNF | b24_m3_bits-s3_xor0 | 24 | 3 | 2,090,842 | 2,090,758 | 1,445,092 | 3 / 24 | 1,250,406 | 456,306 | 16.00 | 28,519x | no | 24 >= 24 |
| prime PDP chained-S3 m=3 +native XOR | b24_m3_bits-s3_xor1 | 24 | 3 | 1,011,486 | 1,011,402 | 726,096 | 3 / 24 | 529,686 | 226,469 | 16.00 | 14,154x | no | 24 >= 24 |
| prime PDP quotient/group-law m=2 plain CNF | b24_m2_bits-grouplaw_xor0 | 24 | 2 | 778,513 | 778,471 | 538,244 | 3 / 24 | 465,742 | 175,981 | 12.00 | 14,665x | no | 24 >= 24 |
| prime PDP quotient/group-law m=2 +native XOR | b24_m2_bits-grouplaw_xor1 | 24 | 2 | 378,527 | 378,485 | 271,800 | 3 / 24 | 198,658 | 87,934 | 12.00 | 7,328x | no | 24 >= 24 |
| prime PDP bits-s3 m=2 plain CNF | b24_m2_bits-s3_xor0 | 24 | 2 | 964,350 | 964,329 | 666,412 | 3 / 24 | 576,606 | 216,289 | 12.00 | 18,024x | no | 24 >= 24 |
| prime PDP bits-grouplaw m=3 plain CNF | b24_m3_bits-grouplaw_xor0 | 24 | 3 | 1,294,200 | 1,294,095 | 894,822 | 3 / 24 | 774,399 | 287,220 | 16.00 | 17,951x | no | 24 >= 24 |
| prime PDP chained-S3 m=3 plain CNF | b32_m3_bits-s3_xor0 | 32 | 3 | 4,079,249 | 4,079,133 | 2,817,188 | 3 / 32 | 2,436,816 | 870,493 | 21.33 | 40,804x | no | 33 >= 32 |
| prime PDP chained-S3 m=3 +native XOR | b32_m3_bits-s3_xor1 | 32 | 3 | 1,950,081 | 1,949,965 | 1,398,582 | 3 / 32 | 1,015,692 | 426,609 | 21.33 | 19,997x | no | 33 >= 32 |
| prime PDP quotient/group-law m=2 plain CNF | b32_m2_bits-grouplaw_xor0 | 32 | 2 | 1,499,255 | 1,499,197 | 1,035,748 | 3 / 32 | 895,948 | 331,002 | 16.00 | 20,688x | no | 32 >= 32 |
| prime PDP quotient/group-law m=2 +native XOR | b32_m2_bits-grouplaw_xor1 | 32 | 2 | 720,759 | 720,701 | 517,058 | 3 / 32 | 376,336 | 163,391 | 16.00 | 10,212x | no | 32 >= 32 |
| prime PDP bits-s3 m=2 plain CNF | b32_m2_bits-s3_xor0 | 32 | 2 | 1,898,161 | 1,898,132 | 1,310,640 | 3 / 32 | 1,133,615 | 415,638 | 16.00 | 25,977x | no | 32 >= 32 |
| prime PDP bits-grouplaw m=3 plain CNF | b32_m3_bits-grouplaw_xor0 | 32 | 3 | 2,482,294 | 2,482,149 | 1,714,970 | 3 / 32 | 1,483,654 | 538,571 | 21.33 | 25,246x | no | 33 >= 32 |
| hashed Bochum network m=2 (crate) | bochum_m2_b10.json | 10 | 2 | 116,120 | 116,120 | 20,528 | 132 / 132 | 208,280 | 28,309 | 5.00 | 5,662x | no | 42 >= 10 |
| hashed Bochum network m=2 (crate) | bochum_m2_b12.json | 12 | 2 | 119,060 | 119,060 | 22,250 | 132 / 132 | 211,510 | 28,995 | 6.00 | 4,832x | no | 50 >= 12 |
| hashed Bochum network m=2 (crate) | bochum_m2_b16.json | 16 | 2 | 128,448 | 128,448 | 31,440 | 260 / 260 | 218,490 | 31,180 | 8.00 | 3,897x | no | 66 >= 16 |
| hashed Bochum network m=2 (crate) | bochum_m2_b6.json | 6 | 2 | 59,716 | 59,716 | 14,730 | 132 / 132 | 102,554 | 14,992 | 3.00 | 4,997x | no | 26 >= 6 |
| hashed Bochum network m=2 (crate) | bochum_m2_b8.json | 8 | 2 | 65,004 | 65,004 | 16,630 | 132 / 132 | 110,350 | 16,257 | 4.00 | 4,064x | no | 34 >= 8 |
| hashed Bochum network m=2 (crate) | bochum_m2_p17.json | 5 | 2 | 57,952 | 57,952 | 13,722 | 132 / 132 | 100,546 | 14,569 | 2.50 | 5,828x | no | 22 >= 5 |
| published 131-bit (not rebuilt) | ECCp-131 target graph (2 slots, seed 1) | 131 | 2 | 1,564,506 | 1,564,506 | 71,723 | 1032 / 1032 | - | 344,892 | 65.50 | 5,266x | no | - |
| published 131-bit (not rebuilt) | ECC2K-130 recorded graph | 131 | 2 | 2,694,680 | 2,694,680 | - | 1032 / 1032 | - | 583,028 | 65.50 | 8,901x | no | - |

Notes on the table. `s (split)` / `s (bb)` differ materially only for ANF: black-boxing the quadratic equations cuts s 6.9x to 8.6x but raises Delta from 3 to 27..33, i.e. it hides the contraction cost in vertex tensors that D.1 does not charge to s. CNF/PDP gates are already arity <= 3, so the two modes agree to 0.01%. Row sources:

- Descended ANF: `/Volumes/SSD990-2/crypto/.scratch/icperf-checker-v2-20261019/inputs/TRIMOSKA-ECICB-2024/upstream/benchmarks/Xn{15l5,17l6,19l6}-1-S.anf` (the Trimoska corpus the baseline pins; the lab's own `crypto-worktrees/xenodochial-jang-dc5daf/docs/ic/corpus` has n=15 and n=19 only, run as the "lab p anf" rows; n=17 exists only in the Trimoska corpus). b = n, m = 3.
- Prime PDP CNFs: generated by `sweeper-dump` (copy of `crypto-worktrees/prime-pdp-sat-sweep-20261031/research/prime_pdp_sat_sweep_20261031/sweeper` plus a `dump` subcommand), seed 20261031, family small-x, l = ceil(b/m), symmetry break on, random target; "chained-S3" = `bits-s3` at m = 3 (S3(x1,x2,xt) and S3(xt,x3,xR)), "quotient/group-law" = `bits-grouplaw` at m = 2 (affine addition with quotient-witness modular products). Extra rows (`bits-s3` m=2, `bits-grouplaw` m=3) included. "+native XOR" = XOR gates as `x` lines.
- Crate rows: `graphs/bochum_m2_*.json`, built by the patched crate copy `rust-dump/` (`examples/btn_dump.rs`) for prime-order curves found by `gen_prime_curves.py` (p17 = the crate's own test curve 17,2,2 order 19; b6..b16 random prime-order curves; hash prime s = first prime in [2p,4p], tau = min(s, ceil(s*max(isqrt(r)+1,20)/(r-1))), alpha = 3, beta = 1; parameters in `graphs/*.params.json`). These are the smallest sizes I built; the crate has no smaller emitter.
- Published rows (not rebuilt): ECCp-131 two-slot seed-1 graph s = 1,564,506, 71,723 constraints, Delta 1,032 from `/Volumes/SSD990-2/prime-sat-tensor-review-20261031/hashed_tensor_compile_stats.json` (agrees with `cryptanalysis/docs/hashed-ecdl.md`); ECC2K-130 s = 2,694,680, Delta 1,032 from the same doc. b = 131 is used for both (field degree 131; the ECC2K-130 group order is about 2^130).

## Exact commands

`reproduce.sh` contains all of them. In order: build the patched sweeper (`cargo build --release --offline` in `sweeper-dump`), `prime_pdp_sat_sweep dump --bits B --l ceil(B/m) --m M --encoding E --xor X --out inst/...` for B in 16 24 32, M in 2 3, E in bits-grouplaw bits-s3, X in 0 1; build the patched crate example; `python3 gen_prime_curves.py graphs 6,8,10,12,16`; `python3 run_table.py table2`; `python3 compact_table.py table2.json`; `python3 fits.py table2.json`. The first full run (`table.json`) and the rerun on gzipped inputs after regenerating the graphs (`table2.json`) give an identical compact table (diffed).

## Caveats

- Remark 1 not on disk (see above); cap = 3 and monomial sharing are my choices. s_split depends on them (sharing off, or a different cap, moves s by a constant factor), the ratios to enumeration (>= 227x on split, >= 37x on bb) are far outside any such factor.
- The D.1 cost used is `s/sqrt(log2 s)` with constant 1 and no log factors; it is a lower bound on the D.1 exponent, and the lean `no_crossover` constant c >= 1 only raises the ratios.
- b and m for each row are the instance's own (ANF: n and 3 summands; PDP: curve bit length and summand count; crate: bit length of p and 2 slots). The enumeration exponent is the LEADS section 4 naive one, (m-1)b/m.
- PDP instances use the small-x factor base only (membership = high bits fixed to 0); the Kummer family and `exp-s3` were not dumped. One random target per cell; the CNF size does not depend materially on the target (constants fold).
- Crate classification by probing for arity > 12 (see above). `k` for the crate counts y bits and infinity flags, so "k >= b" there is trivially true and is not the window-index interface.
- ANF `k` is an upper bound (greedy), the others are declared by the generator, none is proven minimal. For the PDP CNFs I did not verify that the declared interface determines the other variables (y witnesses need a square root, not unit propagation).
- The crate dump patch (`Network::census_json`) is in a copy; the crate under `cryptanalysis/` and `crypto/` is untouched. No solver was run; no timing claims.
- Disk: `inst/` holds 24 gzipped CNFs (27 MB), `graphs/` 11 MB.

## Derived reading for L1..L6

- The only encoding whose s grows slower than b^2 is the crate network, but its measured floor (58k edges at b = 5, 128k at b = 16) and 11.9k edges per bit at b = 131 mean that at b <= 32 the schoolbook PDP CNF (quotient/group-law m=2 plain: 1.50M edges at b = 32) is already about the size of the 131-bit Bochum network (1.56M). [derived coincidence, different tasks; PDP excludes hashing and comparison of the hash against tau]
- Native XOR lines halve s on every PDP row (0.47x to 0.49x): the char-2-style XOR advantage L1 wants to quantify appears on the prime side as a factor of about 2 in s and none in the exponent slope (2.09 vs 2.11).
- Nothing here changes the closed verdict: under the D.1 black-box model every encoding sits hundreds to tens of thousands of times above enumeration, and k >= b leaves no slack for an interface-saving encoding (except n19l6, where k < b).

## Skeptic re-run (one command, about 10 s)

```
cd /Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/L09 && python3 tests/test_btn_census.py && python3 btn_census.py inst/pdp_b16_m2_bits-grouplaw_xor0.cnf.gz -b 16 -m 2 | grep -E '"(s|V|delta)"' && for n in 15 17 19; do l=6; [ $n = 15 ] && l=5; python3 tests/independent_bb_anf.py /Volumes/SSD990-2/crypto/.scratch/icperf-checker-v2-20261019/inputs/TRIMOSKA-ECICB-2024/upstream/benchmarks/Xn${n}l${l}-1-S.anf; done
```

Expected: `ok`; s = 346561, V = 239768, Delta = 3; then 1186, 1681, 1815 (independent black-box recount that does not import the tool).

## Next step [plan]

Run the tool on the section-4 candidates L1 and L11 emit (char-2 gadget networks, Karatsuba-structured prime multipliers) and on the Kummer/`exp-s3` PDP family, and add one more column, the largest contiguous XOR-connected component, which is what L4's backdoor census needs. Also fetch the Bochum Remark 1 text and diff the rule above against it (one hour).
