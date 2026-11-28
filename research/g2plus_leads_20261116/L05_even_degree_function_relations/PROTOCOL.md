# L05: function-relation sieve on even-degree hyperelliptic models (frozen protocol, 2026-11-16)

Everything this lead produces is `[pcc:unverified]`: one producer, a shared
4-core host and no independent replay. This is not a benchmark. There is no
leaderboard row, no `benchmarks/` row, no speed claim and no wall-time metric.
All curves are synthetic random curves over toy primes. Lab numbers quoted here are
`[pcc:unverified]` assertions copied from the files named.

Order of writing: this file, `SPEC.md` and `DERIVATION.md` were written before
any code of this lead and before any datum. `PROTOCOL.md` and `SPEC.md` were then
hashed into `frozen.sha256`. The hand values in `SPEC.md` section 12 come from a
throw-away coordinator script at `p <= 101`. They are unit-test fixtures, not
data.

## 0. Prior art (grep only; research-kb is not installed in this session)

| path read | what it shows | consequence for L05 |
|---|---|---|
| `crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve/PROTOCOL.md`, `SPEC.md` (frozen sha256 `46168ae2…` / `cb7c0918…`) | The same-pass lead L01 freezes the anchored pencil sieve `lambda = (y - a0(x))/Pi(x)` with `g` fixed anchors on **odd-degree** models `deg f = 2g+1`, together with its counting conventions. It has no code or data yet. | L05's arm O is the L01 sieve, transcribed from L01's frozen SPEC sections 3, 4 and 7 (re-specified here so L05 is standalone). L05 adds only the even-degree families. Not a duplicate. |
| `crypto/research/more_leads_20261019/` (cited in L01 section 0), `pencil_*_2026102x/` | A genus-2 pencil sieve anchored on a Cantor-walk divisor, run on odd models and Scholten transfers. | No even-degree model, no `inf+-` bookkeeping. |
| `crypto/src/cryptanalysis/genus_ladder.rs` (lines 365–389: `"deg f must be 2g + 1"`), `hyperelliptic_index_calculus.rs` | The lab's hyperelliptic IC instruments accept odd-degree models only. | Confirms the gap the lead targets. |
| `cryptanalysis/suite/src/prime_hyperelliptic/curve.rs` lines 9–11, 53, 116 | "Degree `2g+2` (the 'real' model with two geometric points at infinity) is explicitly unsupported: its Jacobian needs additional infinity bookkeeping". | Same gap, sibling repository. |
| `cryptanalysis/experiments/hyperelliptic-cover-infrastructure/README.md` lines 25–37 | Rejects the `prime_quadratic_pullback_v1` genus-2 sextic (two rational points at infinity). Lists "prime even-degree/two-infinity Jacobian arithmetic, full-Jacobian relation collection" as unsupported. | No executed even-degree relation collection exists. L05 is not a duplicate. |
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | No lead mentions even-degree models, `2g+2` or two points at infinity. | — |
| `crypto/research/genus_ladder_20261114/`, `genus_ladder_scale_20261115/RESULTS.md` | The lab baseline (a Cantor walk) is odd-degree only. Its g = 2 2LP arm is 3–5x above rho `[pcc:unverified]`. | L05 compares against the pencil sieve, not against the walk, as the brief asks. |
| `crypto/research/plane_quartic_program_20261018/` (listing), `src/cryptanalysis/{quartic_ladder,diem_quartic,plane_quartic_pic,bielliptic_quartic}.rs` (grep for `2g+2` and "even degree") | These are plane-quartic (non-hyperelliptic) work. Nothing about even hyperelliptic models. | — |
| `crypto/research/notes/index-calculus/*` (grep) | The hits for "even degree" and "balanced" are unrelated (Koblitz, Artin–Schreier). | — |

The brief's "1,940 degree-2 cover maps" is an `[pcc:unverified]` assertion that L05
does not check. L05 makes no statement about any cover map.

## 1. Mathematics (checked; full derivation in `DERIVATION.md`)

The curve is `y^2 = f(x)` with `f` monic squarefree of degree `2g+2`, so the two
points `inf+` and `inf-` are rational. Let `s` be the monic polynomial part of
`sqrt f` (`deg s = g+1`, `deg(f - s^2) <= g`). Then:

- If `deg a <= g`: `div(y - a) = sum_{2g+2} (P_k) - (g+1) inf+ - (g+1) inf-`.
- If `a` is monic of degree `g+1`: `div(y - a) = sum_{d} (P_k) + (g+1-d) inf+ - (g+1) inf-`,
  where `d = deg(f - a^2)`. If `a` agrees with `s` in exactly the top `e+1`
  coefficients, `0 <= e <= g`, then `d = 2g+1-e`.

Corrections to the brief, each also stated in `DERIVATION.md`:

1. **Wrong residual degree in the brief's `deg a <= g` case.** With `g` anchors the
   residual has degree `g+2`, not `g+1`. The split probability is `1/(g+2)!`, so
   this family costs about `g+2` times the odd sieve per relation. That is the
   wrong linear system, not a model effect. L05 keeps it as arm E_low, the
   positive control of known effect size.
2. **The right analogue is monic `a` of degree `g+1`.** With leading cancellation
   only (`e = 0`) there are `g` anchors, the residual has degree `g+1` and rows
   have `2g+1` affine entries. This is arm E_0, the primary candidate. Each extra
   cancellation `e` replaces one affine anchor by a condition at `inf+`. At
   `e = g-1` (arm E_1a) one anchor remains, rows have `g+2` affine entries, and
   with an inverse table the per-point cost is one multiplication. At `e = g`
   (arm E_g) there is no anchor at all. `phi = y - t_g(x)` is a degree-(g+1)
   function with its only pole at `inf-`. It gives about `p/(g+1)!` relations,
   using additions only, and only once per curve.
3. **One extra column, not two.** Only `delta = [inf+ - inf-]` enters. In the
   sign-symmetric normalization `l(Q) = log[Q - inf-] - delta/2` (valid modulo
   the odd prime `r`), a depth-`e` row is `sum eps_k l(Q_k) + ((e+1)/2) delta = 0`,
   and a `deg a <= g` row has no `delta` term.
4. **"No Jacobian arithmetic" holds only for the relation phase.** Descent of `G`
   and `T` on an even model needs balanced-divisor arithmetic, or a move to an
   odd model, which exists only when `f` has a rational root. Not measured here.
5. **Any gain is not "because even".** The linear systems `|(g+1) inf- + (g-e) inf+|`
   exist on every model of the curve. A cost difference between E_1a and O is a
   difference between linear systems. A positive H2 below would mean the lab's
   odd-model sieve should also use such systems (through `W+-`). It would not
   mean that even models are cheaper.

Verdict of the check: the lead is mathematically sound once corrections 1–3 are
applied. It is not a duplicate. `proceed = true`.

## 2. Hypotheses and exact claims

- **H1 (primary, the brief's claim).** Take random genus-g curves with an even
  model, `g in {2, 3}` and `p in {1031, 4099, 16411, 65537}`, full base. On the
  even model, the pencil sieve E_0 collects relations that all pass the
  reference checks. Its relation-phase cost in FPM per distinct relation is at
  most 2.0 times that of the odd-degree sieve O run on the odd model **of the
  same curve** (same `J`, same `p`, same column count), with the same
  instrument.
- **H2 (secondary, an improvement claim inside the pass).** The one-anchor
  even family with an inverse table (E_1a_T) costs at least 1.2 times fewer FPM
  per distinct relation than the odd sieve with the same inverse table (O_T), on
  the same curves.
- **H3 (secondary).** On curves whose sextic has **no** rational root (no odd
  model over `F_p`), E_0 costs at most 2.0 times O's FPM per relation, where O
  is run on the odd models of the root curves of the same cell. This comparison
  is unpaired.
- **H4 (reported, no threshold).** Mean affine row weight and maximum column
  weight per arm. The prediction is that E_1a rows have `g+2` affine entries and
  O and E_0 rows have `2g+1`.

## 3. Arms

All arms use Python 3 with the standard library only, one thread, and one counted
field per arm (`SPEC.md` section 4). FPM = multiplications (squarings included)
+ 10 x inversions. Every arm is the same routine, `sieve_pencil` (SPEC 7), with
a different `(model, t, j, R)`:

| arm | model | top part `t` | anchors `j` | residual `R` | inversion of `Pi(x)` |
|---|---|---|---|---|---|
| O | odd model of the curve (`F`, SPEC 5) | 0 | g | g+1 | Montgomery batch per pencil |
| E_0 | even | `x^(g+1)` | g | g+1 | batch |
| E_low | even | 0 | g | g+2 | batch |
| E_1a | even | `t_{g-1}` | 1 | g+1 | batch |
| E_g | even | `t_g` | 0 | g+1 | none (`Pi = 1`) |
| O_T | odd model | 0 | g | g+1 | inverse table, `Pi^-1 = prod inv[x - x_i]` |
| E_0_T | even | `x^(g+1)` | g | g+1 | inverse table |
| E_1a_T | even | `t_{g-1}` | 1 | g+1 | inverse table (`Pi^-1 = inv[x - x_1]`) |

Each `*_T` arm uses the same anchor stream as its batch twin, so it processes
the same pencils and must produce the identical relation set (gate G5). The
inverse table costs `p - 2` multiplications. It is built once per run and
charged to the `*_T` arm.

A run is pencil by pencil until the number of distinct relations reaches
`N_target = n_cols + 8 + n_cols // 100`. The pencil in progress is finished.
`FPM_per_rel = arm FPM / distinct relations`. E_g has a single pencil. Its
counts are reported and it takes no part in any decision.

Positive control: E_low against E_0 on the same curve. The predicted cost ratio
is `g+2` (4 at g = 2, 5 at g = 3), because only the residual degree differs.

Arms per curve class:
- root curves (the even sextic/octic has a rational root, so the odd model exists):
  O, E_0, E_low, E_1a, E_g, O_T, E_0_T, E_1a_T. Solver seeds 1 and 2.
- rootless curves: E_0, E_1a, E_g, E_1a_T. Solver seed 1.
- odd random curves (L01 rule `random_f`, a population check that is reported
  only): O and O_T, solver seed 1.

## 4. Instances, seeds and variables held fixed

- Relation phase: `g in {2, 3}`, `p in {1031, 4099, 16411, 65537}`.
- Gate: g = 2 at `p in {101, 151, 199}`, g = 3 at `p in {37, 53}`.
- Even curves: `f = random_f_even(p, g, s)` (SPEC 3.2), for `s = 1, 2, ...`. Root
  curves are the first 3 `s` with `f` squarefree and at least one root in `F_p`;
  `alpha` is the smallest root. Rootless curves are the first 3 `s` with `f`
  squarefree and no root. Odd random curves are the first 3 `s` with
  `random_f(p, g, s)` squarefree.
- Streams: `stream_seed(tag, g, p, curve_seed, solver_seed)` (SPEC 3.3), one per
  arm family. A `*_T` arm shares its twin's tag.
- Held fixed across the arms of a pair: field, curve, `J`, `p`, column count
  (`n_odd = n_even + 1`; even arms add the `delta` column, DERIVATION section 5),
  `N_target`, counting rules, dedup rule, price `I = 10` for decisions.
- Setup is identical in kind across arms, excluded from every arm's FPM and
  reported as `setup`: the square-root and character table, the base and `f(x)`
  table, `s(x)`, the odd-model transform and the odd model's own base table.
- Verification (V1, V_inf, V2, mutations) runs on an uncounted path.

## 5. Metrics

Per run (arm, g, p, curve class, curve index, solver seed): `mul`, `inv`, `add`,
`fd_add`, `bucket_ops`, `FPM = mul + 10 inv`, pencils, candidates (pre-dedup),
distinct relations, duplicates, `FPM_per_rel`, and `rels_per_pencil / (p / R!)`.
Row statistics: histogram of affine weights, mean affine weight, fraction of
rows with a `delta` entry, maximum column weight, columns never hit, and the
SHA-256 of the sorted canonical relation keys.

Readings (a pair means the same g, p, root curve and solver seed):

- **R1** (H1): `FPM_per_rel(E_0) / FPM_per_rel(O)`.
- **R2** (H2): `FPM_per_rel(O_T) / FPM_per_rel(E_1a_T)`.
- **R2b** (reported): `FPM_per_rel(O) / FPM_per_rel(E_1a)` (batch instrument; predicted about 1).
- **R3** (positive control): `FPM_per_rel(E_low) / FPM_per_rel(E_0)`.
- **R4** (H3): `GM over rootless runs of FPM_per_rel(E_0)` divided by
  `GM over root runs of FPM_per_rel(O)`, per cell.
- **R5** (sensitivity): R1 and R2 with additions priced, i.e. `+ add + fd_add + bucket_ops`
  added to both arms' FPM, and R1 and R2 at I = 1 and I = 30.
- Cell statistic: the geometric mean (GM) over the 6 pairs of a cell (3 root
  curves x 2 seeds), with min and max.

## 6. Correctness gates (all must pass before any metric is read)

- **G1** unit tests (`SPEC.md` section 12) pass, including the hand values
  (`random_f_even`, `s`, the odd transform, `#J` by zeta from both models,
  the E_g census on the fixture curve, `V_inf` orders, and per-pencil count
  formulas).
- **G2 reference gate** (the brief's check). At every gate (g, p), on root curves
  1–3, arms E_0, E_low, E_1a and E_g are run to `N_target` with seed 1, and O on
  the odd model. **Every** collected relation must pass:
  - V1 (affine identity);
  - V_inf (Laurent orders at `inf+-` equal the recorded coefficients; even arms);
  - V2 (the odd-model Cantor sum of the mapped divisor is the identity).

  In addition, both mutations m1 and m2 of every even-arm relation must FAIL V2
  (the negative control: the reference can see a one-unit bookkeeping error).
  On every gate curve, `#J` from the even model's zeta must equal `#J` from the
  odd model's zeta, and `[#J] D` must be the identity for 16 random odd-model
  divisors. The order of `delta` is reported. A failure is examined. If it is
  traced to an implementation bug (code not matching `SPEC.md`), the code is
  fixed to match SPEC, every run is redone, and both outcomes are reported. If it
  is not traced to a bug, H1 is **killed** (brief's kill criterion).
- **G3** relation-phase runs: V1 and V_inf on every relation of every arm. V2
  with m1 and m2 on 200 sampled relations per even-arm run on root curves
  (SPEC 8.3). O relations get V1 and V2 on the odd model. Any failure is handled
  as in G2.
- **G4** no bucket ever exceeds `R` hits. The per-pencil count formula (SPEC 7.5)
  holds for every pencil.
- **G5** every `*_T` arm's relation-key digest equals its batch twin's in the
  same (curve, seed).
- **G6** A/A determinism: rerunning (g = 2, p = 4099, root curve 1, seeds 1 and 2,
  all root arms) reproduces every integer field and digest exactly.
- **G7** (sanity of the split model): pooled per cell over the cell's runs,
  `candidates / (pencils * p / R!)` lies in [0.85, 1.15] for O, E_0, E_1a and
  E_low. A G7 failure makes the affected cell inconclusive and is reported. It
  does not kill.

## 7. Controls

- **Positive control** (the instrument sees an effect of known size): R3. It
  passes if `GM(R3)` lies in `[0.8 (g+2), 1.25 (g+2)]` in every cell. If any cell
  fails, the cost instrument is not trusted and every verdict is inconclusive.
- **Negative control of the reference**: mutations m1 and m2 (G2, G3). They must
  be rejected 100% of the time.
- **A/A**: G6. The between-seed spread of `FPM_per_rel` (max/min over a cell's
  runs) is reported as the noise floor.
- **Instrument identity**: G5 (T and batch twins give the same relations, so they
  differ only in pricing).

## 8. Decision rule (fixed now)

The thresholds are those of the brief (2x) plus the H2 margin. They are not
moved after data.

- **H1 killed** if any relation fails a reference check (G2/G3) and the failure is
  not traced to a bug; or if all gates and the positive control pass and
  `GM(R1) > 2.0` in any g = 2 cell or in two or more g = 3 cells.
- **H1 supported** if all gates G1–G6 pass, the positive control passes, G7
  passes in every cell, and `GM(R1) <= 2.0` in all 8 cells.
- **H1 inconclusive** in every other case: a gate fails and is traced to a bug
  that is not yet rerun, the positive control fails, G7 fails, exactly one g = 3
  cell has `GM(R1) > 2.0`, or a cell is missing (budget guard).
- **H2** (secondary). Supported if `GM(R2) >= 1.2` in all 8 cells. Refuted if
  `GM(R2) < 1.0` in any cell. Mixed otherwise. It is reported with R5.
- **H3** (secondary). Supported if `R4 <= 2.0` in all 8 cells. Refuted if
  `R4 > 2.0` in any g = 2 cell. Otherwise mixed.

Predictions (not decision rules). FPM per relation for O, E_0 and E_1a with the
batch instrument: about `2.5 (g+1)!` (about 15 at g = 2, about 60 at g = 3).
`R1` about 1.0 (E_0 has `O(g^2)` more FD multiplications per pencil, which is
negligible). `R3` about `g+2`. With the inverse table, about `6 (1 + 1/k)` for
E_1a_T against `6 (1.5 + 1/k)` for O_T at g = 2, where `k` is about 3 pencils, so
`R2` is about 1.4. At g = 3 the figures are `24 (1 + 1/k)` against
`24 (2 + 1/k)` with `k` about 12, so `R2` is about 1.9. Predicted mean affine row
weight is about `2g+1` for O and E_0, `g+2` for E_1a and `2g+2` for E_low.

## 9. Confounders and how each is isolated

| confounder | isolation |
|---|---|
| curve-to-curve variation (split rates, base size) | primary pairing on the **same curve** (even model against odd model); 3 curves per cell |
| column count | `n_odd = n_even + 1` equals the even column count (`n_even` plus `delta`); the same `N_target` |
| wrong infinity bookkeeping hidden by a self-consistent implementation | V_inf (model-free series), V2 (group law on a different model), mutations m1/m2 |
| inversion strategy | batch (R1) and table (R2) instruments both reported; T twins must give the same relation sets (G5) |
| additions and bucket traffic unpriced by FPM | R5 prices them in both arms |
| inversion price | R5 at I = 1, 10, 30 |
| a gain that belongs to the linear system, not the model | stated in section 1 (correction 5); no claim that even models are cheaper |
| relation usefulness (rows, `delta` column, anchor density) | H4 row and column statistics; no LA is run; stated as a limit |
| seed luck | 2 solver seeds on the primary pairs; A/A rerun; spread reported |
| counter bugs | G1 hand counts; per-pencil count assertion (G4) |
| setup asymmetry (`s`, odd transform, tables) | excluded from every arm, reported |
| analyst choices | thresholds, readings and the analysis program fixed here and in SPEC 10 |

## 10. Exact commands (executor, from the lead directory, one thread)

```sh
cd /home/user/crypto/research/g2plus_leads_20261116/L05_even_degree_function_relations
mkdir -p raw
python3 -m unittest discover -s tests -v 2> raw/unittest.log
for gp in "2 101" "2 151" "2 199" "3 37" "3 53"; do set -- $gp
  python3 run.py gate --g $1 --p $2 --out raw/gate_g$1_p$2.jsonl
done
for g in 2 3; do for p in 1031 4099 16411 65537; do
  for c in 1 2 3; do for s in 1 2; do
    for arm in O E_0 E_low E_1a E_g O_T E_0_T E_1a_T; do
      python3 run.py relphase --g $g --p $p --class root --curve-index $c --seed $s \
        --arm $arm --out raw/rel_g${g}_p${p}_root_c${c}_s${s}_${arm}.jsonl
    done; done; done
  for c in 1 2 3; do for arm in E_0 E_1a E_g E_1a_T; do
      python3 run.py relphase --g $g --p $p --class rootless --curve-index $c --seed 1 \
        --arm $arm --out raw/rel_g${g}_p${p}_rootless_c${c}_s1_${arm}.jsonl
  done; done
  for c in 1 2 3; do for arm in O O_T; do
      python3 run.py relphase --g $g --p $p --class oddrand --curve-index $c --seed 1 \
        --arm $arm --out raw/rel_g${g}_p${p}_oddrand_c${c}_s1_${arm}.jsonl
  done; done
done; done
for s in 1 2; do for arm in O E_0 E_low E_1a E_g O_T E_0_T E_1a_T; do
  python3 run.py relphase --g 2 --p 4099 --class root --curve-index 1 --seed $s \
    --arm $arm --out raw/aa_g2_p4099_root_c1_s${s}_${arm}.jsonl
done; done
python3 analyze.py --raw raw --out-json raw/analysis.json --out-md raw/summary.md
sha256sum raw/*.jsonl raw/analysis.json raw/summary.md raw/unittest.log > raw/data.sha256
```

Every `run.py` invocation has a CPU-time guard of 540 s (`--cpu-budget`,
default). A run that hits the guard writes `status: "budget_exhausted"` with its
counts so far. It counts as a missing cell for the decision and is reported,
not rerun with other parameters. Peak memory is expected to stay well under
2 GB (the largest objects are arrays of size `p <= 65537`). Expected total CPU
time is about 1–2 hours on one thread.

## 11. Limits

These are toy primes (at most 2^16). The claim covers the relation phase only:
no linear algebra and no descent. The DLP on an even model also needs `G` and
`T` expressed over the base, and that is not measured. Counts are Python counts,
not native code, and FPM is a stated convention (I = 10). There are three curves
per class and cell. Nothing here bears on beating rho, and nothing here is a
statement about any cover map in the lab's catalog. A supported H1 says only
that the relation phase works on even models at a cost comparable to odd
models. A supported H2 says that a cheaper linear system exists. That linear
system is available on odd models too (section 1, correction 5).
