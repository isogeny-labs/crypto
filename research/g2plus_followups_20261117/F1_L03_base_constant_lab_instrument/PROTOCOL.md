# F1: genus-3 2LP base constant cc = 1 against cc = 2, in the lab `genus_ladder` instrument (protocol)

Status: frozen before any decision run (`frozen.sha256`). Follow-up F1 of the g2plus pass
(L03 follow-up), coordinator 2026-11-17. Every number quoted from earlier work and everything
this follow-up produces is `[pcc:unverified]`. **Not a benchmark**: no leaderboard row, no
`benchmarks/` row, no speed claim, no wall-time metric. Implementation: [`SPEC.md`](SPEC.md).
Pilot evidence for every gate: [`PILOT.md`](PILOT.md).

## 0. Sources read, and what the repository state allows

| source | what it gave |
|---|---|
| `/home/user/crypto/research/g2plus_leads_20261116/L03_base_rebalance/{ASSESSMENT,PROTOCOL}.md` | The claim to re-measure: in a standalone port, `cc = 1` cut counted FPM_mid by 0.362 / 0.372 / 0.396 at p = 8209 / 16411 / 32771 against `cc = 2`, with LA share 0.745 / 0.757 / 0.772 at `cc = 2` `[pcc:unverified]`. Its nine curves, `ref/instances_L03.jsonl` (sha256 `91ddea89…7148`, byte-identical copy). |
| `/home/user/crypto-c0b657b/src/bin/genus_ladder.rs` | CLI. **`--cc` is a parameter** (default 2.0), global to a process. Arms `c` (2LP + parity prefilter, recorded as `C_double_lp+pre`) and `R` (negation rho, `R_rho_neg`). Curves are found by a deterministic search from curve seed 1 upward (first `--curves` admitted); solver seeds are `1..--seeds`; target seed = curve seed. |
| `/home/user/crypto-c0b657b/src/cryptanalysis/genus_ladder.rs` 1207-1213, 1276-1530 | `nb = min(ceil(cc · p^(1−1/g)), p)` (`fb_size_2lp`); the factor base is the first `nb` abscissae `x = 0, 1, …` with `chi(f(x)) = +1`, so `fb_cols = nb` whenever at least `nb` such `x` exist. `run_ic`: RNG `StdRng::seed_from_u64(seed)`, independent of `cc`. |
| `/home/user/crypto-c0b657b/research/genus_ladder_20261114/PROTOCOL.md` §4 | The instrument's unit (used here, not the generic "mul + 10 inv"): `FPM = cantor_mul + I·cantor_inv + rel_mul + kappa·la_mul`, `(I, kappa) ∈ {(1, 1), (10, g²), (30, g²)}`, middle primary; at g = 3 the bands are lo (1, 1), **mid (10, 9)**, hi (30, 9). Rho: `cantor_mul + I·cantor_inv`. |
| `/home/user/crypto-c0b657b/research/genus_ladder_scale_20261115/PROTOCOL.md` | The genus-3 grid this follow-up re-uses: `--primes 8209,16411,32771 --genera 3 --curves 3 --seeds 2 --arms acR --cc 2.0 --max-h 8 --rho-cap 3e7 --max-trials 2e8 --seed-search 600`. Its data file `scale_g3.jsonl` is **not** in the scratch worktree nor in `/home/user/crypto` (lost in the 2026-11-16 merge); only its means quoted in L03 `analysis/tables.md` (Q7) and one per-run count in L03 `ASSESSMENT.md` survive. |
| `/home/user/crypto-c0b657b/research/genus_ladder_sge_20261115/{PROTOCOL.md,paired.py}` | The lab's paired-ratio style (hierarchical bootstrap, curves then seeds). Adopted in `f1.py`. |

Binary: `/home/user/crypto-c0b657b/target/release/genus_ladder`, sha256
`50e5986e8e80ee98d7d80c9cbedc169f7b732c84d571bee9b74acbd123a030ff`, built at commit
`a552dc2ee8`. It is **not** the scale study's binary (`61ccabd2…`). Gate G6 checks that it
reproduces the scale study's surviving counts exactly; the pilot shows it does.

Not available in this container, recorded rather than worked around: the `research-kb` MCP
server and `scripts/research-kb` (the bad merge removed `scripts/`); `ecdb` and
`scripts/ecdb.sh`; `scripts/checkpoint.sh`; `tools/benchrec.py`; `tools/pcc/pcc.py`. Curve
facts come from the instrument's own instance search (G1). The prior-art statement is limited
to the L03 table of 2026-11-16 (nobody varied the 2LP constant at any genus in the lab
instrument; the 2026-11-14 study lists "tune the base constant" as an open step).

## 1. Corrections to the brief

1. **The constant is exposed.** `--cc` exists, so no source edit, no copied crate and no
   count-identity port is needed. The whole study runs the lab binary unchanged.
2. **Base formula.** `nb = ceil(cc · p^{2/3})` at genus 3 (not "about `cc · p^{1−1/g}`"; it is
   exactly that, ceiling, capped at p). Values used (computed, and checked against the binary by
   G2 in the pilot at p = 8209 and 8219):

   | p | cc 0.5 | 1.0 | 1.5 | 2.0 | 3.0 |
   |---|---|---|---|---|---|
   | 8209 | 204 | 407 | 611 | 814 | 1221 |
   | 16411 | 323 | 646 | 969 | 1292 | 1938 |
   | 32771 | 513 | 1025 | 1537 | 2049 | 3073 |

3. **Unit.** The instrument defines its unit (§0), so it is used: FPM with inversions at
   I = 1 / 10 / 30 and LA multiplications modulo r at kappa = 1 / 9 / 9. The brief's
   "FPM = mul + 10 inv" is the mid band's inversion price; the mid band additionally prices
   `la_mul` at kappa = 9. The decision is on the mid band.
4. **"Same curves the scale study used" and "the nine curves L03 used" are the same nine
   curves.** p = 8209: curve seeds 6, 18, 23; p = 16411: 8, 27, 43; p = 32771: 8, 16, 31. The
   binary cannot take a curve as input; it re-finds them by its deterministic search (G1).
5. **Seeds.** The scale study ran solver seeds 1, 2. Runs are cheap once the instance search
   is done, so this follow-up runs seeds 1..8 on every curve (24 pairs per prime). Seeds 1, 2
   coincide with the scale study's; G6 uses that.
6. **Common random numbers.** The solver RNG depends only on the seed. For one (curve, seed),
   the walk, the jump table and the root-finding draws are identical at every `cc` up to the
   first linear-algebra attempt; Wiedemann's draws then fall at different stream positions. So
   `cc` arms are paired by (curve, seed), and the pairing is partial, not exact. The A/A arm
   uses different seeds and is therefore an unpaired noise floor (an upper bound on the noise of
   the paired comparison).
7. **The instance search is expensive and not solver work.** Each process repeats the
   deterministic order computation for every tried curve seed (pilot: 87 CPU-s at 8209; the
   16411 and 32771 figures are in PILOT.md). It is charged to no arm (it is the same for every
   arm and identical to the scale study's). Because `--cc` is global per process, the study
   needs one process per (p, cc), each repeating the search. That is why the secondary sweep
   is ordered last and is optional at p = 32771 (§9).
8. **L03's "36-40%" is a port figure**, at seeds {3, 4} (evaluation set) and statistically
   calibrated, not bit-identical. This follow-up does not reuse any L03 run.

## 2. Hypothesis and exact claim

H (primary): on the nine genus-3 curves above, in the lab instrument at commit `a552dc2ee8`
(arm `C_double_lp+pre`, generic Cantor, parity prefilter, Wiedemann, certificate on every run),
the base constant `cc = 1.0` lowers mean counted FPM at the mid band, paired over
3 curves × 8 seeds per prime, by at least 15% against the lab's `cc = 2.0` at each of
p = 8209, 16411, 32771:

    G_p = 1 − Σ_k FPM_mid(cc=1, k) / Σ_k FPM_mid(cc=2, k),  k = (curve, seed) over the 24 pairs.

The claim is a re-tuning of a constant in the lab's baseline. It changes no exponent and is not
an ordering against rho; the IC/rho ratio is reported as the distance remaining.

Model expectation from L03 (not a threshold): G_p ≈ 0.36-0.40 at all three primes.

## 3. Arms

| arm | process(es) | role |
|---|---|---|
| **B** baseline | `--arms cR --cc 2.0`, seeds 1..8 | the lab's arm and constant (`C_double_lp+pre`) |
| **T** treatment | `--arms cR --cc 1.0`, seeds 1..8 | the re-tuned constant |
| **R** negation rho | the `R` arm of the B process (its copy in the T process is G5's negative control) | the instrument's own rho, for the IC/rho distance |
| **AA-det** A/A, determinism | a second `--arms c --cc 2.0` process, same seeds | must be count-identical to B (G4) |
| **AA-seed** A/A, noise floor | B split into even seeds against odd seeds, matched by curve | unpaired noise floor `G_AA` |
| **NC** negative control | rho rows of the T process against those of the B process | `--cc` must not change rho (G5) |
| **PC** positive control | means of `trials` and `la_mul`, T against B | the trade-off is visible: cc = 1 needs more trials and fewer LA multiplications (G7) |
| **S** secondary sweep | `--arms c --cc {0.5, 1.5, 3.0}` | shape of the cost curve; reported, never decisive |

Each arm pays its own set-up inside its counts: the character table ((p−1)/2 squarings, in
`rel_mul`, same for both), the factor-base scan (8 multiplications per scanned x, in `rel_mul`,
proportional to the arm's own `nb`), the 16-jump table (in `cantor_*`), every relation-phase
trial, every LA attempt including failed ones, and the certificate multiplication. No arm uses a
table another arm does not pay for. The instance search (curve order) is not charged to any arm,
identically for IC and rho.

## 4. Instances and seeds

- p ∈ {8209, 16411, 32771} (first primes ≥ 2^13, 2^14, 2^15), genus 3, `--curves 3 --max-h 8
  --seed-search 600`: the first three admitted curve seeds, which must be the nine of §1.4 with
  f, order, h, r equal to `ref/instances_L03.jsonl` (G1).
- Solver seeds 1..8 (`--seeds 8`); target seed = curve seed (instrument rule).
- `--max-trials 2e8`, `--rho-cap 3e7`, `--threads 1`, `restart` = 4096 (default; no
  `--restart-sqrt`), no `--sge`, no `--smith`, no `--quartic`.
- Run count per prime: B 24 IC + 24 rho, T 24 IC + 24 rho, AA-det 24 IC, S 3 × 24 IC.

## 5. Metrics

Per run, from the instrument's JSONL record (exact counts): `trials`, `relations`,
`rels_by_lp`, `core_rows`, `core_cols`, `attempts`, `fb_cols`, `cantor_mul`, `cantor_inv`,
`rel_mul`, `la_mul`, `la_add`, `certified`, `ok`, `exhausted`; rho: `cantor_mul`, `cantor_inv`.
Derived (SPEC §3): FPM at lo / mid / hi; relation-phase share `(cantor_mul + I·cantor_inv +
rel_mul)/FPM`; LA share `kappa·la_mul/FPM`; IC/rho ratio of means; G_p. Wall time is recorded
by the binary and never used.

## 6. Statistics

- G_p, and every ratio, is a ratio of sums over the same (curve, seed) keys.
- Interval: hierarchical paired bootstrap, 4000 resamples, `random.Random(20261117)`: draw the
  3 curves with replacement, then within each drawn curve draw its 8 seeds with replacement; the
  same keys are used in numerator and denominator. Percentile interval at indices
  `int(q·3999 + 0.5)`, q = 0.025, 0.975.
- `G_AA`: same estimator with odd seeds (1,3,5,7) as denominator and even seeds (2,4,6,8),
  re-keyed to (curve, seed − 1), as numerator, within B.
- Counts reported beside G: pairs in which cc = 1 is cheaper at the band.
- Three curves per prime makes the curve-level resampling coarse; this is a declared limit,
  not corrected for.

## 7. Correctness gates (all shown reachable in PILOT.md; any failure → verdict VOID)

| id | gate | pilot evidence |
|---|---|---|
| G0 | every required file exists: `p{P}_cc1.0_main`, `p{P}_cc2.0_main`, `p{P}_cc2.0_rerun` for the 3 primes | layout exercised on p = 8219 dress rehearsal |
| G1 | each file's instance lines are curve seeds of §1.4 with f, order, h, r, g equal to `ref/instances_L03.jsonl` | search at 8209, 16411, 32771 (PILOT §1) |
| G2 | config `cc` equals the process's cc and `threads` = 1; every IC run has `fb_cols = ceil(cc·p^{2/3})` | 8209 cc 2; 8219 cc 0.5, 1, 2, 3 |
| G3 | every expected (curve, seed) run present for each arm in the file (IC 24; rho 24 in main cc 1, 2 files), each `certified = ok = true`, `exhausted = false`; no `skip` row; file ends with a `done` row | 8219 dress at 24 runs; 32779 cc 0.5 and 3 (largest size, both ends of the sweep) |
| G4 | the rerun file equals the B file on every IC run, all fields except `wall_ns` | 8219 cc 0.5 re-run; 8219 dress |
| G5 | rho rows (all fields except `wall_ns`) are identical in the cc = 1 and cc = 2 files | 8219 cc 0.5 against cc 3; 8219 dress |
| G6 | reproduction of the scale study with this build: (8209, curve 6, seed 1, cc 2) `trials` = 42975, and the mean `trials` over (8209, 3 curves, seeds 1, 2, cc 2) is within 0.5 of 43964 | pilot: 42975 and 43963.83 |
| G7 | aggregate positive control at each prime: Σ trials(cc 1) > Σ trials(cc 2) and Σ la_mul(cc 1) < Σ la_mul(cc 2) over the 24 pairs | 8219 dress |
| G8 | `group_ops = setup_ops + walk_ops + verify_ops` on every IC run | 8219 dress |
| G9 | A/A noise floor: abs(`G_AA`) at band mid (point) ≤ 0.05 at every prime | 8219 dress: `G_AA` = 0.0084 [−0.0075, 0.0241] |

**Deduplication and rank (lesson b).** The instrument does not deduplicate relations. A
relation is one accepted walk divisor; the same divisor could be accepted twice only if the
walk revisits it. The birthday estimate is `trials²/(2r)`: at most about 0.1 revisits per run at
the largest trial counts here (1.3·10^5 trials at r ≥ 7.9·10^10, p = 8209, cc = 0.5; about
10^6 trials at r ≥ 4.4·10^12, p = 32771), and a revisit only matters if that divisor is also
smooth. The stop rule is the lab's: union-find cycles ≥ nunk + 8 + nunk/100 (a graph count,
not a rank), then rows are built and peeled and the core must satisfy `core_rows ≥ core_cols +
8 + core_cols/100` (the rank check before Wiedemann), then Wiedemann and the certificate
`[t]G = T` checked by two independent scalar multiplications. A rank-deficient system cannot
yield a wrong answer; it yields a failed attempt whose full cost stays in the counts, and the
walk resumes. `attempts` is reported per cc; a mean above 1 is reported, not excluded.

## 8. Decision rule (numeric, fixed before data)

On band mid, cc = 1.0 against cc = 2.0, if every gate G0-G9 passes:

- **Supported**: G_p ≥ 0.15 **and** its 95% lower bound > 0 at every one of p = 8209, 16411,
  32771.
- **Killed**: G_32771 (point) < 0.05.
- **Inconclusive**: otherwise.
- **Void**: any gate fails. Nothing is rerun with other parameters under this protocol.

**Mandatory band label.** The pilot on the non-decision prime 8219 (PILOT §5) showed that the
sign of the effect depends on the price of an LA multiplication: G = 0.361 at mid but −0.289 at
lo (kappa = 1), because at cc = 2 the LA share is 0.74 at mid and the relation phase dominates
at lo. The verdict is therefore always stated together with the label `band_robust`
(95% lower bound > 0 at lo, mid and hi at every prime) and the list `sign_flip_at_lo` (primes
where G_lo < 0). A supported verdict that is not band-robust is reported as "supported at the
instrument's primary pricing (kappa = g² = 9); reversed at kappa = 1". The thresholds above do
not change with the label.

Reported beside the verdict, never changing it: G at bands lo and hi with intervals; `G_AA` per prime; the
secondary sweep with its argmin (same seeds select and evaluate: winner's curse, labelled);
the IC/rho ratio at every cc and band; the L03 model expectation (0.36-0.40) as agree / disagree.

## 9. Exact commands

`B=/home/user/crypto-c0b657b/target/release/genus_ladder`, run from the follow-up directory,
at most two processes at a time, each under `python3 tools/rt.py LABEL CMD…` (records CPU
seconds and peak RSS to `raw/RUNTIME.log`) and logged verbatim in `raw/COMMANDS.log`. `COMMON =
--genera 3 --curves 3 --seeds 8 --max-h 8 --threads 1 --rho-cap 3e7 --max-trials 2e8
--seed-search 600`.

Order (primary first, so a truncated session still decides):

1. For P in 8209 16411 32771:
   `$B run --primes P COMMON --arms cR --cc 2.0 --out raw/pP_cc2.0_main.jsonl`
   `$B run --primes P COMMON --arms cR --cc 1.0 --out raw/pP_cc1.0_main.jsonl`
2. For P in 8209 16411 32771:
   `$B run --primes P COMMON --arms c --cc 2.0 --out raw/pP_cc2.0_rerun.jsonl`
3. Secondary, for P in 8209 16411 32771, for CC in 0.5 1.5 3.0:
   `$B run --primes P COMMON --arms c --cc CC --out raw/pP_ccCC_main.jsonl`
   (optional at P = 32771; if skipped, say so; it never affects the verdict).
4. `python3 f1.py gates raw analysis/gates.json`, then
   `python3 f1.py analyze raw analysis/analysis.json analysis/tables.md`,
   then `sha256sum raw/*.jsonl > raw/data.sha256` and `sha256sum $B > raw/binary.sha256`.

CPU budget (PILOT §4): each process is dominated by the instance search; at p = 32771 a single
process exceeds the brief's "about 15 CPU-minutes" (see PILOT.md for the measured search time),
because the search cannot be skipped or shared across `--cc` values. This is declared here and
is not a reason to change the grid. Peak memory per process is far under 2 GB (pilot: 13 MB).

If a hook refuses any command, record the refusal verbatim in `raw/COMMANDS.log` and stop;
do not work around it.

## 10. Confounders and how each is isolated

| confounder | isolation |
|---|---|
| different build from the scale study | G6 exact reproduction of surviving scale counts |
| instance drift | G1 against `ref/instances_L03.jsonl` |
| `--cc` silently ignored or mis-rounded | G2 on every run |
| RNG / seed effects | common seeds across cc, paired bootstrap; A/A noise floor `G_AA` |
| non-determinism, scheduling, threads | `--threads 1`; G4 identical rerun; G6 matches a 4-thread study |
| `--cc` leaking into rho | G5 |
| unequal set-up charging | every arm's own table and scan are inside its own counts (§3) |
| failed LA attempts hidden | all attempts charged; `attempts` reported |
| unpriced additions (`la_add`, `rel_add`) | the instrument's unit does not price them; mean `la_add` per cc reported |
| kappa (price of a mod-r multiplication) | bands lo / hi reported with intervals; mandatory band label (§8). kappa = g² is the instrument's own justification (schoolbook mod r ≈ p^g against mod p); it is not re-derived here |
| winner's curse | the decision compares a pre-chosen cc = 1 (from L03), not a selected optimum |
| shared host | counted units only; wall time never used |

## 11. Limits

Three curves per prime, small primes (≤ 32771), one genus, generic Cantor arithmetic, no SGE.
`[pcc:unverified]` throughout: no pcc claim, no validator, no replay certificate. A supported
verdict makes cc = 1 a candidate default for the lab's genus-3 2LP arm, to be confirmed by an
independent replay; it is not an improvement claim until that replay exists.
