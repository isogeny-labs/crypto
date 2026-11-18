# F1 SPEC - implementation spec (frozen, 2026-11-17)

Precise enough that a less capable model makes **no research choice**. Everything the
executor builds is downstream analysis of the lab binary's JSONL; the measurement
itself is the unmodified lab binary (PROTOCOL §0, §9). No Rust is written. One Python 3
standard-library file, `analyze.py`, is written in the follow-up directory. No numpy /
sympy / sage / pari.

## 1. Files

| path | who writes | content |
|---|---|---|
| `raw/binary.sha256` | executor (command) | sha256 of the lab binary |
| `raw/cc1.jsonl`, `raw/cc2.jsonl`, `raw/cc0.5.jsonl`, `raw/cc1.5.jsonl`, `raw/cc3.0.jsonl` | lab binary | one `config` line + one `run`/`skip` line per (arm, prime, curve, seed) |
| `raw/data.sha256`, `raw/analyze.log` | executor (command) | hashes; analyze stdout |
| `analyze.py` | executor | this spec; stdlib only |
| `analysis/analysis.json`, `analysis/tables.md` | `analyze.py` | statistics; verdict line |

The lab binary's record schema is fixed by the instrument (do not invent fields). IC
run keys used here: `kind, arm, p, g, curve_seed, seed, certified, ok, exhausted,
trials, relations, rels_by_lp, core_rows, core_cols, fb_cols, cantor_mul, cantor_inv,
rel_mul, la_mul, la_add, la_wied_mul, attempts`. Rho (`arm == "R_rho_neg"`) keys:
`kind, arm, p, g, curve_seed, seed, certified, ok, cantor_mul, cantor_inv` (no
`rel_mul`/`la_mul`). A `skip` line (rho capped) has `kind == "skip"`, `reason`,
`expected_steps`; treat as "rho unavailable on this cell" (do not count it).

## 2. Formulas (exact; the instrument's own unit)

For an IC run record `v` with genus `g = 3`:
```
def fpm(v, I, kappa):
    return v["cantor_mul"] + I*v["cantor_inv"] + v["rel_mul"] + kappa*v["la_mul"]
fpm_low (v)  = fpm(v, 1, 1)
fpm_mid (v)  = fpm(v, 10, g*g)   # g*g = 9  -> primary
fpm_high(v)  = fpm(v, 30, g*g)   # 9
relation_term(v, I) = v["cantor_mul"] + I*v["cantor_inv"] + v["rel_mul"]
la_term(v, kappa)   = kappa*v["la_mul"]
la_share_mid(v)     = (9*v["la_mul"]) / fpm_mid(v)
```
For a rho record `v` (no `rel_mul`/`la_mul`): `fpm(v, I, kappa) = v["cantor_mul"] +
I*v["cantor_inv"]` (treat missing `rel_mul`, `la_mul` as 0). All arithmetic is exact
Python `int`/`Fraction`; emit floats only in the final JSON. `base_size(p, cc) =
math.ceil(cc * p ** (2/3))` using `p ** (2.0/3.0)` (identical to the Rust
`(p as f64).powf(1.0 - 1.0/3.0)` for the primes in scope; checked in T2).

## 3. Pairing and statistic

`pairs_treatment[p]` = list over (curve_seed, seed in {1,2,3,4}) of
`(fpm_band(cc1_run), fpm_band(cc2_run))` matched on `(p, curve_seed, seed)`. Per-pair
ratio `ratio = fpm_band(cc1) / fpm_band(cc2)` (float). `mean_ratio[p] =
mean(ratios)`; `G[p] = 1 - mean_ratio[p]`.

A/A: `pairs_aa[p]` matches, on the same `cc2.jsonl` and same curve, `(seed 3 -> seed 1)`
and `(seed 4 -> seed 2)`; per-pair ratio `fpm_mid(B) / fpm_mid(A)`;
`G_AA[p] = 1 - mean`.

Hierarchical bootstrap (both): `rng = random.Random(20261117)`; 2000 resamples; each
resample draws 3 curve_seeds with replacement from the prime's 3 curves, then for each
drawn curve draws 4 (treatment) or 2 (A/A) seeds with replacement from that curve's
pairs, pools the drawn pairs, recomputes `1 - mean(ratio)`. CI = 2.5 / 97.5 percentiles
of the 2000 values (`sorted(vals)[int(0.025*2000)]`, `[int(0.975*2000)]`). Report
`{point, lo, hi}` per `(p, band)` for treatment and `(p, mid)` for A/A.

Rho ratio: per `(p, curve_seed, seed, cc)`, `fpm_band(c_run)/fpm_band(rho_run)` matched
on `(p, curve_seed, seed)` (rho from `cc2.jsonl`); report mean and bootstrap CI per
`(p, cc, band)`. Parameter-labelled.

Secondary S1: per `(p, cc in {0.5,1,1.5,2,3})` report mean `fpm_mid` over its
non-exhausted cells and `argmin_cc`. `cc = 1` and `cc = 2` means come from the primary
files; `0.5/1.5/3` from the secondary files (2 seeds). Descriptive only.

## 4. analyze.py CLI and output schema

```
python3 analyze.py CC1.jsonl CC2.jsonl [--secondary F ...] --out J --tables T
python3 analyze.py --selftest      # runs unit tests in §5, prints "OK" or fails nonzero
```
`analysis.json`:
```
{
 "gates": {"Q1_all_certified_ok_not_exhausted": bool, "Q2_base_size_ok": bool,
           "Q3_pairing_curveseeds_match": bool, "Q4_trials_42975_at_8209_c0_s1": bool,
           "Q5_rho_cc_independent_and_certified": bool, "Q6_attempts_all_1": bool,
           "Q7_aa_nondegenerate": bool, "any_exhausted_cells": [ "<arm/p/cs/seed>" ]},
 "treatment": { "<p>": { "low|mid|high": {"G":f,"lo":f,"hi":f,"mean_ratio":f,"n_pairs":int} } },
 "aa":        { "<p>": {"G_AA":f,"lo":f,"hi":f} },
 "shares":    { "<p>": { "cc1|cc2": {"la_share_mid":f,"rel_share_mid":f} } },
 "rho_ratio": { "<p>": { "cc1|cc2": {"mid":{"ratio":f,"lo":f,"hi":f}} } },
 "secondary": { "<p>": {"means_mid": {"0.5":f,"1":f,"1.5":f,"2":f,"3":f}, "argmin_cc": "<cc>",
                        "exhausted": ["<cc>"]} },
 "verdict": "supported" | "killed" | "inconclusive",
 "verdict_reason": "<which clause of PROTOCOL 7 fired>"
}
```
`tables.md`: a per-prime table (`p`, `G_mid {point, CI}`, `G_low`, `G_high`,
`la_share cc2/cc1`, `c/rho mid cc2/cc1`, `G_AA mid`), the S1 row, and a final line
`VERDICT: <verdict> (<reason>)`. The verdict is computed strictly by PROTOCOL §7;
`analyze.py` must not contain any other threshold.

Gate wiring: compute Q1-Q7 from the records; if any of Q1,Q2,Q3,Q5,Q6 is false on a
decision cell, set that prime's treatment verdict contribution to force
`inconclusive` and record the reason. Q4 is a hard instrument-fidelity gate: if false,
`verdict = "inconclusive"`, reason `"Q4 instrument fidelity failed"`. The
`any_exhausted_cells` list names exhausted primary cells; an exhausted primary cell at a
prime makes that prime inconclusive (PROTOCOL §9 budget note).

## 5. Unit tests (`--selftest`; exact expected values from the pilot calibration)

The pilot recorded two real lab IC runs (PILOT.md) whose exact counters pin the FPM
formulas. Hard-code them and assert:

- **T1 fpm identities, cc=1 p=8209 cell**
  `v1 = {cantor_mul:16298030, cantor_inv:370833, rel_mul:14992132, la_mul:2012026}`, g=3:
  - `fpm_low(v1)  == 33673021`
  - `fpm_mid(v1)  == 53106726`
  - `fpm_high(v1) == 60523386`
- **T1b fpm identities, cc=2 p=8209 cell**
  `v2 = {cantor_mul:9811891, cantor_inv:223134, rel_mul:8985775, la_mul:6960576}`, g=3:
  - `fpm_low(v2)  == 25981376`
  - `fpm_mid(v2)  == 83674190`
  - `fpm_high(v2) == 88136870`
- **T2 base size:** `base_size(1031,1.0)==103`, `base_size(1031,2.0)==205`,
  `base_size(8209,1.0)==407`, `base_size(8209,2.0)==814`,
  `base_size(32771,1.0)==1025`, `base_size(32771,2.0)==2049`.
- **T3 la_share_mid:** `abs(la_share_mid(v2) - 0.7487...) < 1e-3` (62645184/83674190);
  `abs(la_share_mid(v1) - 0.3409...) < 1e-3` (18108234/53106726).
- **T4 single-pair G:** with the two cells as one synthetic pair,
  `1 - fpm_mid(v1)/fpm_mid(v2) == 1 - 53106726/83674190`, assert within 1e-9 of
  `0.36531...`. (Direction check only; **not** a decision - one cell is not the paired
  statistic.)
- **T5 rho fpm:** `r = {cantor_mul:883094, cantor_inv:19976}`;
  `fpm_mid(r) == 883094 + 10*19976 == 1082854`; `fpm_low(r) == 903070`.
- **T6 bootstrap determinism:** two calls to the bootstrap with the same input and
  `random.Random(20261117)` give identical CI endpoints.
- **T7 pairing guard:** a constructed input where a `cc1` cell lacks its `cc2` partner
  raises/records a pairing error (no silent unmatched pair).
- **T8 verdict mapping:** feed synthetic `G`/CI triples and assert §7 maps them to
  supported / killed / inconclusive exactly (e.g. `G=[.3,.3,.3]` CIs excluding 0 and
  above A/A -> supported; `G(32771)=.02` -> killed; `G=[.3,.1,.08]` with 32771 CI
  including 0 -> killed; A/A hi `>0.05` -> inconclusive).

`--selftest` must print `OK` and exit 0 before `analyze.log` is trusted; the §9 command
block runs it first.

## 6. Counter placement (where each number comes from; no recomputation)

All counters are emitted by the lab binary; `analyze.py` only reads and combines them.
- `cantor_mul = cm_setup + cm_loop + cm_verify` (instrument internal; we read the total
  field). `la_mul` includes graph + peel + Wiedemann (no SGE here; `--sge` not passed).
- Relation dedup and rank are the instrument's (`run_ic`): DSU over large primes,
  certification = rank/solve success. We never re-count relations or re-run LA.
- `fb_cols` is the realized column count (`== ceil(cc*p^(2/3))` once the base fills;
  Q2). `trials`, `relations` are read for the secondary/reporting tables only.

## 7. Output size discipline

Per-run rows only; never dump per-relation data. Each `raw/*.jsonl` is one config line
plus `arms x primes x curves x seeds` run lines (primary `cc2`: `2 arms x 3 x 3 x 4`
~ 72 lines; `cc1`: `1 x 3 x 3 x 4` ~ 36; each secondary: `1 x 3 x 3 x 2` ~ 18), each a
few hundred bytes - every file well under 50 KB. `analysis.json` and `tables.md` are
summaries only.
