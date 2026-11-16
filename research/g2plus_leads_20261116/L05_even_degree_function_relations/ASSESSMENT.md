# L05 assessment (independent assessor, 2026-11-16) `[pcc:unverified]`

Everything here is `[pcc:unverified]`. One producer, one replay on the same
host, and this assessor. I did not produce the data. This file is the only one
I wrote in the lead directory. My scratch scripts are outside it:
`/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/rederive.py`
and `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/indep.py`.

## Verdict

| hypothesis | frozen rule (PROTOCOL section 8) | verdict | what it means |
|---|---|---|---|
| H1 (primary): E_0 on the even model costs at most 2.0x O on the odd model of the same curve | all gates, PC, G7 pass, `GM(R1) <= 2.0` in all 8 cells | **supported** | parity: GM(R1) is 0.995 to 1.022. Not an improvement. |
| H2: E_1a_T at least 1.2x cheaper than O_T | `GM(R2) >= 1.2` in all 8 cells | **supported under the frozen rule; the effect comes from the baseline's construction** | see "Baseline audit". With a non-wasteful O_T, R2 is about 1.00. |
| H3: rootless E_0 at most 2.0x root O | `R4 <= 2.0` in all 8 cells | **supported** | R4 is 0.989 to 1.057 |

Overall lead verdict: **supported (H1)**. The result is a capability-parity
result. The function-relation sieve works on even-degree (two points at infinity)
models, with correct `inf+-` bookkeeping, at the same relation-phase FPM per
relation as the odd-model sieve. It is **not** a measured improvement over the
index-calculus baseline. H2's 1.4x (g = 2) and 1.9x (g = 3) must not be cited as a
speed-up. The frozen O_T baseline computes `Pi(x)^-1` as a product of `g`
table inverses, which costs `g-1` multiplications per x. A free table lookup
`inv[Pi(x)]` was available. When O_T is priced with that lookup, the gain
disappears (re-derivation below).

## 1. Integrity checks

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
- `raw/unittest.log` ends with `Ran 16 tests ... OK`. `raw/COMMANDS.log` has 553
  lines and every command line has rc=0. No raw file contains a
  `"type": "failure"` record.
- Gate files (I re-read all 5). `gate_summary.pass` is true in every file.
  `J_even == J_odd` holds on all 15 gate curves. V2 failures are 0 across
  811 / 1082 / 1646 / 168 / 238 relations (g2 p101 / p151 / p199, g3 p37 / p53).
  m1 and m2 accept 0 of 616 / 822 / 1249 / 106 / 144. At g = 3 the gate is thin:
  several arms stopped at `pencils_exhausted` (DEVIATIONS 4). SPEC 9 defines a
  pass by the verification counters only, so the gate passes as written. The
  g = 3 even-model bookkeeping still rests on 274 V2-checked relations at gate
  size, plus the sampled V2 checks in the relation phase.
- The replay (separate summary) reproduced all 552 files after timing fields
  were normalised. `analysis.json` and `summary.md` are byte-identical.

## 2. Instrument audit

- **Counters.** `l05/field.py` counts every `mul`, `inv` and add through one `F`
  object per arm. Squarings go through `mul`. Table reads are free (SPEC 4.2).
  `sieve_pencil` asserts the SPEC 7.5 count formula on every pencil, and no
  assertion fired. I re-summed the per-pencil `mul`/`inv` (plus `table_mul`) for
  all 336 root runs (8 cells, 6 pairs, 7 arms). They match the run record in
  every case: 0 mismatches. `relations` equals the sum of `new`, is at least
  `n_target`, and `fpm_per_rel` equals `(mul + 10 inv) / relations`.
- **Self-consistency caveat.** The G4 count formula comes from the same SPEC as
  the code, so it shows that the code matches SPEC, not that the operations are
  minimal. That limit is what lets the O_T issue below pass every gate.
- **Independence of verification.** V1 (affine identity), V_inf (Laurent series
  of `sqrt f` at infinity) and V2 (Cantor on the odd model after the explicit
  birational map) run on uncounted paths. V2 uses a different model and the group
  law, and the mutations m1 (a unit shift between `inf+` and `inf-`) and m2 (one
  sign flip) are always rejected. A bookkeeping bug could not produce
  "all V2 pass and all mutations rejected".
- **My independent math spot-check** (`indep.py`, own arithmetic, no `l05`
  import). For `f1 = random_f_even(101,2,1)`:
  - Brute-force point counts give N1 = 114, N2 = 10366 and #J = 11580. These match
    SPEC 12.7.
  - My own E_g census gives 17 relations, which matches SPEC 12.9.
  - For 3 independently drawn E_0-type relations (monic cubic `a` with the
    `x^2` coefficient different from `s`'s), I checked them on my own
    quintic model `v^2 = sum c_i u^(6-i)`, with `u = 1/(x-alpha)` and `v = y u^3`.
    The zero set of `v - b(u)` is exactly the 5 mapped points plus `(0, +1)`,
    the image of `inf+`. So `div(y - a) = sum P_k - 2 inf+ - 3 inf-`, which is
    the `(ip, im) = (-g, -(g+1))` that SPEC 7.1 assigns.
- **Fairness of the H1 pairing.** The pairing is fair. Both arms use the same
  curve, `J`, `p`, `N_target` rule, column count (`n_odd = n_even + 1` by
  construction), `sieve_pencil` routine, batch inversion recipe and verification
  policy, with setup excluded from both. R2b (O against E_1a, batch) is 0.997 to
  1.008, so the instrument gives the same price to linear systems that should
  cost the same.
- **Positive control.** GM(R3) is 3.986 to 4.004 at g = 2 and 4.864 to 5.000 at
  g = 3. All 8 cells are inside `[0.8(g+2), 1.25(g+2)]`, so the instrument sees a
  known effect of size `g+2`. G7 passes in all 8 cells (pooled 0.965 to 1.116,
  per the executor; consistent with `rels_per_pencil_norm` in the records I read).

## 3. Baseline audit: O_T is needlessly expensive (decisive for H2)

SPEC 7.5 step 4 (table mode) computes `di = prod_i inv_tab[x - x_i]`, which
costs `k(j-1)` multiplications per pencil: `g-1` per x. It also does not keep a
forward-difference evaluator for `Pi`. The batch arms already evaluate `Pi(x)`
by forward differences at a one-off cost of `j(j+1)` multiplications per pencil,
with additions only after that. With the inverse table in hand,
`di = inv_tab[Pi(x)]` is a free table read (`Pi(x) != 0` off the anchors). This
counterfactual O_T' replaces `k(j-1)` with `j(j+1)` per pencil. E_1a_T needs
no change, because its `j - 1 = 0`. I re-derived it from the raw pencil records
(`rederive.py`):

| g | p | R2 frozen GM [min,max] | R2 with O_T' (FPM) GM [min,max] | O_T' with additions priced |
|---|---|---|---|---|
| 2 | 1031 | 1.401 [1.371,1.436] | 1.005 [0.983,1.029] | 0.927 |
| 2 | 4099 | 1.380 [1.337,1.410] | 0.986 [0.955,1.008] | 0.928 |
| 2 | 16411 | 1.395 [1.389,1.401] | 0.996 [0.992,1.001] | 0.923 |
| 2 | 65537 | 1.399 [1.391,1.404] | 0.999 [0.994,1.003] | 0.925 |
| 3 | 1031 | 1.907 [1.743,2.022] | 1.009 [0.925,1.068] | 1.055 |
| 3 | 4099 | 1.921 [1.898,1.947] | 1.003 [0.991,1.017] | 1.062 |
| 3 | 16411 | 1.931 [1.910,1.953] | 1.003 [0.992,1.014] | 1.069 |
| 3 | 65537 | 1.922 [1.917,1.934] | 0.998 [0.994,1.003] | 1.062 |

So the H2 effect is entirely the extra multiplications in the baseline's
inversion recipe. This is a counterfactual computed from counts, not a measured
arm. It is exact for the multiplication count, because the removed and added
multiplications are explicit in SPEC 7.5. The verdict under the frozen rule
stays "supported", because the rule is applied as written and no deviation
touches it. Under the owner's success criterion, though, it is an accounting
artefact and not an improvement. PROTOCOL section 1 (correction 5) already said
that any H2 gain would belong to the linear system, not to the even model.

## 4. Re-derived headline numbers (from `raw/`, my own script)

GM over 6 pairs (3 root curves x solver seeds 1 and 2), with [min, max]:

- **R1 = E_0 / O.** g = 2: 1.022 [1.002,1.054], 0.996 [0.979,1.014],
  1.004 [0.999,1.008], 1.001 [0.998,1.004]. g = 3: 1.007 [0.948,1.105],
  1.007 [0.996,1.016], 0.995 [0.985,1.006], 1.004 [0.999,1.009]
  (p = 1031, 4099, 16411, 65537). These are identical to `raw/summary.md`.
  The largest cell value is 1.022, against a threshold of 2.0, so H1 has about
  2x headroom in every cell. The seed and curve spread is at most ±5% at
  p = 1031 and under ±1% at p >= 16411.
- **R4.** 1.047, 0.989, 1.003, 1.007 (g = 2) and 1.057, 1.017, 0.995, 1.002 (g = 3).
- **FPM per relation for O** (GM): about 15.0 to 15.2 at g = 2 and 57.8 to 61.0 at
  g = 3. This matches the predicted `2.5 (g+1)!`.

## 5. Deviations

DEVIATIONS 1 to 6 do not change any count, arm, seed, threshold or decision
branch actually taken. The analyze.py precedence choices (2) were not
triggered. The thin g = 3 gate (4) follows the SPEC 9 pass definition. None of
them invalidates the frozen rule.

## 6. Distance to matched rho

Not applicable / unchanged. No matched rho reference was run, and none is
allowed for this pass. The lead measures the relation phase only, with no
linear algebra and no descent. On an even model, descent also needs
balanced-divisor arithmetic, which was not measured. Parity in the relation
phase (R1 about 1.00) means the distance to rho for any g >= 2 IC row is exactly
what it was with the odd-model sieve. The result widens **applicability**: it
covers curves with no rational Weierstrass point (H3, rootless sextics and
octics), which the lab's odd-only instruments reject. It does not reduce cost.

## 7. Concerns

1. H2 "supported" is a baseline artefact (section 3) and must not be cited as an
   improvement.
2. The G4 count assertion is tautological with respect to SPEC. It cannot catch a
   wasteful but specified recipe.
3. The g = 3 gate is thin (168 and 238 relations, several `pencils_exhausted`).
4. Rootless curves get no V2 (no odd model over F_p). Their correctness rests on
   V1, V_inf and the root-curve V2 evidence for the same code path.
5. The `delta` column's effect on linear algebra (dense column, H4) is unmeasured.
6. These are toy primes, Python counts and FPM at I = 10. There is one host, and
   the replay ran on that same host.

## 8. Single most valuable next step

Use the even-model support where it adds reach, not speed. Plug the E_0 sieve
into the full g = 2 pipeline (relations, then sparse linear algebra with the
`delta` column, then descent through balanced-divisor arithmetic) on rootless
sextics, where no odd model exists. Measure the full DLP cost against a
matched rho with the §5a band. Separately, if anyone wants to pursue the "fewer
anchors" linear system (E_1a), first fix O_T to use `inv_tab[Pi(x)]` and rerun
under a new frozen protocol. The counterfactual here predicts R2 of about 1.0,
so this is low priority.
