# L1 PROTOCOL: keyed-hash factor-base membership (wave 7, 2026-11-22)

Status: DESIGN, frozen before any data. Phase 1 only: the smoke runs under `smoke/` are labelled
"smoke, not data". Every result sentence carries [pcc:unverified]. Frozen files: `FREEZE.sha256`.

## 1. Question

Does a factor base defined by a keyed hash of the x-coordinate (H_key(u(P)) < theta, SHA-256 with a
fixed published key) give the same relation rate as the interval ("bits") base, a uniformly random base
and the projected base of the same size, in the C = 2 table-and-probe search for a random subgroup
target R = ±P_i ± P_j? Null expected; the experiment is powered to see a 3 % departure.

## 2. Mechanism (stated before data)

- A hash base has no polynomial membership, so the algebraic F4/Semaev decomposition cannot use it
  (its columns need an algebraic description such as "x in an interval"). It can enter only generic
  search.
- In generic search the frame-search ceiling applies (curve25519_frame_search_20261103 README point 2):
  for N columns, P(R = ±P_i ± P_j) <= (#signed pair sums)/(r-1) = 2N^2/(r-1) for points. The ceiling is
  reached only by the projected base ([8]P_i in E[r]); an unprojected base lands in E[r] for about 1/8 of
  its pairs (cofactor cosets cancel), so its rate is about N^2/(4r).
- The hash is independent of the group law, so membership gives no coset bias: predicted hash/random = 1.
  lead_sweep_20261110/LEAD-EXO-coordinate-revealing-oracles found coordinate/random = 0.998 at 2^20 but with
  per-size ratios 0.97 to 1.75 and 9 to 101 relations, i.e. not powered for 3 %; this protocol is.
- Cost is the only thing the hash changes: the set {u : H(u) < theta} can only be enumerated by scanning
  the field (p hash evaluations, p/N per column), against about 2 Legendre/sqrt per column for bits and
  random bases. Priced in section 9.

## 3. Instrument (exact commands, one core)

```
cd research/curve25519_wave7_untried_20261122/L1_hash_base
python3 l1lib.py                                   # curve facts (curve_facts.json)
python3 run_l1.py --out out                        # full run, all 5 curves x 2 sizes
python3 analyse_l1.py --in out --out analysis      # RESULTS.md, summary.json
smoke/run_smoke.sh                                 # smoke, not data (k = 16, N = 18, B = 40, ~1 s)
```

`l1lib.py`: Montgomery y^2 = x^3 + A x^2 + x over F_p, E(F_p) cyclic of order 8r. A full-group log table
(one pass over <g>, g of exact order 8r, checked) serves as accelerator and to draw targets
R = [8j]g, j uniform in [1, r-1] (uniform on E[r] minus O). It is not trusted: every counted hit is
re-verified by real affine point arithmetic (`verify_sum`: u(P_i ± P_j) = u(R) for the table entry that
produced the hit; a T0-keyed hit is verified by real addition of T0 = (0,0) to R). Once per cell
`instrument_check` compares 2000 real pair sums, 300 real [8]P and T0 shifts and [r][8]P = O against the
log model; any mismatch fails P7. The table for an arm is the set of u(±P_i ± P_j) over all
signed pairs i <= j (i = j gives 2P_i; P_i - P_i = O is dropped), exactly the C = 2 pair table; a probe
is one lookup of u(R).

Curves (facts verified by computation in `curve_facts.json`: #E by Legendre count, r and twist r' prime,
r > 2^(k-3), p = 5 mod 8, A^2 - 4 non-square, E(F_p) cyclic Z/8r):

| k | p | A | #E | r |
|--:|--:|--:|--:|--:|
| 16 | 65437 | 1058 | 8·8209 | 8209 |
| 18 | 262133 | 78 | 8·32887 | 32887 |
| 20 | 1048573 | 926 | 8·131203 | 131203 |
| 22 | 4194301 | 43186 | 8·524731 | 524731 |
| 24 | 16777141 | 7842 | 8·2098009 | 2098009 |

k = 16..22 are the family-rule members (frame_search README, erratum E1: A = 78, 43186; the engine's
`family` output agrees). k = 24 is the brief's faithful member p = 2^24 - 75, A = 7842 (twist 4·4192553, r'
prime), not the smooth-family k = 24 member (p = 16777213, A = 318) used by the frame-search study.

## 4. Arms (all with exactly N distinct ±-folded columns; points of E[8] excluded from every base)

| arm | definition | bases per cell |
|:--|:--|--:|
| `bits_rand` | first N valid Weierstrass x >= L (wrapping), L uniform: interval base at a random offset | B |
| `bits_lo0` | same with L = 0 (the lab's base), one draw | 1 |
| `rand` | N points uniform without replacement | B |
| `rand2` | same, independent seeds (A/A for `rand`) | B |
| `hash` | SHA-256(key1 ‖ u as 4 bytes LE), first 8 bytes big-endian = H(u); base b = rank window [bN, (b+1)N) of the valid u sorted by H, i.e. membership H(u) in [theta_b, theta_{b+1}) with theta chosen so that |V| = N exactly; window b = 0 is the literal "H(u) < theta" base | B |
| `hash2` | same with key2 (A/A for `hash`) | B |
| `rand_proj` | N distinct [8]P_i of random P_i (the projected base) | B |
| `hash_proj` | [8]P_i for P_i in the hash window b, N distinct images | B |
| `plant3`, `plant5` | `rand` with N' = floor(fN) + Bernoulli(frac), f = 1.0149, 1.0247 (planted yield ratio about 1.030, 1.050) | B |
| `T0` | table keyed by kappa = u + 1/u on the `rand` bases (paired; hit if R or R + T0 is a pair sum) | same bases as `rand` |

Key bytes are fixed in `l1lib.py` (HASH_KEYS). Windows of one key stand in for independent keys under
the random-oracle model; the model is tested by the A/A arm `hash/hash2` and by P2b (literal base inside
the central 98 % of the window distribution).

## 5. Cells, seeds, counted unit, estimator

- Cells: the five curves above, two sizes each, N = round(c·sqrt(r)), c = 0.2, 0.4 (nominal unprojected
  yield c^2/4 = 1 %, 4 %; N = 18, 36, 36, 73, 72, 145, 145, 290, 290, 579). B = 512 bases per arm
  and cell (section 7).
- Targets: m_u = ceil(200·4r/N^2) per base for unprojected arms (about 200 expected hits per base,
  102,400 per arm and cell), m_p = ceil(200(r-1)/(2N^2)) for projected arms. Targets for base index b are
  the prefix of one stream shared by all arms.
- Seeds: seed(parts) = first 8 bytes (big-endian) of SHA-256("L1|v1|" + "|".join(parts)); parts name
  the role, k, N, arm and base index b (`l1lib.seed`; roles `targets`, `bitsL`, `rand`, `rand2`, `randp`,
  `plant3`, `plant5`, `check`, `boot`). `--B 1024` reproduces the first 512 bases exactly.
- Counted unit for yield: verified hits per probed target. Group-operation unit for cost: GAE (one group
  addition); a hash evaluation is not a group operation (section 9).
- Estimator: rate = sum(hits)/sum(targets) per arm and cell; ratio of rates; 95 % percentile interval from
  4000 bootstrap resamples of the bases (independent per arm, paired for T0). Pooled value = geometric
  mean over the 10 cells with the cell bootstrap draws averaged in log space. The exact mean covered
  points per base (log model, no target noise) is reported beside it as a cross-check.

## 6. Predictions (numeric thresholds, written before data; a failed prediction stays failed)

- **P1** (primary null). hash/rand pooled MC ratio in [0.98, 1.02] with its 95 % CI inside [0.97, 1.03],
  and at least 8 of 10 cell CIs contain 1.
- **P1p** (power check). hash/rand CI half-width <= 0.03 in at least 8 of 10 cells. If it fails the cell set
  is underpowered and no null is claimed; rerun the failing cells with `--B 1024`.
- **P2**. hash/bits_rand and bits_rand/rand pooled in [0.98, 1.02] with CI inside [0.97, 1.03].
  **P2b**: `bits_lo0` and the literal-theta hash base (hash window 0) each lie inside the 1st to 99th
  percentile of their arm's exact-coverage distribution in at least 8 of 10 cells.
- **P3** (ceiling, projected base). No unprojected base exceeds 2N^2/(r-1) (deterministic); projected arms
  lie in [0.78, 1.0] of it (collision loss predicted 0.96 at c = 0.2, 0.85 at c = 0.4); rand_proj/rand in
  [6.0, 8.4] in every cell (predicted 7.7, 6.9); hash_proj/rand_proj pooled in [0.98, 1.02], CI inside
  [0.97, 1.03].
- **P4** (positive control, T0). T0/blind exact coverage ratio in [1.80, 2.20] in all 10 cells (the
  T0 study measured 1.944 to 2.016) and the MC CI excludes 1 in all 10.
- **P5** (planted effect, shows the detector sees 3 to 5 %). plant5/rand CI excludes 1 in at least 9 of
  10 cells and the pooled ratio is within 0.015 of the realized design ratio mean(N'^2)/N^2; plant3/rand
  pooled CI excludes 1, pooled ratio within 0.01 of its design ratio, and at least 6 of 10 cell CIs exclude 1.
- **P6** (A/A noise floor). rand/rand2 and hash/hash2 pooled in [0.985, 1.015] with CI inside [0.97, 1.03],
  and at least 8 of 10 cell CIs contain 1 for each.
- **P7** (instrument). 0 verification failures, every counted hit verified, 0 instrument-check failures,
  MC hit total vs exact expected |z| <= 4 in every arm and cell (T0 included).
- **P8** (cost). The priced total of the hash arm is >= that of the random arm in every cell at
  w = 0, 0.1, 1 GAE per hash evaluation (equal at 0, strictly larger above).

## 7. Power (3 % departure) and the frozen constant

Smoke, not data (k = 16, B = 300): CV of covered points across bases times sqrt(N) = 0.41 to 0.68 (N = 18,
36), so CV(N) <= CV_A/sqrt(N) with CV_A = 0.7 frozen. Per-arm relative error of the pooled rate =
sqrt(CV^2/B + 1/(B·200)); the ratio of two arms has sd sqrt(2) times that. For B = 512 and the smallest
cell (N = 18, CV = 0.15): sd 1.0 %, 95 % half-width 2.0 %; larger N is tighter. Formula for B in
`run_l1.py`: B = max(512, ceil(3·(1.96·CV_A/sqrt(N)/0.03)^2)) = 512 for every cell. The pooled interval
over 10 cells has half-width about 0.7 %. A planted 3 % effect is about 3 sd per cell and 6 sd pooled.
Smoke result at B = 300 on the two k = 16 cells: pooled plant3/rand 1.034, plant5/rand 1.044 (smoke).

## 8. Controls

- Positive: T0 key (about 2x, `T0/blind`) and the two planted sizes (3 %, 5 %) through the same estimator.
  Also the projected base as the 8x ceiling arm.
- Random/negative: `rand` is the reference population; `rand2` and `hash2` are A/A arms.
- Same instrument, same targets, interleaved by base index b for every arm; arm labels are not
  masked in analysis (the arms are defined by construction, not by judgement).

## 9. Membership-test cost, priced separately (analysis plan)

Group operations (GAE) per cell and arm: table = N^2 additions (N(N-1) pair sums + N doublings); projected
arms add 3N doublings; a probe is a lookup (0 group ops, 0 hash evaluations in table mode). Hash base:
H_build = p hash evaluations to enumerate the set (p/N per column, 3,600 to 29,000 here), or p/2 hashes plus p
Legendre symbols if non-residues are skipped first; the alternative "membership-test every probe candidate"
mode costs N hash evaluations per target. Bits and random bases cost about 2N Legendre/sqrt (unpriced
native work, matched across arms). Priced total = GAE + w·(hash evaluations) at w = 0, 0.1, 1 and cost per
relation = total/(Q·rate) for Q probed targets. Hash/random total ratio from the design numbers
(N^2 vs p): w = 0.1: 6.0 to 21.2; w = 1: 50 to 203 (`analysis/RESULTS.md` regenerates it). Two arms are
ranked only where the order holds across the band: hash vs random ties at w = 0 and loses at 0.1 and 1, so
there is no strict ranking claim. No distance to rho is computed here: the generic baseline is 7.8x
rho at k = 20 (frame_search README, imported); the algebraic arms are 2.6x to 1.4e5x behind it, and this
lead cannot change either.

## 10. Failure and interpretation

- P1 fails (pooled ratio outside [0.98, 1.02] or CI outside [0.97, 1.03]): a departure was found; report its
  sign and size, check P6 and P2b first (window model), then look for the mechanism before any claim. A
  hash/rand ratio above 1.03 would still need the priced cost of section 9 to beat the random base, and
  the validation gate before any row.
- P5 fails: the detector does not see 3 % and the null in P1 is not claimable.
- P4 fails: the keyed-table detector is broken; stop and debug.
- P1p fails: underpowered; extend B, do not reinterpret.
- A positive result on hash/rand is not an index-calculus improvement: it would be a relabelling unless it
  lowers priced cost per relation over the matched random base across the band.

## 11. Estimated resources (smoke timing probe, `smoke/timing.txt`)

Table build 0.6 s (k = 20), 2.1 s (k = 22), about 10 s (k = 24, extrapolated); hash pass 1.0 s at k = 22,
about 4 s per key at k = 24; base evaluation 10 / 52 / 273 ms at N = 145 / 290 / 579. Full run about
4608 base evaluations per cell: sum over the 10 cells about 2,400 s, so 40 to 50 minutes on one core; budget
3 h. Memory: peak RSS 466 MB at k = 22 (measured), about 1.2 GB at k = 24 (the Legendre count allocates five
p-sized int64 arrays). Output: 10 compressed npz under 0.3 MB each. Smoke: 1 s CPU.

## 12. Sources checked (repository files actually read)

BRIEF.md; research/curve25519_frame_search_20261103/README.md, PROTOCOL.md (curves, frames),
scripts/frame_census.py, scripts/run_confirm_rulefix.sh; research/lead_sweep_20261110/
LEAD-EXO-coordinate-revealing-oracles/RESULT.md, LEAD-EXO-hashed-factor-base-tensor-ecdl/RESULT.md;
research/curve25519_t0_symmetry_20261102/README.md; engine output of
`/home/user/bin-archive/curve25519_family-5abc70ba family k` for k = 16..24. Not read: the other briefed
studies (hashed_ecdl_counts_20261031, closure ledger, the autolab idea notes); research-kb and the lead atlas
are unreachable here.

## 13. Notes

- Bases exclude points of E[8] (a uniform rule); the Weierstrass x of the engine's model is u + A/3, the hash
  is taken on the Montgomery u.
- The k = 24 curve differs from the frame-search study's k = 24 member (brief's choice kept).
- Hash arm replicates are disjoint rank windows of one key (slightly negatively correlated, negligible at
  B·N << M valid columns).
