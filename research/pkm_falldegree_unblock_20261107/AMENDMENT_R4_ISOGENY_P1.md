# DRAFT Amendment R4 (proposed number 16): the 2-isogeny arm at `P1`

**Status: draft, revision 2 (2026-11-14), NOT frozen.** Revision 1 was reviewed by an independent red team
(`REVIEW_R3_R4.md`): "freeze with edits", blocking edits E4.1–E4.8. This revision applies all eight; section 9 maps each edit to
the text and the evidence. The text is written on the owner's instruction "do the isogeny arm at P1 first" (2026-11-14). Target
when frozen: `research/prime_fall_degree_20261027/PREREGISTRATION.md`, appended as Amendment 16, **before any isogeny row at
`P1` exists**. None exists. Every number is `[pcc:unverified]`.

## 0. Disclosure: everything seen before this text

- **Stage A** (`RESULTS.md`, `results/R9/analysis/pfdA_summary.json`), median `d_F`:
  - Kummer `P1`, `t = 1..14`: 3 4 4 4 4 5 5 5 5 6 6 ≥6 ≥6 ≥6.
  - Kummer null `P1`: the same up to `t = 10`. H-null was unresolved; on completed `N` it reads "shape-driven".
  - Kummer `P0`, `t = 1..8`: 3 4 4 4 4 5 5 5.
  - 2-isogeny `P0`, `t = 1..8`: 3 5 5 5 5 5 5 5. Kummer naive `P1`: 4 6 ≥8 ….
- **The `t = 12` and `t = 13` brackets** (`RESULTS_T12_BRACKET.md`, `RESULTS_T13_BRACKET.md`):
  - Kummer `t = 12` is exact 6 on both draws (median 6, "holds at t = 12").
  - Kummer `t = 13`: draw 1 exact 6, draw 0 ≥ 6 (its cap-6 cell was lost three times to Spot). Reading: undecided.
- **Stage C** (`RESULTS_STAGE_C.md`): "grows in m" on all six ladders, including 2-isogeny `P0` at `t = 2, 3`.
- **Stage B:** no row exists (`STAGE_B_LOST.md`).
- **Rows produced while preparing this amendment.** None is a `P1` isogeny row:
  - `gaux/`: the 16 stage A `P0` isogeny draws rerun, equal to the admitted values;
  - `iso/selftest/`: planted `P0` isogeny and planted `P1` Kummer rows, `t ≤ 6`, used only to test the certificate.

The predictions in section 5 were written knowing all of the above.

## 1. Why

- Amendment 1 moved the isogeny arm to `P0 = 786433`, because the builder counted `#E'` with an `O(p)` Legendre loop. So every
  Kummer–isogeny comparison in this study is also a comparison between primes.
- At `P0` the isogeny ladder is flat at 5 for `t = 2..8`, while Kummer reaches 6 at `t = 10` and exact 6 at `t = 12` (`P1`).
- The open question for the isogeny work stream is whether the isogeny systems stay at 5 beyond `t = 8`. That is slide 12 of
  Petit's PKC 2016 talk: low-degree rational maps `L_j`, with `V = {L(x) = 0}`.

## 2. Instrument (code), and why it is the frozen instrument

- **Flag.** `--aux-order naive|mestre`, default `naive`. The `naive` path's output is unchanged byte for byte. `mestre` computes
  `#E'` exactly by Mestre's method on `E'` and its quadratic twist, with BSGS over the Hasse interval. It draws its points
  from a private stream seeded by `(p, a, b)`, so for the same seed the builder tries the same curves and selects the same
  `E'`, `Q`, `R'`, `V` and chain as `naive`. Rows built this way carry `"aux_order_method": "mestre"`.
- **Binary** (`iso/PINS.json`):
  - Source: frozen f13a690df016d0a457bd40579fbb9d054f486adf plus `gaux/aux_order_f13a690df0.patch`. The patch applies with
    `git apply`; the patched `examples/pkm_tower_pilot.rs` has SHA-256 `9c63c61a0db9…`.
  - Build: musl static, with the recipe of `image/build_layer_r2.sh`. `pkm_tower_pilot_r4` has SHA-256 `705b151b6f35…`.
  - `prime_fall_degree_tools` and `hky_fp_tower_check` are the frozen binaries.
  - The binary is **not** built from `main`, whose F4 engine has changed since.
- **Threads.** `RAYON_NUM_THREADS=12`, fixed by `iso/entry_iso.sh` and the runner. F4's `muladds` counter depends on the rayon
  thread count (`gaux/GAUX.md`): 1, 4 and 12 threads give different counts, and 12 reproduces the admitted rows.
- **Wrapper.** `iso/pilot_mestre.sh` runs `pkm_tower_pilot_r4 --aux-order mestre "$@"`. It is the pilot for every stage,
  including gate G3, so no frozen tool changes and G3 never falls back to the `O(p)` count.

## 3. Gates (all must pass; any failure stops R4 and is reported)

- **G-AUX** (`gaux/GAUX.md`). Checks 1 and 2 are the unit tests `mestre_exact_order_equals_point_count` (772 curves) and
  `mestre_and_naive_build_the_same_isogeny_tower`; they pass on main + patch and on frozen + patch (`gaux/TESTS_FROZEN.md`). Check 3:
  the 16 stage A `P0` isogeny draws, rerun with `--aux-order mestre`, equal `pfdA_summary.json` on `x_r`, `closure_dF`, `falls`,
  `ffd`, `closure_muladds`, `closure_max_cols`, `D` and `f4_muladds`.
  - Check 3 passed in `gaux/v3/`. It also passed through the runner, run on this machine as the integration test
    (`iso/INTEGRATION.md`).
  - Check 3 is also the `gaux` stage of every R4 job, on the job's own host, before any `P1` row: edit E4.7.
  - The pass configuration (binary `705b151b…`, 12 threads) is pinned. Any later mismatch under it is final.
- **G1** (frozen): `hky_fp_tower_check` prints "G1: 6 cases pass" (stage `g1`).
- **G2** (§4, as registered): on `A-m2-iso-p1` rows at `t ≤ 10`, every exact closure `d_F` is `≤` the refuting F4 degree `D`
  (`iso/score_iso.py`, field `G2`).
- **G3** (Amendment 11 scope): `prime_fall_degree_tools gate3 DIR iso/pilot_mestre.sh --max-n 20 --jobs 2 --budget 900` over the
  `A-m2-iso-p1*` rows (stage `g3`). It runs before any CAP or X row at `t ≥ 11` is read. A failure or disagreement follows
  Amendment 11's stop rule.
- **Mestre panic.** A log line "Mestre: #E ambiguous" is an instrument failure. The entry script stops the job with exit 3, and
  R4 stops.
- **Independent recount** (post-run). Every distinct auxiliary curve `(p, aux_a, aux_b)` in the R4 rows is recounted by
  `iso/naive_count.c` (naive Jacobi-symbol count, no shared code) through `iso/recount.py`. A mismatch voids every row on that
  curve. On the 8 `P0` curves of `gaux/v3/` the recount equals Mestre. At `P1` it takes about 4 minutes per curve on 4 threads.

## 4. Cells and commands

Every argv is fixed in `iso/entry_iso.sh`. The table is a summary; the script is binding.

| stage | output | engine and limits | cells |
|:--|:--|:--|:--|
| `A` | `A-m2-iso-p1` | `hky`, cmax 7, budget 1800, max-cols 2e6, max-entries 3e8 | isogeny, `m = 2`, tower, `t = 1..10` |
| `A-big` | `A-m2-iso-p1-big` | as `A`, cmax 5 | `t = 11..13` |
| `DA` | `DA-m2-iso-p1` | F4 (`tower`), budget 1800, max-nnz 2e9 | `t = 1..13` |
| `cert` | `CERT-m2-iso-p1` | `iso/certify_m2.py` | every row above |
| `cap-A` | `CAP-iso-m2-tT-kK-cC` | F4, budget 1800, `--cap D − 1`, `D` from `DA` (`run.sh cap-A` rule) | `t = 11..13`, both draws |
| `x:T:K:C:DENSE` | `X-iso-m2-tT-kK-cC` | F4, budget 86,400, `--max-dense` (Amendment 14: 12,800,000,000 at 56Gi), `--trace` | Amendment 7 B bracket, only where `d_F` is undecided |

- **Common to all stages:** `--p 2013265921 --ladder-t none --planted 0 --random 2`, the pilot's default seed.
- **Bracket order** (E4.5). `c*_iso(t) = 6` at `t = 11..13`, so cap 5 runs first. Cap 6 runs if cap 5 does not refute, and
  cap 7 if cap 6 does not refute. Each step's result is the next step's input, as in Amendment 7 B.
- **Decomposability** (E4.3).
  - Every draw is certified by `iso/certify_m2.py`. It counts the ordered pairs in `V × V` with `S3(x1, x2, x_R) = 0` using
    `verify.py`'s `s3_coeffs`, with one quadratic solved per `x1`, so the count is exact and `O(|V|)`.
  - Self-test: equal to `verify.py`'s brute force on 40 rows, including 24 planted rows, all of which it finds decomposable
    (`iso/selftest/`).
  - A draw with a solution is decomposable and is excluded from every median, as Amendment 7 C excludes Kummer draws.
  - A certified-inconsistent draw is never labelled "possibly decomposable".
- **`t = 14`** (λ = 0.133) is not included. With the certificate it would be valid. Adding it is the owner's decision, and
  cells would follow the same table.

## 5. Readings (fixed now; E4.4)

The scorer is `iso/score_iso.py`, committed and self-tested on synthetic rows that use the R4 file names (E4.5).

- **Per-draw `d_F`.**
  - The closure gives an exact value, or a lower bound (the frozen `d_f` semantics).
  - A refuting F4 run at degree `D` gives an upper bound `D` (G2).
  - A completed capped run at cap `C` gives `≥ C + 1` if it does not refute, and `≤ C` if it does.
  - The value is exact when the bounds meet, else `≥ L`.
- **Medians.** `lower_median` of the frozen scorer: the value at index `(n − 1) // 2`, censored if any censored value is at or
  below it. Decomposable draws are excluded.
  - With two draws the lower median is the minimum. So "flat at 5" allows one draw per `t` to have grown, while "grows" needs
    both draws.
  - The number of draws with `d_F ≥ 6` is therefore reported next to every median.
- **P-ISO-1 (transfer).** Isogeny `P1` medians at `t = 1..8` against `P0`'s (3, 5, 5, 5, 5, 5, 5, 5):
  - **holds** if all eight are exact and equal;
  - **refuted** if any exact median differs;
  - **undecided** otherwise.
- **P-ISO-2 (the question)**, at `t = 9..13`. Per `t`:
  - **grows** if the median is exact and `> 5`, or is a censored bound `≥ 6` at a `t` whose remaining draws are certified
    inconsistent;
  - **below 5** if it is exact and `< 5`;
  - **flat at 5** if it is exact and equal to 5;
  - **undecided** otherwise.
  
  Over the range: "grows" if any `t` grows, else "below 5" if any `t` is below 5, else "flat at 5" if every `t` is flat, else
  "undecided".
- **§6 as written.** H-LFD, H-FFD and H-gap are applied to the isogeny `P1` ladder and reported. H-LFD will read "grows" from
  `t = 1` alone (3, then 5), as it did at `P0`. It does not answer the isogeny question; P-ISO-2 does.
- **Comparison with Kummer `P1` (descriptive only).** Per `t`, the isogeny median beside the Kummer median, with all draws
  listed. Two draws per cell give no test.
- **What a "flat at 5" reading would and would not mean** (E4.9, stated instead of adding a null arm):
  - P-ISO-2 cannot separate a Semaev-specific effect from an effect of the tower ring itself. The Kummer null at `P1` read the
    same as the Kummer tower on completed `N`.
  - "Flat at 5" is a solving-degree description. It is not an improvement over the index-calculus baseline. That would also
    need the relation cost, the auxiliary-curve search (`aux_curves_tried`) and the matched rho reference (AGENTS.md §5a), and
    none of those is measured here.

## 6. Jobs, resources and cost (E4.8)

Route: the RunPod reserved-pod route of draft R3. R3 itself must be frozen with its own edits (E3.1–E3.9) before R4 runs; R4
adds the runner's `iso` mode (`runpod/server.py`): the R4 binaries plus `repo.tar`, a `git archive` of `iso/BUNDLE_PATHS` at
the job's commit.

| job | stages | vCPU / memory | time limit | projected wall | projected cost |
|:--|:--|:--|:--|:--|:--|
| `iso-a` | `g1 gaux A A-big DA g3 cert cap-A` | ≥ 12 vCPU (12 rayon threads) / 56Gi | 172,800 s | 2–6 h (heuristic) | US$2–7 expected, US$50 at the limit |
| `iso-x-tT-kK-cC` | `gaux x:T:K:C:12800000000` (one per bracket step) | ≥ 12 / 56Gi | 90,000 s | cap 5: minutes; cap 6: 0.5–4 h (Kummer t = 12/13) | under US$5 each expected, about US$26 at the limit |
| recount | `recount.py` on this side | — | — | under 1 h | none |

- **Pricing** (derived). About US$0.065 per vCPU-hour on memory-optimised reserved CPU pods. That is the review's figure, from
  a 32 vCPU / 256 GB pod at US$2.08/h, so 16 vCPU / 128 GB costs about US$1.04/h.
- **Ceilings.** The `iso-a` limit exceeds the US$40 per-job ceiling at worst. Whether reserved-pod ceilings change is the owner's
  decision, asked separately. Until then `iso-a` runs under the US$40 stop of R3's cost guard (E3.2). A stop is reported as
  censored-by-cost, and its rows up to the stop are kept.

## 7. Records

Each job keeps:
- a resume checkpoint;
- a `bench.record/v1`;
- a director card and receipt with the pod id and `costPerHr`;
- the runner's `runner_request.json` with the verified SHA-256s, `repo.tar`'s SHA-256 and commit;
- the results tarball's SHA-256, `binary.sha256` and `host.txt`, including the cgroup `memory.max`.

S3 upload follows R3 and is blocked today, because this session has no working AWS credentials.

## 8. Not changed

Kummer rows, frozen binaries, §6 rules for other arms, seeds, draws and budgets.

## 9. Review edits → where they are

| edit | where |
|:--|:--|
| E4.1 executable commands, pinned set | `iso/entry_iso.sh`, `iso/pilot_mestre.sh`, `iso/PINS.json`, `iso/BUNDLE_PATHS`; runner `iso` mode |
| E4.2 G2, G3 via wrapper | §3; stage `g3`; `score_iso.py` G2 |
| E4.3 decomposability | §4; `iso/certify_m2.py`, `iso/selftest/` |
| E4.4 readings | §5; `iso/score_iso.py` |
| E4.5 scorer and bracket | §4 bracket order, §5; `score_iso.py --selftest` |
| E4.6 reproducible binary | §2; `gaux/aux_order_f13a690df0.patch` (`git apply`), patched-source hash, `gaux/TESTS_FROZEN.md`; the artifact-store upload of `705b151b…` waits on S3 |
| E4.7 on-host control, recount, panic | §3; stage `gaux` in every job; `iso/naive_count.c`, `iso/recount.py`; exit 3 on panic |
| E4.8 resources, cost, disclosure | §0, §6 |
| E4.9 (optional) null | §5, stated as a limitation instead |
