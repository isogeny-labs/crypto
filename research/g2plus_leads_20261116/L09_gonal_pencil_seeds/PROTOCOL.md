# L09: Free gonal-pencil relations as large-prime graph seeds (frozen protocol, 2026-11-16)

Status of everything this lead produces: `[pcc:unverified]` (one producer, shared
4-core host, no independent replay). This is not a benchmark: no leaderboard row,
no `benchmarks/` row, no speed claim, no wall-time metric. Synthetic random curves
only; nothing here is a statement about a deployed curve. Lab numbers quoted below
are `[pcc:unverified]` assertions copied from the files named.

Written-order note: this file and `SPEC.md` were written and hashed
(`frozen.sha256`) before any code of this lead existed and before any datum. The
fixture values in `SPEC.md` section 12 were computed by the coordinator with a
throw-away brute-force script over F_31 (enumerating all of F_31 x F_31) to fix
unit-test expectations; they are test fixtures, not data.

## 0. Prior art (grep only; research-kb is not installed in this session)

| path read | what it shows | consequence |
|---|---|---|
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | Lists `LEAD-ICB-large-prime-variants`, `LEAD-ICE-relation-graph-rank-large-prime`; no gonal/trigonal-pencil lead, no genus-4 non-hyperelliptic lead. | Not a duplicate. |
| `crypto/src/cryptanalysis/genus_ladder.rs`, `crypto/research/genus_ladder_20261114/RESULTS.md` | Genus 2..5 **hyperelliptic** IC (Gaudry walk, 2LP arm), FPM tallies. RESULTS.md line 205: "non-hyperelliptic curves of genus 4 and 5 are not" built. | No genus-4 non-hyperelliptic relation generator exists in the lab. Cost unit (FPM = mul + 10 inv, squarings as mul) taken from here. |
| `crypto/src/cryptanalysis/diem_quartic.rs` (lines 1799-1803 `fb_size`, 1848-1935 `LpGraph`, 2104 `residual_roots`) | Diem line relations on genus-3 plane quartics (residual 2 points), SLP/DLP arms, parity union-find yield `E - V + B`. | The 2LP yield rule is reused (re-derived in section 2.5). Residual here is 3 points, not 2. No pencil seeding there. |
| `crypto/src/cryptanalysis/smith_trigonal.rs` | Trigonal maps used to build Smith's isogeny genus 3 hyperelliptic -> non-hyperelliptic. | Trigonal map used for a transfer, not as a relation source. Unrelated mechanism. |
| `crypto/research/pencil_large_prime_20261029/README.md` (and the genus-2 pencil family listed in `L01_anchored_pencil_sieve/PROTOCOL.md` section 0) | Signed large-prime graph for genus-2 Vitse-Wallet pencil relations on Scholten transfers. | "Pencil" there is a pencil of functions through a walk divisor at genus 2, not the gonal pencils of a genus-4 curve. Not a duplicate. |
| `crypto/research/plane_quartic_program_20261018/`, `crypto/research/notes/index-calculus/` (grep "pencil", "trigonal", "Diem") | Only a pencil of quartics in a GLV note; Diem notes are about descent/quartics. | No duplicate. |
| `cryptanalysis/` (grep trigonal / gonal / ruling) | Hits are Koblitz N83 experiments ("ruling" in prose). | No duplicate. |
| sibling leads `g2plus_leads_20261116/L01..L07` (listing, L01 PROTOCOL header) | L01 is fixed-anchor pencil sieving at genus 2/3 hyperelliptic. | Different genus, different curve class, different mechanism. |

Conclusion: no executed work duplicates L09. Proceed.

## 1. Mathematics (checked; corrections to the brief)

**Model.** A smooth curve `C` of bidegree (3,3) on `P^1 x P^1` has genus
`(3-1)(3-1) = 4` and is non-hyperelliptic; `P^1 x P^1` is the split smooth quadric
and `C` is the canonical curve (adjunction: `K_C = (K_Q + C)|_C = ((-2,-2)+(3,3))|_C
= (1,1)|_C`). Its two trigonal pencils are the projections `pr_x`, `pr_y`, with
fibre classes `D_x = (1,0)|_C` and `D_y = (0,1)|_C`, `deg D_x = deg D_y = 3`,
`D_x + D_y ~ K_C`. We use this model directly, affine equation
`F(x,y) = sum_{0<=i,j<=3} a_ij x^i y^j`.

**Correction 1 (model).** The brief's "nodal plane quintic" must be a plane
quintic with **two** nodes: a plane quintic has arithmetic genus 6, so one node
gives genus 5. The binodal quintic is the projection of the canonical curve from a
rational point `O` of `C`; its lines are the (1,1)-sections of `Q` through `O`. We
therefore never build the quintic: "a line through `P1, P2`" is, by definition
here, the unique (1,1)-form `L = b00 + b10 x + b01 y + b11 x y` vanishing at
`O, P1, P2`, and its section is `O + P1 + P2 + R1 + R2 + R3`. This is exactly
Diem's line relation on the binodal quintic (5 points, 2 chosen, residual 3).

**Correction 2 (split condition of the quadric).** The two pencils are rational
only when the canonical quadric is split. If it is non-split (rank 4, non-square
discriminant) the two g^1_3 are defined over `F_{p^2}` and conjugate: there are no
rational pencil fibres. If it is a cone there is one pencil. The (3,3) model
builds the split case by construction; the lead applies only to that case, which
is a positive-density but not universal subset of genus-4 curves.

**Relations (true by construction).** For `t in F_p`:
`div((x - t)/(x - t0)) = F^x_t - F^x_{t0}`, so every x-fibre satisfies
`F^x_t ~ D_x`; likewise `F^y_t ~ D_y`. For a (1,1)-form `L`,
`L / ((x - t0)(y - s0))` is a rational function, so
`O + P1 + P2 + R1 + R2 + R3 ~ D_x + D_y`. Rows are degree-0 combinations; the
unknown classes `D_x`, `D_y` are matrix columns (arm A only ever sees `D_x + D_y`,
so it uses one column `K`).

**Correction 3 (fibre usage; the brief's hypergraph remark).** A fully split
fibre with 1 small + 2 large points has **two** large primes: it is an ordinary
edge of the 2LP graph, not a 3-uniform hyperedge. Only fibres with 0 small + 3
large points are 3-uniform. Primary arm B uses exactly the fully split,
squarefree fibres with at least one small point (0, 1 or 2 large primes). 3-LP
fibres are excluded from the primary arm and counted; an exploratory arm B3 (small
p only, exact rank) includes them (section 3).

**Correction 4 (cost of "free").** Fibres are not free: each costs one monic
cubic split test (one inversion to make it monic, `x^p mod c`) and, if split, one
cubic root extraction. There are `2p` affine fibres (the `t = infinity` fibres
have no rational point in our curve family, section 4). All pencil work is
charged to arm B in FPM.

**Expected size of the effect (prediction, not a threshold).** A fibre splits with
probability about 1/6 (S_3 monodromy), so the pencils give about `p/3` split
fibres, of which a fraction `1-(1-beta)^3 ~ 3 beta` have a small point: about
`beta p` usable rows. A line trial gives a usable row with probability about
`(1/6) * 3 beta = beta/2`. So the pencils are worth about `2p` line trials, while
the 2LP graph needs `Theta(p)` edges, i.e. `Theta(p/beta)` trials. Predicted
saving: of order `beta` in line trials at base fraction `beta`, shrinking as the
optimal `beta` falls with `p`. In FPM the pencils cost about `2p` cubic split tests,
comparable to the work of about `2p` line trials they replace, so the FPM saving is
predicted to be much smaller than the trial saving and possibly negative. Lines
through two FB points also need `C(n_FB, 2)` to exceed the trials, which requires
roughly `beta^3 p >= 8`; small `beta` at small `p` is infeasible (censored by pair
exhaustion), which is why the grid starts at 1/8. Prediction: trial saving
10-25% at `beta*` for `p = 16381`; FPM saving under 5%. Recorded before data; it
does not move any threshold.

## 2. Definitions

2.1 **Small set** `S = FB ∪ {O}`; `FB` = the first `n_FB = max(4, round(beta *
(N1 - 1)))` points of a seeded shuffle of the rational points other than `O`.
Every other rational point is a **large prime** (LP).

2.2 **Columns.** Arm A, A2, PC1, PC4: `FB` (n_FB), `O`, `K`: `n_cols = n_FB + 2`.
Arm B, B3: `FB`, `O`, `D_x`, `D_y`: `n_cols = n_FB + 3`.

2.3 **Rows.** Line row: `+1` on each of `O, P1, P2, R1, R2, R3` (as small column or
LP), `-1` on `K` (A-type arms) or on both `D_x`, `D_y` (B arms). Fibre row: `+1` on
the 3 fibre points, `-1` on `D_x` or `D_y`. A row is kept only if its points are 6
(line) or 3 (fibre) **distinct** rational points; rows are deduplicated by their
sorted point-id tuple plus kind.

2.4 **Trial.** One uniformly sampled unordered pair `{P1, P2}` of distinct FB
points not sampled before in that arm's stream. Every trial is counted, whatever
its outcome (degenerate, no split, tangent, too many LP, duplicate, accepted).

2.5 **Yield (stopping proxy).** `Y = rows_total - rank_LP`, where `rank_LP` is the
rank over `F_ell` (ell odd) of the LP part of the rows. Each LP part has at most 2
entries, all `+1`; with 1-LP rows as edges to a sink vertex, `rank_LP = V -
B`, `V` = distinct LP vertices seen, `B` = connected components with no sink edge
that are bipartite. So `Y = F0 + E - V + B` (F0 = rows with no LP, E = rows with
1 or 2 LP). This is the diem_quartic rule, re-derived. `Y` upper-bounds the
dimension of the LP-free part of the row space; the gap is measured in gate G5.

2.6 **Target.** `Y >= n_cols - 1`. (Rank of the true relation lattice tensored with a
generic `ell` is `n_cols - 1`: the degree map is the only obstruction because
`ell = 2^61 - 1 > #J` for all `p` here.) Arm B needs one more row than A because of
its extra column; that is charged to B.

2.7 **T(arm)** = number of line trials at the first moment `Y >= n_cols - 1`. In B,
pencil rows are inserted before trial 1 (so `T_B` may be 0). Cap: `min(64 * ceil(p /
beta), C(n_FB, 2))` trials; a run that reaches the cap without meeting the target
is **censored** (`T = cap`, flagged, reason `trial_cap` or `pairs_exhausted`).

## 3. Arms

All arms of one `(p, seed, beta)` cell share the curve, `O`, and `FB`.

| arm | rows | pair stream | role |
|---|---|---|---|
| A | lines only | stream 1 | baseline |
| A2 | lines only | stream 2 (independent) | A/A noise floor |
| B | pencil rows (≤2 LP) first, then lines | stream 1 (common random numbers with A) | treatment |
| PC1 | `m=1` x matched free line rows first, then lines | extra rows from stream 3; main lines stream 1 with stream-3 pairs skipped | structural comparator: are pencil rows worth as much as line rows of the same LP type? |
| PC4 | `m=4` x matched free line rows first, then lines | as PC1 | **positive control**: proves the instrument sees a supply 4x B's |
| B3 (exploratory) | pencil rows incl. 3-LP, then lines; exact-rank stopping | stream 1 | hypergraph treatment, p ∈ {251, 1021} only, not in the decision |

"Matched" (PC arms): let `(n0, n1, n2)` be the counts of pencil rows of B by number
of LP (0, 1, 2). PC_m draws line trials from stream 3, keeps accepted, non-duplicate
rows into bucket k (k = its LP count) while bucket k holds fewer than `m * n_k`, and
stops when all buckets are full or after `64 * p` stream-3 trials (shortfall
recorded). Stream-3 trials are free (not in `T`, not in FPM); the pairs they used
are excluded from the main stream of that arm (skipped, not counted).

Exact-rank sub-runs (gates G5 and arm B3): same rows, stopping on
`dim(rowspace ∩ LP-free) = rank_full - rank_LP >= n_cols - 1`, computed by
incremental Gaussian elimination over `F_ell`, `ell = 2^61 - 1`.

## 4. Instances

- Primes: `p ∈ {251, 1021, 4093, 16381}` (all prime; about 2^8, 2^10, 2^12, 2^14).
- Base fractions: `beta_idx 0..5` -> `beta ∈ {1/8, 3/16, 1/4, 3/8, 1/2, 3/4}`.
- Seeds: `s ∈ {0, ..., 19}` (20 per cell). Additional seeds are not added after data.
- Curve rule (depends on `(p, s)` only, so all `beta` of one seed share the curve):
  draw `a_ij` uniform in `F_p` from the curve stream; accept iff (i) `a_33 != 0`,
  (ii) `Ax(y) = sum_j a_3j y^j` and `Ay(x) = sum_i a_i3 x^i` have no root in `F_p`
  (irreducible cubics; then `C` has no rational point with `x = inf` or `y = inf`
  and every pencil fibre polynomial has degree exactly 3), (iii) no affine rational
  point with `F = F_x = F_y = 0`, (iv) `N1` (affine rational points) satisfies
  `|N1 - (p + 1)| <= 8 sqrt(p)`, (v) `N1 >= 16`. Otherwise redraw from the same
  stream. Geometric smoothness over `F_pbar` is not certified (confounder X4).
- `O` = first point of the seeded point shuffle; `FB` = next `n_FB` points (nested in `beta`).
- Seed derivation and streams: SPEC section 3.

## 5. Metrics and unit

Per run (one JSON line): `T`, censored flag, trial outcome counts, pencil row
counts by LP type and 3-LP fibre count, `n_FB`, `n_cols`, `V`, `E`, `F0`, `B`, `Y`,
FPM by phase. FPM = F_p multiplications (squarings included) + 10 x inversions,
counted inside the field type. Phases: `setup` (curve, enumeration; reported, not
charged to an arm), `pencil` (B, B3), `line` (all arms). `FPM_rel(arm) = FPM_line +
FPM_pencil`. Union-find operations counted separately (`uf_ops`), not in FPM.
LA proxy for choosing `beta*` only: `LA(beta) = 24 * n_cols^2` FPM (Wiedemann,
`3 n_cols` mat-vecs at assumed weight 8 per row, one mod-ell product = 1 FPM).
No wall time is recorded or used.

Paired statistics per `(p, beta, s)`:
`r_B = 1 - T_B/T_A`, `r_AA = 1 - T_A2/T_A`, `r_PC1`, `r_PC4` likewise;
`f_B = 1 - FPM_rel(B)/FPM_rel(A)`, `f_AA = 1 - FPM_rel(A2)/FPM_rel(A)`.

Per cell: median and IQR (q25, q75) of each; noise floor `N = median_s |r_AA|`
(and `N_f = median_s |f_AA|`); bootstrap 95% CI of `median r_B` (10,000 resamples
of seeds, SplitMix64 seed `0x4C09B007`).

`beta*(p)` = the grid `beta`, among those where at most 2 of 20 A2 runs are
censored, minimising `median_s (FPM_rel(A2) + LA)` (chosen from the A/A arm A2 only,
so the selection does not use A, B or PC data). If no grid `beta` qualifies at a
`p`, that `p` is INCONCLUSIVE.

## 6. Decision rule (fixed before data)

Per prime `p`, at `beta*(p)`, with `thr = 0.05 + N`:

- **SUPPORTED(p)**: `median r_B >= thr` and the bootstrap CI lower end of `median r_B` `> N`.
- **KILLED(p)**: `median r_B < thr`.
- **INCONCLUSIVE(p)**: neither (median above `thr` but CI lower end `<= N`), or any
  gate in section 7 fails at that `p`, or the positive control fails at that `p`.

Positive-control requirement at `(p, beta*)`: `median r_PC4 >= thr`. If it fails,
`p` is INCONCLUSIVE (instrument could not resolve a supply 4x larger than B's).

**Headline verdict of the lead** = the verdict at `p = 16381`. The verdicts at the
other primes and at every `beta` are reported as the trend (same rule applied
mechanically, labelled secondary). Secondary FPM verdict at the same cell: same
rule with `f_B`, `N_f`. If trials are SUPPORTED but FPM is not, report "trial
saving does not survive the pencil cost": the success criterion counts measured
cost, so that is not an improvement. Structural comparison: report `median(r_B -
r_PC1)`; a negative value means pencil rows are worth less than line rows of the
same LP type (disjoint-fibre structure), positive means more. Censored runs count
with `T = cap`; if more than 2 of 20 seeds of any arm in the decision cell are
censored, that `p` is INCONCLUSIVE.

## 7. Correctness gates (must pass before any metric is read)

- **G0 unit tests**: `cargo test --release` passes, including the F_31 fixtures (SPEC §12).
- **G1 curve**: every accepted curve meets section 4 (i)-(v); assertion in code.
- **G2 rows on curve**: for `p ∈ {251, 1021}` every accepted row, and for larger `p`
  every 97th accepted row, is re-checked: each point satisfies `F = 0`; each line
  point satisfies `L = 0`; fibre points share the fibre coordinate; the exact division
  of `G` by the three known linear factors had zero remainder (asserted on every trial).
  Any failure aborts the run (no partial data used).
- **G3 split census** (`census` command, 20 seeds per p): pooled fraction of fully
  split x-fibres and y-fibres among `t ∈ F_p`, and of fully split residual cubics
  among non-degenerate line trials (first `min(20000, C(n_FB,2))` trials of stream 1
at beta_idx 5),
  lies in `1/6 ± 0.03` at `p = 251`, `± 0.015` at 1021, `± 0.01` at 4093 and 16381.
  Also `sum_t (#points in x-fibre t) = N1` and same for y.
- **G4 union-find**: the union-find `rank_LP = V - B` equals the exact elimination
  rank of the LP part at every exact sub-run's stop (p ∈ {251, 1021}); any mismatch
  fails the gate for all p.
- **G5 proxy validity** (exact sub-runs, arms A and B, p = 251 all beta, p = 1021
  beta_idx ∈ {3, 4, 5}, 20 seeds; censored runs excluded from G5): `T_exact / T_uf - 1` per run. Gate passes iff for
  both arms the median is within [0, 0.03] **and** the median of the per-seed
  difference `(T_exact/T_uf)_B - (T_exact/T_uf)_A` is within ±0.01. If G5 fails, the
  union-find `T` is not a valid proxy and every p is INCONCLUSIVE.
- **G6 determinism**: `cell` re-run of `p=1021, beta_idx=2, seeds 0..1` reproduces
  byte-identical JSONL (sha256 compared).
- **G7 CRN sanity**: B shares A's stream and only adds rows, so `Y_B(t) >= Y_A(t)` at
  every trial count `t`; B's target is one higher. Gate: `T_B <= T_A + 64` in at
  least 95% of the (p, beta, s) runs; violations are listed (a large violation
  indicates a stream or bookkeeping bug) and a gate failure makes every p INCONCLUSIVE.

## 8. Variables held fixed and confounders

| ID | confounder | isolation |
|---|---|---|
| X1 | curve-to-curve variation | paired design: every arm of a seed uses the same curve, `O`, FB |
| X2 | pair-stream luck | CRN: B and PC use stream 1 like A; A2 measures stream noise |
| X3 | extra column in B | charged (target `n_cols - 1` with B's larger `n_cols`) |
| X4 | singular curves over extensions | rejected over F_p; residual risk ~1/p, same for all arms of a seed |
| X5 | duplicate rows inflating Y | dedup by point-id tuple (§2.3); G5 measures residual dependence |
| X6 | beta selection bias | `beta*` from A2 only |
| X7 | root-finding randomness changing FPM between arms | CZ randomness seeded from the polynomial's coefficients, identical in every arm |
| X8 | proxy `Y` vs exact rank | G4, G5 |
| X9 | 3-LP fibres discarded | counted; exploratory B3 with exact rank at small p |
| X10 | analyst choice | `analyze.py` implements §5-6 mechanically; written from SPEC before data |
| X11 | host load | metrics are counts; no timing |

## 9. Exact commands (executor runs these unchanged, from the lead directory)

```
cd /home/user/crypto/research/g2plus_leads_20261116/L09_gonal_pencil_seeds
mkdir -p raw
cargo test --release --manifest-path code/Cargo.toml 2>&1 | tee raw/g0_tests.log
cargo build --release --manifest-path code/Cargo.toml 2>&1 | tee raw/build.log
B=code/target/release/l09
$B census --p 251   --seeds 20 > raw/census_p251.jsonl
$B census --p 1021  --seeds 20 > raw/census_p1021.jsonl
$B census --p 4093  --seeds 20 > raw/census_p4093.jsonl
$B census --p 16381 --seeds 20 > raw/census_p16381.jsonl
for b in 0 1 2 3 4 5; do $B exact --p 251 --beta-idx $b --seeds 20 --arms A,B,B3 > raw/exact_p251_b$b.jsonl; done
for b in 3 4 5; do $B exact --p 1021 --beta-idx $b --seeds 20 --arms A,B,B3 > raw/exact_p1021_b$b.jsonl; done
for p in 251 1021 4093 16381; do for b in 0 1 2 3 4 5; do
  $B cell --p $p --beta-idx $b --seeds 20 --arms A,A2,B,PC1,PC4 > raw/cell_p${p}_b$b.jsonl
done; done
$B cell --p 1021 --beta-idx 2 --seeds 2 --arms A,A2,B,PC1,PC4 > raw/g6_rerun.jsonl
python3 code/analyze.py raw/ > raw/summary.json
sha256sum raw/* > raw/SHA256SUMS
```

Each command is one process, one thread. Size estimate (from the section 1
counts, about 2 microseconds and 400 FPM per trial in release Rust): the largest
cell (`p = 16381`, `beta = 1/8`) is about `3e5` trials x 5 arms x 20 seeds, of order
100 CPU-seconds; the exact sub-runs at `p = 1021` are the heaviest (dense elimination,
about 1e9 mod-ell operations per run); every command is expected under 10 CPU-minutes and under 500 MB.
If a single command exceeds 10 CPU-minutes, the executor stops it, records this
verbatim in `raw/overruns.txt`, and reruns that cell split by seeds (`--seed-from
k --seeds 5`, four parts, concatenated in seed order); no other change.

Outputs: `raw/*.jsonl`, `raw/summary.json`, `raw/SHA256SUMS`. The director's
assessment writes `RESULTS.md` (not part of the frozen protocol).
