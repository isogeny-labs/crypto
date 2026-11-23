# Validator report: genus 4 on prime-order Jacobians (`research/genus4_prime_order_20261118`)

- **Disposition: validated** (`validator-report.json`: `status: pass`, `independent: true`).
- **Validated commit:** `7c0e84dedacc4acd5d6d587bfc6015fcbd2247ba` on `claude/hyperelliptic-index-calculus-td1fdn` (PR isogeny-labs/crypto#4693).
  Every raw row was read as a git blob at that commit. The working tree was clean; its ignored `.jsonl`/`.log` copies were not used.
- **Freeze:** `6da1e3c5b2`. **Binary commit:** `ca72267899`, the freeze commit's parent.
- **Validator:** a Validator subagent, 2026-11-18. The recompute, the certificates, the prefilter tests and the replay comparison use the validator's own code (`research/genus4_prime_order_20261118/validation/scripts/`). The producer's `analyze.py`, `tune.py`, `curves.py` and `report.py` were read but never run or imported.
- **Standing:** every number in the study is still `[pcc:unverified]` until a pcc claim records it. This report checks the numbers. It does not issue a claim.

Machine-readable: `validator-report.json` (sha256 `559ac7428bcabb01641d7d4b6d0ea811548f61873b1b53ed8b25ecf1006ab9ba`), `replay-certificate.json` (sha256 `7015b18119e7479c349ea7e16dc7bbbf76b922c05eeda74a27cc8a295da565ab`), `VALIDATION.json`. `python3 scripts/validation_gate.py evidence benchmarks/20261118-genus4-prime-order` says `valid`. Working files: `research/genus4_prime_order_20261118/validation/` (`scripts/`, `out/`). Paths below starting `validation/` are under `research/genus4_prime_order_20261118/`.

## Verdict in one paragraph

Every number in `analysis.json` and `tune.json` recomputes exactly from the committed raw rows: 187 cells, maximum relative difference 0, bootstrap intervals included. That holds with the protocol's design and seed and the frozen script's draw order; another draw order moves interval ends by at most about 2e-4 and changes no decision.

The prefilter is sound, both mathematically and as implemented:
- A reducible f always has a rational 2-torsion point.
- The library's `f_irreducible` agrees with the validator's independent distinct-degree test on all 5,000 scanned seeds and on 1,116 constructed polynomials.
- Every seed skipped below each prime's fifth chosen seed carries a checked factor certificate.

The chosen seeds are exactly the five smallest admitted over fully covered ranges. The rebuilt binary has the producer's code, and 73 nonce-drawn run rows replay identically. The validator's own arithmetic certifies all 10 curves, all 60 instance keys and 49 recovered logarithms.

The headline, "x\* below the ideal negation-rho bound at all three prices at both primes", is stated correctly within its scope (F3). The wording needs these corrections:
- "fully charged" omits the walk's uncounted per-step hash and coefficient additions (F2);
- "What is new here" is an unchecked novelty statement (F4);
- the exponents are two-point slopes (F4);
- DEVIATIONS.md's D2 was logged after its data (F1, no effect on selection).

## Checks

| id | requirement | outcome | evidence |
|---|---|---|---|
| C1 | `PROTOCOL.md`, `search.sh`, `curves.py`, `tune.py`, `run.sh`, `analyze.py` hash to `frozen.sha256` at the freeze commit and at the validated commit. `frozen.sha256` and `binary.sha256` are unchanged since. The freeze commit has no data and its parent has no study files. All 66 data files enter later: part A first, in `6caa725bf2`. No `src/`, `Cargo.*` or `crates/` change from `ca72267899` to the validated commit | pass | `validation/out/freeze_check.json` |
| C1b | Each deviation committed before the data it concerns | pass, with F1 | `validation/out/freeze_check.json` |
| C2 | Prefilter soundness: reducible f ⇒ #J even, the implementation, and no wrongly skipped seed below the fifth chosen seed | pass | `validation/out/prefilter_theory.json`, `theory_crosscheck.json`, `prefilter_diff.json`, `prefilter_rust_vs_validator.json`, `prefilter_rust_flags.json`, `select_check.json` |
| C3 | Chosen seeds are the 5 smallest admitted over fully covered ranges (`cell` rows, `tried` counts); residue 3 and the extension blocks are complete | pass | `validation/out/select_check.json` |
| C4a | The producer binary `493ee294…` (hashed and copied at dispatch, before any validator build) equals `binary.sha256`. The validator rebuild from `git archive ca72267899` (5,521 paths, list sha256 `9fcf42d1…`), in a validator-owned tree and target directory, is `ac1dc60d…`. Its `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame`, `.eh_frame_hdr`, `.gcc_except_table`, `.init_array` and `.got` are byte-identical; all 57,320 symbol addresses and types are equal; 36 names differ only by their `.llvm.<hash>` suffix; the build-id differs | pass (the hash does not reproduce across build paths; the code does) | `validation/out/binary_diff.txt` |
| C4b | Nonce `e57128a78a513434ef225010299714d3` (drawn 2026-11-18T21:26:34Z, before any replay), plan `validation/out/plan.json`. Replayed with the rebuild: 73/73 run rows identical in every field except `wall_ns`: part A 24 rows (one cell at each cc = 1.4, 2.0, 2.8, all four SGE settings, so all 12 configurations); part B 12 x and 12 x\* rows at each prime (2 curves × 3 targets × 2 seeds each); 1 Q row (455, k = 0). 12/12 instance rows of the run cells identical, and 3/3 search `cell` rows identical, with their 2 instance rows: two drawn p = 32771 chunks plus the chunk that admitted 2374 | pass | `validation/out/replay_compare.json`, `validation/out/replay/` |
| C5 | Independent certificates (below) | pass | `validation/out/certs.json` |
| C6 | Every number in `analysis.json` and `tune.json` recomputed by the validator's own code: 187 cells, all exact | pass | `validation/out/recompute.json`, `compare_claims.json` |
| C7 | G1: 240/240 IC runs certified, ok and not exhausted; each part-B file has exactly the 60 expected keys and no duplicate rows. G2: 10/10 curves h = 1, r = order, and every run row's r equals its instance. G3: 5/5 Q runs certified and ok, ratio 0.80414 ∈ [0.6, 1.6]. Every field is indexed with `[]` | pass | `validation/out/recompute.json` |
| C8 | §5a band against the strongest reference, every unpriced counter listed | pass, with F2 and F8 | `validation/out/band.json` |
| C9 | Claims of `RESULTS.md` | pass, with F1–F9 | this file |

### C2: the prefilter

- **Mathematics.** Let y² = f(x) with deg f = 2g + 1 odd and f squarefree. Then J[2] is spanned by the classes [(eᵢ, 0) − ∞], with the single relation Σ = div(y). Taking Galois invariants gives #J(F_p)[2] = 2^(m−1), where m is the number of irreducible factors of f over F_p. Constructively: if f is reducible, it has a monic factor w with 1 ≤ deg w ≤ 4, because one of any two complementary factor degrees is at most 4. Then (w, 0) is a non-zero F_p-rational reduced divisor with 2(w, 0) = div(w) ~ 0. So 2 | #J(F_p), and a prime-order (h = 1) curve has irreducible f. A non-squarefree f is rejected by `JCurve::new` anyway. The prefilter tests the same `random_f(p, 4, cs)` that `make_instance` builds the curve from.
- **Numeric test on independent seeds.** 520 random squarefree genus-4 curves (validator RNG) at p ∈ {11, 13, 17, 19, 23, 29}. #J = L(1) comes from point counts over F_{p^k}, k = 1..4: character sums and Newton's identities, with no BSGS. Parity equals reducibility on all 520, in both directions: 468 reducible and even, 52 irreducible and odd. 2^(m−1) divides #J on all 520, and all 468 2-torsion certificates check in the validator's Cantor arithmetic. The L(1) method was cross-checked against the library's `jacobian_order` on 229 curves at p ≤ 31. Wherever the library returns an order (88 curves), it equals L(1); the rest are "ambiguous", as expected at tiny p.
- **Implementation.**
  - Rabin's test as written is correct: `x^(p^9) ≡ x (mod f)` and `gcd(x^(p^3) − x, f) = 1`, with remainders taken by `vrem` against the monic f.
  - Differential test against the validator's distinct-degree factorisation: 1,116 cases at p = 16411 and 32771, all agreeing. They are 300 irreducible f (none called reducible); 24 each of the factor patterns (3,3,3), (1,8), (2,7), (4,5), (3,6), (2,2,5), (1,1,7), (1⁹) and g²h; and 600 random f. The (3,3,3) products pass the x^(p^9) step and are caught only by the gcd step.
  - Per seed, through the validator's harness `val_po`, built after `genus_ladder` on the same library: `f_irreducible(p, random_f(p, 4, cs))` agrees with the validator on every seed in 1..2000 at p = 16411 (196 irreducible) and 1..3000 at p = 32771 (350 irreducible).
  - Per file, the binary's `f_reducible_even_order` count equals the validator's reducible count. Irreducible seeds equal admitted plus `order_not_near_prime` in all 57 complete files.
- **No wrongly skipped seed.**
  - Below 875 (p = 16411), 870 seeds were not admitted: 790 skipped as reducible, each with a checked certificate w | f, and 80 point-counted and rejected as `order_not_near_prime`.
  - Below 2374 (p = 32771): 2,369 not admitted, 2,103 of them with a certificate and 266 point-counted.
  - The only rejection reasons anywhere are those two, so no seed fell to an ambiguous or failed order computation.
  - The point-counted rejections are not prefilter decisions. 16 nonce-drawn ones (8 per prime) are certified composite independently: the library's N is composite and in the exact Hasse–Weil interval, f is irreducible, and [N]D = 0 for a non-zero divisor of the validator's own. If #J = q were prime, q | N and N < 2q would force N = q. The other 330 rest on the library's order computation, which the three replayed search cells reproduce.

### C3: selection

- **p = 16411.** Four residue files cover 1..2000. Each has `tried` 500, equal to its list length, and admitted + Σ rejects = tried. They admit 11 seeds: 373, 455, 548, 752, 875, 1036, 1041, 1317, 1331, 1438, 1718. The chosen seeds are 373, 455, 548, 752, 875. Seed 2640 lies outside 1..2000, so its exclusion is moot.
- **p = 32771.**
  - Residues 0–2 of 1..1800 are complete.
  - Residue 3: the original file has no `cell` row and is not counted as coverage. Its two admitted seeds (355, 1623) are found again by the 18 re-scan chunks. The chunks hold 113 + 113 + 112 + 112 = 450 seeds, every chunk with a `cell` row and `tried` equal to its list length.
  - Blocks 1801–2200 and 2201–2600 hold 16 chunks each (400 seeds), all complete.
  - Contiguous coverage is 1..2600, and no seed is covered twice. The admitted seeds are 355, 882, 1461, 1623, 2374, 2508, 2579, 2600. The chosen seeds are 355, 882, 1461, 1623, 2374.
- **`curves.json`.** Its `admitted`, `chosen` and `orders` equal the validator's. Each search file's seed list was reconstructed from `search.sh` or `search_chunks.sh`, because the config row does not record `--curve-seeds`. The `tried` counts confirm the reconstruction: 28, 16 and 1-seed chunks.

### C5: certificates (validator arithmetic, `jac.py` re-reviewed at `ca72267899`)

- **Curves.** For all 10 curves:
  - f re-derived from the seed equals the row and the library dump;
  - f is monic of degree 9, squarefree and irreducible;
  - r is prime (deterministic Miller–Rabin), order = r, and h = 1;
  - r lies in the exact Hasse–Weil interval, and 2·lo > hi;
  - r has 57 bits at p = 16411 and 60–61 bits at p = 32771.
- **Automorphisms.** f is irreducible, so ∞ is the only rational Weierstrass point and every F_p-automorphism is x ↦ ax + b. The x⁸ coefficient forces b, and the scan over all p − 1 values of a finds only the identity on every curve. Two planted controls are found: an odd f gives a = −1, and f = g(x³) gives the two cube roots of unity.
- **Keys.** For all 60 (curve, target) keys:
  - G is re-derived from the target seed, G ≠ 0 and [r]G = 0, using a NAF chain different from the producer's ladders. So ord G = r, r | #J, and #J = r.
  - t is re-derived and T = [t]G.
  - G, T and t equal the library dump.
- **Logarithms.** 49 recovered logarithms were checked: 48 arm-x and x\* runs in the replayed cells and the Q run (455, k = 0). In each, recovered = t and [recovered]G = T, and the dumped counts equal the committed row. A tamper control (log + 1, count + 1, one T coefficient) is caught 3/3 in each mode.

## Headline numbers (recomputed = claimed exactly)

Ratio of mean IC cost to mean reference over 60 keys (5 curves × 6 targets × 2 seeds); 95% hierarchical bootstrap, 2000 resamples, seed 20261124. Cost = group operations + w × (rel_mul + rel_add + la_mul + la_add).

| p | arm | vs √(πr/4), w = 0 | w = 0.1 | w = 1 | vs √(πr/2), w = 1 | D1 (upper < 1 at all w) |
|---|---|---|---|---|---|---|
| 16411 | x (cc 2.0) | 0.0017844 [0.0017719, 0.0017953] | 0.101472 [0.101094, 0.101845] | 0.998658 [0.994924, 1.002345] | 0.706158 [0.703517, 0.708765] | no |
| 16411 | x\* (cc 1.4) | 0.0029467 [0.0029275, 0.0029635] | 0.090244 [0.089805, 0.090635] | **0.875916 [0.871709, 0.879712]** | 0.619366 [0.616391, 0.622050] | **yes** |
| 32771 | x (cc 2.0) | 0.0011332 [0.0011286, 0.0011393] | 0.077949 [0.077692, 0.078198] | 0.769290 [0.766703, 0.771782] | 0.543970 [0.542141, 0.545732] | yes |
| 32771 | x\* (cc 1.4) | 0.0019274 [0.0019220, 0.0019327] | 0.065721 [0.065562, 0.065879] | **0.639868 [0.638294, 0.641384]** | 0.452455 [0.451342, 0.453527] | **yes** |

- **x\* / x at w = 1, paired by key.** 0.877093 [0.873004, 0.881324] at p = 16411 and 0.831764 [0.829061, 0.834628] at p = 32771 (seed 20261125, the frozen script's choice).
  - Paired per-key ratios have mean 0.8772 and 0.8318, ranges 0.850–0.910 and 0.815–0.851.
  - x\* is cheaper on 60/60 keys at both primes (lesson L-26895912385b).
- **D2 slope of ln(ratio vs √(πr/4), w = 1) in ln p.** x −0.37731 [−0.38412, −0.37059]; x\* −0.45404 [−0.46217, −0.44553].
- **Slope of ln(mean IC cost) in ln p.** At w = 1: 1.6191 (x) and 1.5424 (x\*). At w = 0: 1.3399 and 1.3826. The reference's slope is 1.9964.
- **Per curve, x\* vs √(πr/4) at w = 1.** 0.8761, 0.8754, 0.8792, 0.8703, 0.8785 at p = 16411; 0.6395, 0.6397, 0.6396, 0.6401, 0.6405 at p = 32771. All curves are h = 1, so no cofactor mix enters (lesson L-f19644df279f).
- **Mean cost at w = 1 (GAE per key).** x 2.38909e8 and 7.32056e8; x\* 2.09546e8 and 6.08898e8. Mean √(πr/4) is 2.39230e8 and 9.51599e8.
  - Factor-base columns: 2900/4872 (x) and 2030/3410 (x\*).
  - Linear-algebra share of w = 1 cost: 76.6%/80.6% (x) and 56.0%/60.4% (x\*). Of native work: 76.7%/80.8% and 56.2%/60.6%.
- **Part A** (p = 16411, curves 172 and 180, h = 2, 12 keys per configuration, all 144 runs certified and correct).
  - The frozen rule picks cc = 1.4, SGE (4, 1.5): 2.09537e8, which is 0.87985× the baseline (2.0, (4, 1.5)) at 2.38150e8.
  - Runner-up: (1.4, (8, 2)) at 1.0072× the choice. Best cc = 2.0 cell: (8, 2) at 1.1099×.
- **G3.** Measured plain rho over √(πr/2) is 0.80414 (5 runs; per run 0.481, 0.521, 0.852, 0.787, 1.383).

## Rho reference and the §5a band

**Which reference.** The reference is the ideal negation bound √(πr/4) group operations, with nothing charged. It is PROTOCOL §6's R_neg and D1's reference.
- All 10 curves have only the identity and the hyperelliptic involution as F_p-automorphisms (C5), and Frobenius is trivial on J(F_p). So negation is the strongest available symmetry, and every measured walk costs more than this floor.
- It is the expected cost of an ideal random walk on the r/2 classes. That is a heuristic floor, not a proven lower bound (F3).
- The measured plain rho of G3 (5 runs) sits at 1.137× this floor. The replayed Q key is identical and its log is certified.

**Band, mean GAE per key, against √(πr/4)** (`validation/out/band.json`):

| p | arm | w = 0 | w = 0.1 | w = 1 | reference | order across band |
|---|---|---|---|---|---|---|
| 16411 | x | 4.2689e5 (0.00178) | 2.4275e7 (0.1015) | 2.3891e8 (0.9987; upper 1.0023) | 2.3923e8 | not ranked (D1) |
| 16411 | x\* | 7.0494e5 (0.00295) | 2.1589e7 (0.0902) | 2.0955e8 (0.8759) | 2.3923e8 | **below at all three** |
| 32771 | x | 1.0784e6 (0.00113) | 7.4176e7 (0.0779) | 7.3206e8 (0.7693) | 9.5160e8 | below at all three |
| 32771 | x\* | 1.8341e6 (0.00193) | 6.2540e7 (0.0657) | 6.0890e8 (0.6399) | 9.5160e8 | **below at all three** |

**Unpriced counters, every arm.**
- **IC, priced at w:** `rel_mul`, `rel_add`, `la_mul`, `la_add`. The setup work is counted: the character table, the factor-base evaluation and the Ferrari constants (`genus_ladder.rs` ~1500–1556).
- **IC, not emitted:** one `mix64` partition hash and two `addm` mod r per walk step (`genus_ladder.rs` 1590–1594), and the large-prime table operations per relation (F2).
- **Reference:** none (a closed form).
- **Measured Q:** `hash_uncharged` and `coef_adds_uncharged` as listed; `compares_uncharged` = 0 on all 5 runs.

`VALIDATION.json` carries the x\* band at p = 16411, the binding case of the "both primes" claim; its note gives the other three. These are `genus_ladder` rows, not ecbench sessions, so `ecbench band` does not apply. The band was recomputed with its rule, every listed operation counter priced at w (lesson L-546d37ca7750).

## The claims of RESULTS.md

1. **"Below the ideal negation-rho bound at all w": correct, with scope.**
   - D1 is met for x\* at both primes and for x at p = 32771. For x at p = 16411 it is not met (upper end 1.0023). RESULTS says all of this.
   - IC cost is linear in w and the reference is constant, so "below at w = 0 and w = 1" means below for every w ∈ [0, 1]. In fact it holds up to the break-even price: 1.142 (x\*, 16411), 1.565 (x\*, 32771) and 1.300 (x, 32771) GAE per native operation.
   - The scope is set out in F3.
2. **"Known in theory, now measured": right in substance, too broad in one sentence (F4).**
   - The cited theory is correct: Gaudry 2000; Gaudry–Thomé–Thériault–Diem 2007, Õ(q^(2−2/g)) for double large primes, so q^1.5 at g = 4 against rho's Õ(q^(g/2)) = q².
   - The novelty sentence "What is new here…" was not checked against the knowledge base, and the validator could not check it either.
   - What this study measured that the earlier one did not is the w = 1 end on prime-order curves. At w = 0 and 0.1, arm x was already below the floor at p = 16411 in `genus4_followup_20261118` (validated).
3. **Pricing mod-r linear-algebra operations like F_p operations: not material to the verdict.**
   - One counted Jacobian group operation here costs about 394 F_p multiplications and 7 inversions (`cantor_mul` and `cantor_inv` per group operation).
   - A mulmod modulo a 57–61-bit r is far below 1 GAE, so w = 1 overcharges IC either way.
   - With relation-phase F_p work held at 1 GAE, x\* reaches the floor only if a mod-r operation costs more than 1.253 GAE (p = 16411) or 1.931 GAE (p = 32771). With F_p work at 0, the thresholds are 2.033 and 2.581.
   - The convention affects how the w = 1 number reads, not the order. No calibrated w_cal was measured (F8).
4. **Tuning.** The P4 numbers recompute. cc\* = 1.4 sits at the grid edge (F6).
5. **Other prose.** These recompute or check:
   - #J ≈ 2^56 and 2^60;
   - "per-curve ratios within 0.01";
   - "IC cost grows as p^1.62 / p^1.54", recomputed but a two-point slope (F4);
   - 240 runs certified;
   - G3 0.80 and the earlier 1.004× over 108 runs (validated 1.00365);
   - "LA is 76% of arm x's native work at p = 16411" (PROTOCOL §1);
   - the h = 1 curve 2640 at 1.000 (the earlier validator's F7);
   - the D1/D2 deviation narrative, with F1;
   - the prediction table P1–P5.

## Findings

- **F1 (minor, deviation record).** The DEVIATIONS.md header says each deviation was logged before the data it concerns was read. That holds for D1: `d09efa5d95` and the fixed `search_chunks.sh` (`3a54c389f9`) precede the re-scan data (`b4e483061d`). It does not hold for D2, which is stamped 20:50 UTC and committed in `8c54acb9e2`:
  - D2 lists the seeds found in 2201–2600 and was committed with that block, after the 1801–2200 block (`d202be88ca`).
  - What D2 records is the frozen extension rule (PROTOCOL §2), announced in the commit messages of `b4e483061d` and `d202be88ca` before each block's data.
  - Its one genuine deviation, stopping 2601–3000, cannot change the five smallest seeds, because 1..2600 is fully covered.
  - Git cannot show whether the partial files were "deleted unread".
  - Effect on any result: none.
- **F2 (minor, pricing symmetry).** "Fully charged" and "every native operation is charged at up to 1 GAE" are slightly overstated.
  - The IC walk's per-step hash and two coefficient additions mod r, and the large-prime table operations, are not emitted.
  - Pricing 3 operations per walk step at w moves x\* at w = 1 from 0.8759 to 0.8848 (p = 16411) and from 0.6399 to 0.6457 (p = 32771). It moves x at p = 16411 from 0.9987 to 1.0040.
  - No D1 decision changes. The reference charges rho nothing for the same per-step work, which favours rho.
- **F3 (interpretation, scope of the headline).** The headline holds under these conditions:
  - 5 prime-order genus-4 Jacobians per prime, the five smallest admitted seeds, with #J ≈ 2^56 and 2^60; 6 targets × 2 seeds each. The intervals resample keys of these curves; per-curve ratios agree within 0.009.
  - The unit is counted group operations plus w per native operation. There is no wall-clock statement, and the host was not isolated (disclosed).
  - The reference is a closed form, the ideal-walk expectation, not a measured negation walk. RESULTS calls it "a lower bound on any rho with the negation map", which is the random-mapping heuristic, not a theorem.
  - x\* was tuned on two h = 2 curves at p = 16411 only and applied unchanged at 32771.
  - It is a genus-4 hyperelliptic result, not an ECDLP result, as RESULTS says.
- **F4 (interpretation, framing).**
  - "What is new here is this lab's implementation crossing…" is a novelty statement. Under the knowledge-base rule it needs a check, and research-kb was unavailable to producer and validator. Scope it to this lab, or label it unchecked.
  - "IC cost grows as p^1.62 (x) and p^1.54 (x\*)" are local slopes between p and 2p, not exponent fits; AGENTS.md §5 asks for four sizes. At w = 0 the slopes are 1.340 and 1.383. The match with p^1.5 is suggestive only.
- **F5 (minor, prose).** "Linear algebra from 77–81% of native work to 56–60%": the 56–60% figures are linear algebra over total w = 1 cost. Over native work they are 56.2% and 60.6% (76.7% and 80.8% for x).
- **F6 (note, tuning).**
  - cc\* = 1.4 is the lower edge of the grid {1.4, 2.0, 2.8}, so the optimum is not located.
  - The cc choice is robust: the best cc = 2.0 cell costs 1.110× the choice. The SGE choice (4, 1.5) over (8, 2) is a 0.72% margin on 12 keys.
  - "The optimum falls with p" is consistent with p = 8209 picking the upper edge 2.0, but neither optimum is located.
- **F7 (note, bootstrap).**
  - `analyze.py` reseeds the same generator at both primes, and both primes share one design. Replicate j therefore draws the same index pattern at both, so the D2 interval is not built from independent per-prime bootstraps, as PROTOCOL §6 says it is.
  - With an independent stream at p = 32771, the intervals are x [−0.38473, −0.36987] and x\* [−0.46120, −0.44625].
  - The protocol fixes no draw order. Drawing each curve's targets and seeds as soon as the curve is drawn moves ends by at most about 2e-4: x at p = 16411, w = 1 becomes [0.994764, 1.002512]. No decision changes.
- **F8 (note, §5a calibrated weight).** No w_cal was measured for the mod-p and mod-r counters (owner rule 2026-11-12). Because the order holds up to the break-even price, it holds at w_cal and 10·w_cal for any w_cal below 0.114 GAE per operation. "Ranked at calibrated price" is therefore an inference, not a measurement.
- **F9 (note, records).**
  - Bench record `20261118T1652Z-genus4-prime-order` is still `open` (`closed_utc` null). Its raw entries carry the right sha256 but `uploaded: false` (no S3, disclosed), and the 58 search files are not listed in it. Its 5 metrics match the recompute.
  - There is no measured rho at p = 32771 (disclosed).

## Reproducibility

- **Host.** 4-core Intel Xeon, Linux 6.18.44.
- **Toolchain.** rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0.
- **Build.**
  - Command: `cargo build --release --locked --offline --bin genus_ladder` in `…/scratchpad/val4po/src_ca72267899`, with sources touched first. The log shows `Compiling crypto v0.1.0 (<that tree>)`; the build took 13 m 12 s with no other `crypto_lib` compile running.
  - The root disk had 1.8 GB free, so the validator-owned `CARGO_TARGET_DIR` was a tmpfs (`/dev/shm/val4po_target`) reached through a symlink.
- **Harness.** `val_po` (`validation/scripts/val_po.rs`, sha256 `81bbb745…`) was built afterwards in 1.4 s, with the library reused.
- **Replays.** `validation/scripts/jobs.py` and `run_jobs.sh` replay one nonce-drawn cell per invocation, with the flags of `run.sh`, `search.sh` and `search_chunks.sh`. Job log: `validation/out/replay/jobs.log`, 42/42 exit 0.
- **Not committed.** `out/replay/h_irr.jsonl` is 521 KB (a data file over 64 KiB). Its per-seed flags are in `out/prefilter_rust_flags.json`, together with its sha256.
- **Not done.**
  - No replay with the producer's binary: its code sections equal the rebuild's.
  - S3 copies not checked: none were uploaded.
  - Not run on isolab-eks: counts are deterministic, and the local host is a disclosed deviation.
  - The 330 point-counted rejections outside the drawn 16 were not recounted independently.
  - No knowledge-base search for the novelty statement (research-kb not available to this subagent).
- **Lessons applied.** L-546d37ca7750, L-99ee952350b2, L-3f39e1d8b3a9, L-7f8877029be4, L-ce5e4e4b764b, L-ca1840e60dc0, L-b0626ec2a4c0, L-26895912385b, L-10ef4b64c2a6, L-f19644df279f, L-c53b34f2a6ce, L-1a5a6b64022c, L-8d86e8fc352c (corrections belong in an `ERRATA.md`, not in the pinned files).
