# Results: chasing the five top-tier leads (2026-11-12)

Protocol: `PROTOCOL.md` (frozen at commit dccf37b6, amended at e50d4f23 before the S1 cells ran). All runs local
(4 vCPU), raw records in this directory with the engines' commit hashes; every number `[pcc:unverified]` (one
engine, one machine, no independent replay) unless a line says otherwise. Engines: ml-cryptanalysis at 27a67b9
(`scripts/ic_bounds_growth_lib.py`, `csd/ic_bounds_big.py`), N4v2 `v2.rs` built here (`j2/BUILD.txt`).

**Summary.** [pcc:unverified]
- **J2 (percolation factor-base size): measured improvement, held out.** On the N4v2 plane-quartic DL1 solver (weak
  GHS family), lowering the factor-base constant from the frozen 2.00 to 1.60 cuts the charged cost to 0.655–0.685×
  at k = 21 and 22 on fresh seeds, with the order holding across the w ∈ {0, 0.1, 1} band; the success threshold sits
  between 1.50 and 1.60 at every k. Falsified at k = 16 (no gain there). Engineering class, unvalidated.
- **S1 (mechanism of the prime degree law): Dm survives.** A second equation lowers the fall degree at k = 10;
  characteristic down to 19 with one equation does not. The record's small-characteristic cells were void (Fermat
  moduli admit no unit instance) and were rescoped before data. Side finding: about 1 in 10 small-characteristic
  unit instances has a non-generic closure (exceptional loci), one class extending the D = 4 plateau by one depth.
- **P1 (quotient law): reproduces exactly out of sample** (72/72 cells), which confirms instance-independence; the
  decisive test is the GPU-class stage 2.
- **M1 (m = 3 arity): closes** at the measured depths from existing exact records.
- **J1 (quartic solver on generic Jacobians): blocked** on the instance side (no genus-3 point counter; ℓ < 2^64),
  with the solver adapter sized at about 140 lines.

## P1 quotient law: stage 0 reproduced, stage 1 (fresh primes) — see table below

Stage 0 (`tools/p1_stage0.txt`): the twelve committed cells give q/2^k = 2.500, 4.375, 5.250, 5.156 (D = 3);
2.953, 5.016, 5.688, 5.365 (D = 4); 3.058, 5.083, 5.712, 5.374 (D = 5); 3.067 (D = 6): lane P's table to the digit.
Side finding: the N8 main instance (12289, 4021, 9917, 4408) is unit at k ≤ 6 and proper at k = 8, 9 (2, 16 zeros).

Stage 1 (`p1/`, `tools/p1_score.py`, figure `figures/p1_quotient_law.svg`): primes 13313, 15361, 19457; per prime two unit
instances and one proper control; D = 3 at k = 2..9 and D = 4 at k = 4..9 (18 closures, growth-lib engine, full sweep clean
on every closure). **All 72 plateau cells equal the committed q/2^k exactly** (D = 3: 2.500, 4.375, 5.250, 5.156; D = 4:
2.953, 5.016, 5.688, 5.365), on unit and proper instances alike; 1 ∈ V_4(k) iff k ≤ 5 on every unit instance. The
prediction (within 2%) holds with zero deviation.

What this does and does not show. The closures at these sizes are instance-independent (the profile is the same for
every prime, curve and target tested), so an out-of-sample prime reproduces the committed value to the digit: stage 1
confirms that independence and the [2.5, 6.0] band at j ≤ 3, not the mechanism behind the quotient size. Outside the
plateau the ratio keeps falling (D = 3: 4.47, 3.56, 2.66, 1.89 at j = 4..7), so the band is a statement about
j ≤ 3 only. The informative next cell is stage 2, the direct D = 6 closure at k = 15 with the derived prediction
q = 167,665 (GPU-class, about 6 H200-hours; owner's call on the rented-GPU route). The corollary "any closure-type
solver pays at least about 3B per target" stays a heuristic about that class of solver.

## S1 crossing: the record's characteristic cells are void; rescoped cells — see below

**Structural result (exact, `tools/unit_test.py`).** Over F_l the chain ideal at depth k is proper whenever the
Semaev curve has an F_{l^m}-point with both coordinates in μ_{2^{v_2(l^m − 1)}} ⊂ μ_N; for the Fermat primes
F_l^× itself lies in μ_N (17 at k ≥ 4, 257 at k ≥ 8) and for 97 a third of F_97^× does, so unit instances do not
exist in practice: 0 of 60 seeded draws at k = 6 and 10 for each of l = 17, 97, 257. The record's cells C-c17,
C-c97, C-c257 (which needed 1 ∈ V_D) cannot be run. Primes with v_2(l − 1) = 1 admit unit instances freely
(19: 50/60, 23: 40/60, 43: 50/60, 107: 57/60, 227: 59/60), so the characteristic arm was rescoped to
l ∈ {19, 43, 107, 227} with one generator (the Frobenius images σ_l(g) of these l have weight ≥ 6, so the record's
"F-links" mechanism has no admissible element at D ≤ 5). This is also a statement about the binary analogy the
record draws: the l = 2 case is the extreme where F_2^× = {1} ⊂ μ_N trivially, and binary systems are never
single-equation chains of this form.

**Rescoped arm (b′), scored (`s1/bigsmall/`, `s1/big/`; scorer `tools/s1_score.py`).** One generator, four unit
instances per l ∈ {19, 43, 107, 227}, k = 4..9 at D ≤ 5 and k = 10 at D = 5, on the big engine (CPU, `--nosplit`;
deviation from the protocol's engine split, recorded: the growth-lib driver ran 20× slower on the oversubscribed host
and was stopped after its control rows; the two engines agree on every cell cross-checked). Control arm (a) over
12289 reproduces the committed V_5(10) profile (0, 0, 2, 138, 3163, 18569) exactly.

| l | k = 4..9 (D ≤ 5), first D with 1 ∈ V_D | k = 10 (D = 5) | reading |
|---|---|---|---|
| 19 | law at every k on 4/4 instances, except one instance at k = 6 (enters at D = 4) | 1 ∉ V_5 on 2/2, dims generic | law |
| 43 | law at every k on 4/4 | 1 ∉ V_5 on 2/2, dims generic | law |
| 107 | law at every k on 4/4 | 1 ∉ V_5 on 2/2, dims generic | law |
| 227 | law at every k on 4/4 | 1 ∉ V_5 on 2/2; dim V_5(10) = 18,567 on both (generic 18,569) | law (dims depart) |

288 small cells and 8 k = 10 cells, no engine failure, no disagreement between the two engines on the cells run on
both. With one equation, characteristic l from 19 to 227 (all below 2^(k+1) = 2048 at k = 10) leaves the law's step
positions unchanged on every scored cell.

**Exploratory follow-up (post hoc; `tools/s1_order_probe.py`, `tools/s1_freq_probe.py`, figure
`figures/s1_departures.svg`).** One F_19 instance in arm (b′), (a, b, t) = (18, 18, 2), has 1 ∈ V_4(6): the D = 4
plateau lasts one depth longer than the law; both engines reproduce it, and it is back on the law at k = 7..10.
- The same (a, b, t) = (−1, −1, 2) over 40961, 43, 107 and 227 gives the generic closure: not the curve.
- Over 40961, targets of exact order 3, 4, 5, 6, 8, 9, 12 give the generic closure (14 instances); order-2 targets
  depart slightly (dims 607 and 835 at k = 6, 7; the known degenerate case S_3 = c·h²): not small target order.
- Frequency among unit instances with non-order-2 targets: departure from the generic dims (605, 829) at k = 6 or 7
  in 5/30 (l = 19), 2/30 (23), 3/30 (43), 0/20 (107), 2/20 (227), and 0/14 at 40961. Three classes: smaller closures
  (602, 826) and (603, 828), with 1 still excluded, and full closures (794, 857), with 1 ∈ V_4(6) (two at l = 19, one
  at l = 227, which is above 2^(k+1) = 128).
- Reading: departures occur at every small l including l > 2^(k+1), at rates of order a few per hundred, and not at
  large l: consistent with exceptional codimension-one loci in (a, b, t) hit with probability of order 1/l, not with
  a characteristic mechanism. The loci themselves are not identified. For the attack side: such loci cannot be
  steered to with a shared factor base (lane P's "target-adapted translation" rejection applies).

**Arm (d), scored (Dm, two equations over 12289, k = 10, D = 5).** Both instances, both second inputs: **1 ∈ V_5(10)**, the closure fills all 21,700 columns, one
degree below the law (LFD 6 at k = 10). Negative control d0 ({g, x_1·g}, a second input already in the closure):
rank 18,569 and 1 ∉ V_5(10) on both instances, identical to one generator, so the multi-input engine path does not fill
the closure by itself.

**Scoring against the four hypotheses** (C-c cells void, replaced by arm (b′) as amended before data):

| hypothesis | arm (b′) one equation, small l | C-d1 {g, h} | C-d2 {g, σ_257 g} | verdict |
|---|---|---|---|---|
| L (one equation and large characteristic) | predicted a departure; observed law | predicted below law: yes | predicted below law: yes | characteristic half not supported at k ≤ 10 |
| Dm (number of equations) | predicted law: yes | predicted below law: yes | predicted below law: yes | **survives** |
| F-links (long-range low-weight elements) | no admissible element at D ≤ 5 | predicted law: **no** | predicted below law: yes | falsified by C-d1 |
| S (characteristic- and equation-free) | predicted law: yes | predicted law: **no** | predicted law: **no** | falsified |

**Reading.** At depths up to 10, the measured law ⌊(k+14)/4⌋ is a property of the one-equation system, not of large
characteristic: a second independent equation lowers the fall degree by at least one at k = 10, characteristic as
small as 19 does not move it. This is the record's Dm explanation of why binary descents (n equations) follow a
different, slower law. The written growth theorem's characteristic hypothesis (p > 2^(k+1)) is therefore a proof
device at these depths; whether the characteristic matters at k ≥ 39, where that theorem applies, is untouched.
Scope: two instances per k = 10 cell, one engine for the k = 10 cells, unit instances only.

## J2 percolation factor-base size — see table below

Grid (`j2/units/`, frozen seeds 1–3; binary SHA-256 in `j2/BUILD.txt`; scorer `tools/j2_score.py`, band
`tools/j2_band.py`; figure `figures/j2_threshold_cost.svg`; bench record
`benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json`). DL1 on N4v2's own weak-family instances, every
attempt charged, M-eq priced with the median of three `v2 calib` records per size (PARAMS.md rule).

| k (q) | first-attempt success at c = 1.40 / 1.50 / 1.60 / 1.75 / 2.00 | charged DL1 cost / c = 2.00 at c = 1.60 (w = 0) | at c = 1.75 (w = 0) |
|---|---|---|---|
| 16 (65,479) | 0/3, 0/3, 3/3, 3/3, 3/3 | 0.87–1.12 | 0.91–1.00 |
| 18 (262,111) | 0/3, 0/3, 3/3, 3/3, 3/3 | 0.72–0.80 | 0.79–0.85 |
| 20 (1,048,507) | 0/3, 0/3, 3/3, 3/3, 3/3 | 0.67–0.69 | 0.78 |
| 22 (4,194,271) | (1.40, 1.50 not in the grid), 3/3, 3/3, 3/3 | 0.65–0.66 | 0.77 |

Against the preregistered predictions:
- **Threshold law: holds.** First-attempt success flips between c = 1.50 and 1.60 at every k, inside the predicted
  c_50 ∈ [1.40, 1.75] (model 1.49–1.57).
- **Success at c = 1.75 ≥ 0.9: holds** (3/3 at every k; three seeds is a weak test).
- **Cost ratio at c = 1.75 in [0.70, 0.85] at every k: falsified at k = 16** (0.91–1.00), holds at k = 18, 20, 22
  (0.77–0.85). The record named this risk ("a large-q lever"); the ratio falls monotonically with k.
- **Band (§5a).** At k ≥ 18 the per-seed order c = 1.60 < 1.75 < 2.00 holds at w = 0, 0.1 and 1 (unpriced hash and
  compare operations priced at w GAE); at k = 16 it fails at w = 0 and 0.1.
- **Controls.** A/A repeat of (k = 16, seed 1, c = 1.75): every counter identical. Seeds agree to within 1% at
  k ≥ 20.
- **Post-hoc arm.** c = 1.60 is cheaper than the preregistered c = 1.75 at k ≥ 18 (0.65–0.80). It was selected after
  reading the grid, so it was frozen as a held-out test (amendment 05:00 UTC; seeds 4–8, k = 21 and 22):
  **confirmed.** Seeds 4–8 at k = 21 (q = 2,097,091, not in the grid) and k = 22: first-attempt success 10/10 at
  c = 1.60 and 10/10 at 1.75; paired ratio to c = 2.00 at w = 0 of 0.655–0.685 at c = 1.60 (predicted [0.60, 0.72]) and
  0.771–0.775 at c = 1.75 (predicted [0.72, 0.82]); per-seed order 1.60 < 1.75 < 2.00 at w = 0, 0.1 and 1; every unit
  recovers the planted log. (`j2/heldout/`; 26 units first ran after the artifact-store bot removed the binary from
  the worktree and produced empty output: kept in `j2/heldout/failed-binary-missing/` and rerun with the
  hash-identical build; the first loop's log printed `rc=0` for them because `$?` captured `date`, fixed in
  `tools/j2_heldout.sh`.)

Class: engineering (one parameter of the frozen pipeline; same algorithm), on the programme's weak-family quartic
row (N4v2 DL1, itself about 0.005× matched negation rho at q = 2^21 per its README). Not validated: no replay
certificate or validator report, so no leaderboard or `benchmarks/README.md` row; the next step is the validation
gate on the held-out rule.

## M1 three-summand arity grid (desk, from `ml-cryptanalysis/experiments/ic-bounds-m3-depth/`, exact records)

For m = 3 (variables 3k, generator NF(S_4) of degree 6): LFD = 7 at every k ≤ 5 (V_D = 0 for D ≤ 5 and V_6 = span g,
so LFD ≥ 7 is a proof for k ≥ 3; exact LFD 7 on 12 instances at k = 5, and V_7 = I ∩ M_≤7 at k = 2..5), and
LFD ≥ 8 at k = 6 on the one instance run (V_7 closed at dim 19,958 against 62,956 for the ideal part). Against the
§1 bar of `research/leads_20261112/README.md` (m = 3: LFD/k < 0.522 for the columns alone, < 0.184 with sparse
linear algebra): LFD/k = 1.4 at k = 5 and ≥ 1.33 at k = 6; columns at the LFD against the enumeration 2^{2k}:
16,384 vs 1,024 at k = 5 (16×) and ≥ 106,762 vs 4,096 at k = 6 (26×). The m = 3 Gröbner route is above
enumeration at every measured depth, with the gap widening. The only measurement that would add information is
D = 8 at k = 6 (106,762 columns) and k = 7 (401,930): GPU-class, not run. **M1 closes at the measured depths.**

## J1 quartic solver on cofactor-one Jacobians: chased to its obstruction (no run)

`J1_adapter_report.md` (code reading of N4v2): the D family reads only q, ℓ and the 15 quartic coefficients plus
the GHS curve E0 for the target, the three descent start classes and the final check; nothing in relations,
descent, linear algebra or verification uses 4 | #E0. A generic adapter (`v2 icx <inst.json>`) is about 140 lines
(a `Src` abstraction for start classes and the check, a forced base point, Tonelli–Shanks for q ≡ 1 mod 4, which
the GHS choice q ≡ 7 mod 12 hid). The obstruction is the instance side: the lab has no point counter for a
generic genus-3 curve (brute force is O(q^{2k}), unusable beyond q ≈ 20; no Sage/PARI/Magma on the host), GHS never
counts #J (it counts #E0 by BSGS and trusts the cover), and ℓ < 2^64 with #J ≈ q³ caps prime-order instances at
q ≈ 2^21. So the preregistered k = 12..18 grid needs a genus-3 point-counting instrument first (weeks), or a
family with #J known by construction, which reintroduces structure. Recorded as blocked, with the adapter sized.
