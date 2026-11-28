# Lead 4 protocol: which eligible positions did the program visit? (frozen 2026-10-26)

**Status: frozen before `visit.gp` was written and before any model was evaluated.**
What was already known when this was written: the per-position table below (printed by
`summarize.gp`, a descriptive listing, no model), the ten published indices, and the
prior record's statement that 52,520 RNG models (PR 2378, tested on the 21 only) gave
zero exact fits. The family below was therefore designed after seeing the published
indices. There is no held-out stream, so every result here is in-sample.

Provenance question only. No ECDLP claim, no claim about the historical rule beyond what a
measured number supports. Atlas lead `LEAD-STR-curve-generation-provenance-seeds` is not
changed by this file.

## Question

Reframe: not "why were 11 rejected" but "which eligible positions did the program visit".
Take every position that passes the 1997 draft's listed checks (eligible), in each of the
six recovered streams. A visitation model says which of them the program evaluated and
accepted. Does any small, frozen family reproduce the ten published positions, and how
many eligible positions does it predict were visited?

## Inputs (read only; hashes fixed here)

| File | sha256 |
|---|---|
| `inputs/acc69.tsv` (69 prime-order positions; from `nist_seed_1997_acceptance_20261024/analysis.json`, sha256 `a32bd8b3...228aed`) | `2ef860b048524c123523d2cdc13c376a67546e522e86e73c6c5d192a2301658e` |
| `inputs/near_branches.tsv` (102 eligible `l_max=255` branches = 99 positions; from `nist_seed_1997_nearprime_20261024/runs/positive-20261024T022524Z/analysis.json`, sha256 `01b33e08...c126e33`, verified there as `status: verified`) | `f30fe801e077b6235d78ebf6215453f848dce4b1bce9434d4e1ded0595b91f29` |
| `inputs/streams.tsv` (counts only) | `023119bdfc0d6adf469902a69d9e97d373dc7f5b284591e0a7f7db848eb3a9d0` |
| `../inputs/historical.tsv` (21 curves with run, index, role) | `f425dc7c323bdbe6660fc599d1c625e1851a0e30d2a83d38a1c62d35e002f8fe` |
| `lib.gp` (loader), `summarize.gp` | `d5a51320...830219` after the E2 rename; recorded again in `out/` at run time |

`inputs/extract.py` only flattens the sibling JSON to TSV (parse, no computation).

## Per-position data located (measured, from `out/summary_data.txt`)

Positions are 1-based block indices in each timestamp stream; the stream prefix ends at the
last published index, so prefix lengths sum to 21,753 (checked). Per position the tables
give stream, index, 1997 label, root branch(es) with a certified order, bit length of the
prime part `n` of the group order, field bit length, smooth cofactor `h`. Order primality is
certified by the sibling runs (`h = 1` means full prime order; `h > 1` is a smooth cofactor
with `n` prime).

Eligible counts per stream (192-bit, P-224, 239-bit, P-256, P-384, P-521):
E1 = 5, 4, 8, 2, 1, 1 (21); E2 = 12, 12, 8, 2, 3, 1 (38); E3 = 19, 4, 17, 6, 5, 1 (52);
E4 = 26, 12, 17, 6, 7, 1 (69); E5 = 44, 17, 20, 9, 8, 1 (99). All ten published positions
lie in every set. Published indices: 192-bit {139, 5228, 6013}; P-224 {7026};
239-bit {2185, 2320, 3048}; P-256 {1842}; P-384 {2688}; P-521 {1136}.

## Eligibility sets (five cohorts, fixed)

- E1: ten published + eleven retrospective omissions (21).
- E2: prime-order positions with a qualifying selected (canonical) root branch, including
  order wider than the field (38 = 69 minus the 31 opposite-root-only).
- E3: prime-order positions with order not wider than the field, either root (52).
- E4: all 69 prime-order eligible positions.
- E5: all 99 near-prime eligible positions (`l_max = 255`; a lower bound, 10,083 positions
  are censored and treated as not eligible).

Censored and non-eligible blocks are all treated alike: a rejected block. This ignores the
difference between a cheap rejection (no square root) and a counted rejection (point
counting). That is an exclusion of this study.

## Model family (fixed grid; 1,095 strict models, 540 permissive models)

Per stream, eligible positions `e_1 < ... < e_m` (sorted), published set `Y`, `k = |Y|`.
`x` is the next block the program evaluates; blocks are 20-draw windows, so skipping `c`
extra blocks stands for consuming `20c` extra draws.

**Family T (stride scan).** Parameters:
extra consumption per rejected block `r in {0..7}` (stride `g = r+1`);
start phase `phi in {0..r}` (start `x = 1 + phi`);
extra consumption after an acceptance `a in {0,1,2,3,4,5,6,8,10,15,20,30,40,50,100}`;
stop rule `stop in {count, none}` (count = stop after `k` accepts for that stream, the
count of published curves of that stream; none = scan to the prefix end).
Pairs `(r, phi)`: 36. Strict models: `36 * 15 * 2 = 1,080`. Permissive: `36 * 15 = 540`.

Scan, for each eligible `e` in ascending order: if `e < x`, it was consumed by a skip (not
visited). Else if `(e - x) mod g != 0`, the stride jumps over it (not visited). Else visited:
- **Strict (S):** visited means accepted and published: `x <- e + 1 + a`.
- **Permissive (Pm):** if `e in Y` accepted, `x <- e + 1 + a`; else a hidden rejection (an
  unpublished eligible position, as in the eleven omissions), `x <- e + g`.
Under S the predicted published set `P` is the accepted set. Under Pm all labels are used
to drive the scan; it is valid only if every published position is visited.

**Family Q (quotas, strict only, 15 models).** Full scan (`r = 0, a = 0`, no stop). Classes
`kappa in {K1, K2, K3}`: K1 = sign(`bits(n) - bits(p)`); K2 = root branch set (canonical only,
other only, both); K3 = the pair. Quota `q in {1,2,3,4,5}` accepted per class per stream;
visited eligible beyond quota are rejected.

Not in the family (stated so they are not mistaken for tested): variable consumption that
depends on rejection reason, retry loops that restart a stream, parallel processes with
unequal progress, any model whose consumption depends on the candidate's own content
(hash-dependent skips), and all models for which the 20-draw phase is broken.
"Time-ordered runs with retries" is represented only by the stride scan with `a` and
count-stop; a restarting retry is equivalent to `a = 0, r = 0` with a published position
excluded, which the family already contains.

## Statistics (fixed)

For model `m`, cohort `E`, strict:
`D(m,E) = sum over streams |P Delta Y|`, `TP = sum |P cap Y|`, `FP = sum |P \ Y|`,
`V = sum |P|` (eligible positions predicted visited), `V/|E|`.
- **Reproduces all 10 published:** `TP = 10`. **Exact fit:** `D = 0`.
- `S1*(E) = min over the 1,095 strict models of D(m,E)`.
- Baseline (`r=0, a=0, stop=none`) has `D = |E| - 10`, `V = |E|`.

Permissive: `X(m,E) = number of visited unpublished eligible positions summed over
streams`, defined only if all ten published are visited (valid). `S2*(E) = min over valid
permissive models of X`. Baseline `X0 = |E| - 10`. `S2*` measures how few hidden
rejections a consumption model needs.

## Calibration against count-preserving random labellings

For each cohort `E`: `N = 1,000` random labellings. Each draws, per stream, `k` positions
uniformly without replacement from that stream's `E` positions (same `k` per stream as the
real data: 3, 1, 3, 1, 1, 1). `setrand(20261026)` once; draw order: replicate, cohort 1..5,
stream 1..6, partial Fisher-Yates. Strict `P` sets do not depend on labels (stop uses only
`k`), so they are computed once. For each labelling compute `S1*` and `S2*`.
`P(S <= obs) = (1 + #{null S <= obs}) / (N + 1)` (small is evidence of structure beyond
chance). Also report the null probability that `S1* = 0` (a spurious exact fit).
Multiplicity: five cohorts, so a lead is declared only at `P <= 0.01` after dividing by five
(`P <= 0.002`) on S1* **or** S2*. Nothing else counts as a lead.

**Positive control PC1.** Plant labels equal to the strict prediction `P` of model
`(r=1, phi=0, a=3, stop=count)` under E4 (counts preserved from that planted set).
The pipeline must give `D = 0` for that model and `P(S1* <= 0) <= 0.01`; otherwise the
detector cannot see a fitting rule and every null result is void.
**Negative control** is the null itself. **Determinism:** the run is repeated once and the
output files must match byte for byte (no noise source exists; clocks are not used).

## Frozen predictions (before any model value is computed)

1. No strict model has `D = 0` for E3, E4, E5. (E1 and E2 are not predicted; E1 is the
   cohort the family is most able to overfit.)
2. For every cohort, `P(S1* <= obs) > 0.002` and `P(S2* <= obs) > 0.002`: no structure
   beyond count-preserving chance.
3. Every strict model with `TP = 10` has `FP >= 1` for E3, E4 and E5, so it predicts more
   than ten eligible positions visited (`V > 10`).

If prediction 1 fails for E1/E2 and P is not small, that is overfitting of a flexible grid,
not evidence for a rule.

## Execution rule

The run is launched only as `scripts/checkpoint.sh gate && gp -q -s 300M visit.gp`; if the
repository's checkpoint gate refuses (no resume checkpoint can be uploaded), the study is
reported blocked with the exact message. No `CHECKPOINT_SKIP`. Code self-tests use
synthetic toy inputs only and are not results.

## Limits

In-sample; family designed after seeing the published indices; no held-out stream; eligible
sets are lower bounds (censored positions); acceptance equals publication under strict
models, which contradicts the eleven named omissions unless the model's skips hide them;
permissive models allow hidden rejections but do not explain them. A zero-fit result
excludes only this grid.
