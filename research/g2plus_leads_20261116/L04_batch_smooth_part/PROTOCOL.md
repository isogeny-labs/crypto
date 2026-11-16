# L04: batch smooth-part test of Mumford u-polynomials (frozen protocol, 2026-11-16)

Control arm for L01. Status of everything this lead produces: `[pcc:unverified]`
(one producer, shared 4-core host, no independent replay). This is not a
benchmark. It adds no leaderboard row, no `benchmarks/` row, no speed claim and
no wall-time metric. Inputs are synthetic and seeded. Lab numbers quoted here
are `[pcc:unverified]` assertions.

Order of writing: this file and `SPEC.md` were written and hashed
(`frozen.sha256`) before any code of this lead existed and before any data. The
hand values in `SPEC.md` section 9 were computed by the coordinator with a
throwaway port of `disc_character` at p = 7. They are test fixtures, not data.

## 0. Prior art (grep only; research-kb is not installed in this session)

| path read | what it shows | consequence |
|---|---|---|
| `crypto/src/cryptanalysis/genus_ladder.rs` (lines 902-1163: `ModU`, `pow_xpa`, `splits_completely`, `vrem`, `vgcd_monic`, `vdiv_monic`, `roots_of`, `disc_character`; lines 1208-1455: `fb_size_2lp`, `run_ic` factor base and relation test) | The lab's per-candidate test: optional parity prefilter `disc_character`, then `x^p mod u == x`, then Cantor-Zassenhaus roots, then "at most 2 roots outside the base" (Arm::DoubleLp, `max_lp = 2`). The base is the first `ceil(c p^{1-1/g})` abscissae `x = 0,1,2,...` with `f(x)` a nonzero square. Inversions in the relation phase are counted as Fermat multiplications (`cinv`). | This defines baseline arm A and the decision function. Its counting conventions are carried over exactly, except that inversions are counted as `inv` (unit 10) per the pass rule. A lab-convention column is also reported (section 6). |
| `crypto/research/genus_ladder_20261114/tables.md`, `genus_ladder_*` directory listing | Scale, SGE and harvest studies of the same instrument. A grep for "362" found no per-trial "362 FPM" figure (the only hits are unrelated table cells). | The brief's "about 362 FPM per trial at g = 3" could not be traced to a file. It is treated as `[pcc:unverified]` and is not used. Arm A is re-measured here. |
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | Root-finding leads `LEAD-ICE-gpu-rootfind-oracle` and `LEAD-ICE-rootfind-pairtable-oracle` concern summation-polynomial root oracles. There is no batch remainder-tree smoothness lead. | Not a duplicate. |
| grep for `remainder tree`, `product tree`, `subproduct`, `F_S mod`, `batch smooth` over `crypto/research`, `crypto/research/notes/index-calculus/`, `cryptanalysis/research` | Hits are isogeny evaluation (`isogeny_eval/sqrt_velu.rs`, `poly.rs`), the prime-field companion polynomial work and `explicit_relations_perf_20261030`. None apply a remainder tree to the smoothness test of Mumford `u` against a factor base. | Not a duplicate. |
| `crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve/PROTOCOL.md` (sections 0-1) | L01 uses the `genus_ladder` arm as baseline in FPM. | L04 supplies the per-candidate cost of the cheapest decision procedure, so L01 can compare against it. |

## 1. Mathematics checked, with corrections to the brief

Setting. `p` is an odd prime and `g` is in {2, 3}. A trial yields a Mumford
divisor `(u, v)` with `u` monic of degree `g`. The base is
`S ⊂ F_p`, `|S| = N`. In the lab, `N = ceil(p^{1-1/g})` (c = 1) and S holds the
first N abscissae with `f(x)` a nonzero square. `F_S(x) = prod_{s in S} (x - s)` is
monic, squarefree and of degree N.

The relation decision in the lab's 2LP arm, restated exactly:
`D(u) = 1` iff `u` is squarefree, `u` splits completely over `F_p`, and
`#{roots of u not in S} <= 2`.

Corrections and additions to the brief:

1. **The decision is not "≥ g−2 roots in S" alone.** The brief's condition is
   necessary but not sufficient. `u` must also be squarefree and fully split.
   The S-part test has to be followed by a cofactor test: with `G` the S-part
   and `c = u / G` (degree at most 2), `c` must split, be squarefree and be
   coprime to `G`.
2. **`gcd(u, F_S mod u) = gcd(u, F_S)` is the squarefree S-part.** It is the
   product of the distinct `(x - s)` over the roots `s` of `u` that lie in S. It
   is not "the part of u with roots in S" counted with multiplicity. A repeated
   root `s in S` appears once in `G` and once in `c`, which is why the
   coprimality check in item 1 is needed.
3. **At g = 2 the S-part is irrelevant to the decision.** There `g - 2 = 0`,
   so every squarefree split `u` is accepted. That is decided exactly by
   `chi(disc u) = 1`, which the lab prefilter already computes with no `x^p`
   step (`decided = g == 2` in `run_ic`). A batch S-part test cannot improve
   the decision at g = 2. It is predicted to be dominated, and g = 2 is kept
   only as a secondary cell.
4. **A schoolbook remainder tree does not amortize.** Reducing a degree-N
   polynomial by a monic degree-D polynomial costs `D (N - D + 1)`
   multiplications. With batch size m, the levels of the remainder tree whose
   node degree is at most N cost about `g m N` together. The product tree costs
   about `(g m)^2 / 2`. When `g m < N`, the top reduction costs about
   `g m (N - g m)`. In every regime the cost per candidate is at least about
   `g N`, the same order as the naive arm B (`g (N - g + 1)`). Remainder trees
   save only with subquadratic multiplication, where the cost per candidate is
   about `O(g M(N) log N / N)` for `m ≈ N / g`. That case is **not measured**
   here (brief: schoolbook). The report states it as an analytic remark only.
5. **Arm A costs about `log2 p` products modulo u.** The `x^p` test is about
   `(bitlen(p) - 1)(2g^2 - g) + popcount-term · g` multiplications, about 260
   at g = 3, p = 2^16. Root finding is paid only on split candidates (about
   1/6 at g = 3). The prefilter halves the `x^p` work at g = 3: `chi(disc) = +1`
   iff the number of irreducible factors is odd, which holds with probability
   1/3 + 1/6 = 1/2.
6. **A priori prediction (recorded before data).** At g = 3, p = 65521,
   N = 1626: B ≈ 3 · 1624 = 4872 FPM per candidate against A ≈ 200-350. The
   predicted ratio is R ≈ 0.05, falling further with p (N grows like
   p^{2/3}, A like log p). The predicted break-even (R = 2) is at
   N ≈ A / (2g), about 40 at p = 2^16. The predicted verdict is **killed**.
   The lead is still run: it is cheap and it gives L01 a measured control. It
   is not mathematically void, since item 4 is a heuristic cost estimate and
   not a lower bound on every remainder-tree schedule.

Corrected statement of the lead. For monic `u` of degree g, take
`R = F_S mod u`, `G = gcd(u, R)` (monic) and `c = u / G`. Then `D(u) = 1` iff
`deg G >= g - 2`, `c` is squarefree and split (degree at most 2: linear, or
quadratic with `chi(disc c) = 1`), and `gcd(G, c) = 1`. If `R = 0`, then
`G = u` and `D(u) = 1`. The question is whether computing `R` for a batch of
u by a remainder tree (or one at a time) costs fewer FPM per candidate than
arm A, by at least a factor of 2 at g = 3.

## 2. Hypothesis and exact claim

H1 (lead): at g = 3 and the lab's 2LP base size, the best batch arm (B, B_pf,
C(m), C_pf(m), schoolbook arithmetic) costs at most half the FPM per candidate
of the best A arm (A_pf, A_nopf). Both totals include root extraction.

H0: it does not.

The claim being tested is
`R(p) = FPM_total/cand(best A) / FPM_total/cand(best batch) >= 2` at each
g = 3 main prime.

## 3. Arms (identical inputs; full definitions in SPEC.md)

| arm | definition |
|---|---|
| `A_pf` (baseline, lab) | `disc_character` prefilter, then `x^p mod u == x` at g = 3 (at g = 2 the prefilter decides), then Cantor-Zassenhaus roots on survivors, then count of roots outside S. |
| `A_nopf` (baseline variant) | Same without the prefilter. |
| `A_pf_aa` (A/A control) | `A_pf` with a different Cantor-Zassenhaus seed. |
| `B` (naive batch) | `R = F_S mod u` by schoolbook reduction of the precomputed `F_S`, then the post-test of section 1. |
| `B_pf` | `disc_character` first; only `chi = +1` candidates go on to B. |
| `C_m{m}` (remainder tree) | Consecutive batches of m candidates: schoolbook product tree, remainder tree from `F_S`, then the post-test. Sweep m ∈ {1, 4, 16, 64, 256, 1024}. |
| `Cpf_m{m}` | Prefilter first; the survivors (in order) are batched as in C. |
| positive control `pc` | All of the above (m ∈ {1, 4, 16}) at g = 3, p = 65521 with the base forced to N = 8. Predicted: B and B_pf beat A_pf by more than 2x. This shows the instrument can see a batch win where one exists. |

Every batch arm extracts roots of accepted candidates by the same counted
Cantor-Zassenhaus routine, applied to `G` and `c` separately.

## 4. Fixed variables, instances and seeds

- Arithmetic: Python 3 standard library, integers mod p, schoolbook throughout.
  One thread per run.
- Base: `N = ceil(1.0 * p ** (1 - 1/g))`, capped at p, except in pc (N = 8).
  S is the first N values `x = 0, 1, ...` with `chi(f(x)) = 1`.
- `f` is monic of degree 2g+1 with its other coefficients drawn by
  `random.Random(seed_f).randrange(p)`. `f` is used only to define S.
- Inputs: T monic `u` of degree g, coefficients `u_0 .. u_{g-1}` drawn by
  `random.Random(seed_u).randrange(p)`. Correctness cells also append 256
  planted `u` (`seed_planted`).
- Root-finding RNG: `random.Random(seed_root)`, created fresh at the start of
  each arm run. A_pf_aa uses `seed_root_aa`.
- Per-cell seeds: `seed_f = base`, `seed_u = base + 50`, `seed_root = base + 70`,
  `seed_root_aa = base + 71`, `seed_planted = base + 90`.

| cell | role | g | p | N | T | base seed |
|---|---|---|---|---|---|---|
| `corr_g2_p1009` | gate | 2 | 1009 | 32 | 2048 + 256 planted | 4040201 |
| `corr_g3_p1009` | gate | 3 | 1009 | 101 | 2048 + 256 planted | 4040301 |
| `main_g3_p65521` | verdict | 3 | 65521 | 1626 | 4096 | 4040311 |
| `main_g3_p262139` | verdict | 3 | 262139 | 4096 | 4096 | 4040312 |
| `main_g3_p1048573` | verdict | 3 | 1048573 | 10322 | 1024 | 4040313 |
| `sec_g2_p65521` | secondary | 2 | 65521 | 256 | 4096 | 4040211 |
| `sec_g2_p1048573` | secondary | 2 | 1048573 | 1024 | 1024 | 4040213 |
| `pc_g3_p65521_N8` | positive control | 3 | 65521 | 8 | 4096 | 4040391 |

All primes were checked by trial division. N is the value the code computes; the
values shown were computed by the coordinator. If a code-computed N differs,
the code value is recorded and used, and the difference is noted.

## 5. Confounders and their isolation

| confounder | isolation |
|---|---|
| Input distribution: uniform random monic `u` instead of a Cantor walk. | Decision cost of A (`x^p`) and of B/C (reduction length) depends on degrees, not values (up to rare zero-trims). Only the root-extraction share depends on the acceptance rate, which is reported next to the theoretical value. |
| Cantor-Zassenhaus randomness | A/A arm (gate G7). |
| Inversion convention (lab counts Fermat multiplications) | `mul` and `inv` are counted separately. Both `fpm = mul + 10 inv` (primary) and `fpm_lab = mul + inv · (bitlen(p-2) + popcount(p-2) - 2)` are reported. The verdict uses `fpm`. It is also recomputed with `fpm_lab`, and any disagreement is reported. |
| Prefilter advantage | Every batch arm also has a prefiltered variant (B_pf, Cpf). |
| Setup cost (chi table, S, `F_S`) | Counted in `setup` and excluded from the per-candidate metric for all arms. It is reported, so amortization can be judged. |
| Partial last batch | T is a multiple of every m. For Cpf, the partial last batch is processed at its true size and its size is recorded. |
| Multiplication algorithm (schoolbook) | Fixed and stated. Subquadratic multiplication is an analytic remark only (section 1, item 4). |
| Implementation error | Gates G2-G5, G8, including a brute-force oracle at p = 1009. |
| Analyst choice | Thresholds frozen here. The verdict is computed by `analyze.py` mechanically. |

## 6. Metrics (exact counts; no wall time)

Per arm run: `mul` and `inv` counters, split into the phases `prefilter`,
`decide` (x^p test, or product tree + remainder tree + reduction + post-test),
`roots` (Cantor-Zassenhaus) and `setup`. For C arms, `decide` is also split
into `tree_prod`, `tree_rem` and `post`. Also recorded: acceptance count,
split count, the decision string and its SHA-256, and Cantor-Zassenhaus
failures.

Primary metric: `fpm_total_per_cand = (mul + 10 inv over prefilter + decide + roots) / T_cell`,
where `T_cell` is the number of input `u` (prefilter rejects included).

## 7. Correctness gates (all must pass before any metric is read)

- G1: every p is prime (trial division in code).
- G2: `python3 -m unittest` passes all tests of SPEC section 9.
- G3: within each cell, all arms (including A_pf_aa) produce identical
  decision strings.
- G4: in `corr_*` cells, every decision equals the brute-force oracle (all
  roots by evaluation at every x in F_p). Planted inputs match their
  type-expected decision (SPEC section 7).
- G5: for every accepted `u` in every arm, the product of `(x - r)` over the
  extracted roots equals `u`, and the number of roots outside S is at most 2.
  Cantor-Zassenhaus failures = 0.
- G7 (A/A): decisions are identical, and
  `|fpm_total_per_cand(A_pf_aa) / fpm_total_per_cand(A_pf) - 1| <= 0.05`
  in every cell.
- G8: `deg F_S = N` and `F_S(s) = 0` for every s in S (uncounted check).

If G1-G5, G7 or G8 fails: no verdict. Only implementation bugs are fixed, the
failure is recorded in `raw/GATES.md`, and all cells are rerun. Thresholds and
grid do not change.

- G6 (positive control): `R_pc = best A / best batch >= 2` in `pc_g3_p65521_N8`.
  If G6 fails, the outcome is **inconclusive (instrument)**, whatever the main
  cells show.

## 8. Decision rule (frozen)

For each main g = 3 cell:
`R(p) = min(A_pf, A_nopf) / min(B, B_pf, C_m*, Cpf_m*)` on `fpm_total_per_cand`.

- **Supported**: `R(p) >= 2.0` for all three g = 3 main primes.
- **Killed**: `R(p) < 2.0` for all three (kill criterion of the brief).
- **Inconclusive (p-dependent)**: otherwise. The per-p values are reported.

Secondary (no verdict): R at g = 2. The best m per cell. The component
breakdown. The measured slope of B in N against the predicted
`g (N - g + 1)`. The empirical break-even estimate `N* = A_best / (2 g)`.
Distance to rho is not applicable: this measures a sub-step, not a solve.

## 9. Commands (run from the lead directory; output under raw/)

```
mkdir -p raw
python3 -m unittest discover -s code -p 'test_*.py' -v > raw/unittest.log 2>&1
python3 code/run_cell.py --cell corr_g2_p1009 --g 2 --p 1009 --T 2048 --seed 4040201 --arms all --msweep 1,4,16,64,256,1024 --planted --oracle --out raw/corr_g2_p1009.jsonl
python3 code/run_cell.py --cell corr_g3_p1009 --g 3 --p 1009 --T 2048 --seed 4040301 --arms all --msweep 1,4,16,64,256,1024 --planted --oracle --out raw/corr_g3_p1009.jsonl
python3 code/run_cell.py --cell pc_g3_p65521_N8 --g 3 --p 65521 --T 4096 --seed 4040391 --nfb 8 --arms all --msweep 1,4,16 --out raw/pc_g3_p65521_N8.jsonl
for spec in "main_g3_p65521 3 65521 4096 4040311" "main_g3_p262139 3 262139 4096 4040312" "main_g3_p1048573 3 1048573 1024 4040313" "sec_g2_p65521 2 65521 4096 4040211" "sec_g2_p1048573 2 1048573 1024 4040213"; do
  set -- $spec
  for grp in AB C Cpf; do
    python3 code/run_cell.py --cell $1 --g $2 --p $3 --T $4 --seed $5 --arms $grp --msweep 1,4,16,64,256,1024 --out raw/$1_$grp.jsonl
  done
done
python3 code/analyze.py --raw raw --out raw/analysis.json --verdict raw/VERDICT.md
sha256sum raw/*.jsonl raw/unittest.log > raw/data.sha256
```

Resource rule: each `run_cell.py` invocation must stay under about 10 CPU
minutes and 2 GB. If one exceeds 10 minutes, stop it and rerun that single
invocation once with T halved (this fallback is frozen here). Record the change in
`raw/GATES.md`. The cell's verdict row is then flagged `T_halved`.

If a repository hook refuses a command, record the refusal verbatim in
`raw/GATES.md` and continue with what remains possible. Do not work around
it.
