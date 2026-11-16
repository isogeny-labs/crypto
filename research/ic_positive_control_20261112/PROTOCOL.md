# IC positive control: can the instrument report a win? (frozen protocol, 2026-11-12)

Owner-approved step 1 of `../chained_prime_transfer_20261111/NEXT_STEPS_20261112.md`.
This is a **calibration of the instrument**, not a result about the transfer. Every
numeric statement below is `[pcc:unverified]`.

Written-order note (disclosed): the code (`ic_control.rs`, `ic_positive_control.rs`)
and its unit tests existed before this file. No measured datum existed. The smoke
runs of section 8 were made before the freeze, are labelled smoke, and are **not**
used as data in `RESULTS.md`.

## 1. Question

Every index-calculus (IC) row of the chained-transfer program is a loss to Pollard
rho, and nothing shows the lab's instrument could report a win. Genus-3 IC on a
**full** Jacobian `J(F_p)` (`#J = h r`, `r` prime, `r` close to `p^3`) is the regime
where IC is known to cost less than rho: double-large-prime IC
(Gaudry-Thome-Theriault-Diem) is `O~(p^{2-2/g}) = O~(p^{4/3})` for `g = 3`, rho on the
order-`r` subgroup is `~sqrt(r) = p^{3/2}`. Basic full-factor-base IC is `O~(p^2)` with
linear algebra and is expected to lose, so the positive control implements the
large-prime variant (arm C). If the instrument, run honestly across the AGENTS.md
section 5a band, cannot show C below rho at any `p` it can reach, then it is too weak,
and every earlier "distance to rho" overstates the gap. If it can, the earlier
losses are informative.

## 2. Curves

For each prime `p` of the list (section 6): random monic `y^2 = f(x)`, `deg f = 7`,
coefficients drawn from SplitMix64 of `(curve_seed, p)`, `f` squarefree
(`H64Curve::new`), `curve_seed` counting up from `curve_seed0 = 1`. `#J` is computed
exactly: `N_1`, `N_2` by enumeration, then baby-step giant-step over the Hasse-Weil
window for the last `L`-polynomial coefficient with a random divisor, every
candidate checked on 32 random divisors, exactly one must survive. A curve is
**admitted** iff `#J = h r` with `r` prime, `r > h`, `h <= 64`. The first `K`
admitted curves per `p` are used (K in section 6). `G = [h] D` for a random `D`
(checked order `r`), `T = [t] G` with `t` uniform in `[1, r)`, `T` computed by
`mul_rtl`. Every curve is a full Jacobian: no cover, no transfer, no endomorphism.

## 3. Arms (all counted on one group, `H64Curve`, Cantor arithmetic)

Group operations are `CountedGroup` adds and doublings. **Native operations** are
modular multiplications ("mul") and modular additions/subtractions ("add") on
residues mod `p` (relation phase) or mod `r` (linear algebra); a mulmod by `+-1` is
an add. They are priced at `w` GAE per operation, `w in {0, 0.1, 1}` (AGENTS.md 5a),
the same way on every arm. **Primary reading: mul only** (the repository
convention, `linalg_mulmods`); **sensitivity: mul + add**, reported in
`control.json` (`native_all`).

Common relation source (A, B, C): an r-adding walk with 16 jumps
`[a_j]G + [b_j]T` (`a_j, b_j < 2^12`, set-up counted), restarted from a random
`[a]G + [b]T` every 4096 steps (counted). Every step is one group addition and one
trial. A trial is a reduced divisor `R = [a]G + [b]T`; it is a candidate iff
`deg u = 3`. Smoothness: `u | x^p - x` (square-and-multiply on `F_p[x]/u`, counted);
roots by Cantor-Zassenhaus (counted); `y = v(x)`; a place is `+-` its canonical
column `(x, min(y, p-y))`; places with `y = 0` contribute 0. A relation reads
`sum eps_i lambda_i - (h b) t = h a (mod r)` where `lambda_i` is the log of
`[h] P_i` (cofactor `h` projects the class onto the order-`r` subgroup).

- **A (full):** factor base = every degree-1 place (up to sign), `~p/2` columns;
  relations = fully smooth `u`.
- **B (single large prime):** factor base = the first `nb_B = ceil(c_B p^{3/4})`
  columns by `x`; relations with at most one place outside it (a large prime).
- **C (double large prime):** factor base = the first `nb_C = ceil(c_C p^{2/3})`
  columns by `x`; relations with at most two places outside it; the large primes
  are vertices of a graph, a relation is an edge (or an edge to the vertex "1"); the
  independent cycles of the component of "1" are the full relations.
- **R (rho):** `rho_reference_walk(..., RhoWalk::negation())` on the order-`r`
  subgroup (the repository's matched reference for `A = 2`). Group ops = all
  charged operations; walk steps (`walk_operations`) reported apart from set-up
  (`total - walk`). Native work = one canonicalisation per step
  (`canonicalisations_uncharged`, never below the step count). Rho's native work is
  small; it is priced anyway.

Shared pipeline after collection: cycle rows (large-prime columns eliminated along a
breadth-first spanning tree), peeling of columns of weight one, then Wiedemann on
`A^T D A` with a random diagonal `D` (Berlekamp-Massey, one reconstruction pass,
residual check `A x = c`); retried on failure with more relations. Linear algebra
counters: `la_graph` (tree/cycle row construction), `la_peel`, `la_wied`. Every
mulmod and add/sub mod `r` is counted, including the matrix-vector products (an
entry of `+-1` is an add, any other entry a mul). Union-find edge operations are
added to `la.add`. The stopping rule is checked incrementally (cycles of the
component of "1" for B and C, a peeling check for A); only the final build, peel and
solve are counted, which assumes the incremental bookkeeping costs no more than the
counted union-find operations.

**Certificate (every log, every arm):** `[t_found] G == T` by the counted
`CountedGroup::mul` (charged to the arm) **and** by the independent right-to-left
`mul_rtl` (not charged). A run whose log fails either is reported as failed and sets
the exit status.

## 4. What is not claimed

No transfer, no cover, no precomputation. IC arms and rho are not run to the same
`p` where the cost is prohibitive (A is capped, section 6). The native counters are
operation counts, not a time; a measured ns/op calibration is recorded as an
informational column only (`calibration_ns_informational`) and never replaces the
band. Relation-phase additions/subtractions in the smoothness test are counted.
Memory (bytes) is listed, not priced (AGENTS.md 5a).

## 5. Parameters frozen here

| Parameter | Value |
|---|---|
| Factor-base rule B | `nb_B = ceil(2.0 p^{3/4})` columns (`c_B = 2.0`) |
| Factor-base rule C | `nb_C = ceil(2.0 p^{2/3})` columns (`c_C = 2.0`) |
| Factor base A | every degree-1 place up to sign (`~p/2`) |
| Cofactor cap | `h <= 64`, `r` prime, `r > h` |
| Walk | 16 jumps, `a_j, b_j < 2^12`, restart every 4096 steps |
| Margins | solve is tried when the core has at least `cols + 8 + cols/100` rows (A, after peeling) or the component of "1" has at least `N + 8 + N/100` independent cycles (B, C), `N = nb + 1` |
| Wiedemann | `A^T D A`, random `D`; sparse 16-entry projection on the first attempt, dense on retries; sequence length `2 cols + 2` |
| Rho | `rho_reference_walk`, `RhoWalk::negation()`, `max_steps = 60 sqrt(r) + 10^4` |
| Prices | `w in {0, 0.1, 1}` GAE per native multiplication (primary); mul+add as sensitivity |
| Informational | timing calibration `w_rel = ns(mulmod mod p)/ns(group add)`, `w_la = ns(mulmod mod r)/ns(group add)`, one value per `p`, best of 5 (a further point beside the band, never replacing it) |
| Fit | least squares of `ln(mean cost)` on `ln p` over `p >= 500` |

`c_B = c_C = 2.0` was chosen on smoke runs (section 8): at `p = 8209`, `c in
{1, 2, 4, 8}` for B and C, `c = 2` gave the lowest total at `w = 0.1` and `w = 1`
for both (at `w = 0` a larger `c` is cheaper in group operations, because the linear
algebra is free there). Fixing one `c` per arm for the whole run, not per price, is
part of the freeze.

The sources that produced the data are hashed in `frozen.sha256` (this file, the
module, the binary source) before the first measured run.

## 6. Run design

Primes: the first prime at or above `2^7, 2^8, ..., 2^16`, that is
`131, 257, 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537`. Admitted curves
per prime (`K`): 5 for `p <= 2053`, 3 for `4099` and `8209`, 2 for `16411` and
`32771`, 1 for `65537` (the `O(p^2)` point count bounds the number of curves
affordable there). Seeds per curve and arm: 4 (derived `1000 (s+1) + curve_index`).
`curve_seed0 = 1`. Arm A is run up to `p = 32771` (`--full-cap 32771`): the dense
Wiedemann core at `65537` is the only unaffordable job; this is a deviation from
"all arms at all `p`" and is reported. B, C and R run at every `p`. Runs execute
three jobs at a time on a shared 4-core host; counts are deterministic given the
seed, wall times are not used in any verdict.

Smoke runs of section 8 used `curve_seed0 = 5001` (different curves) and are
excluded.

The reported cost of an arm at `p` is the **mean over all (curve, seed) runs** of
group operations, native multiplications, and `total(w)`. All runs are listed in
`control.json` (`rows`). A run whose log is not certified marks `all_certified =
false` and fails the run.

## 7. Frozen prediction and decision rule

**Decision rule (frozen).** The instrument *can detect a win* iff there is a prime
`p` in the run at which the mean total cost of C is below the mean total cost of R
at **all three** prices `w = 0, 0.1, 1` simultaneously, with every log of both arms
certified. A weaker reading (for each price separately, some `p`) is reported
beside it. If the rule fails, the verdict is "the instrument as priced by AGENTS.md
5a cannot show a win in the regime where IC is known to win, at the largest `p`
reached", and the calibrated column says whether that is a pricing effect or a
cost effect. The calibrated reading never changes the verdict.

**Prediction (frozen, from smoke runs at `p = 257 ... 16411`, which are not data).**

| # | Statement | Falsified if |
|---|---|---|
| P1 | At `w = 0`, C is below R at every `p >= 257` of the run | any `p >= 257` with C >= R |
| P2 | At `w = 0.1`, C/R > 1 at every `p`; C/R at `p = 16411` is in [8, 40] and C/R at `p = 65537` is smaller than at `16411` | C/R <= 1 anywhere, or ratio outside [8, 40], or not decreasing |
| P3 | At `w = 1`, C/R > 1 at every `p`; C/R at `p = 16411` is in [40, 250] | C/R <= 1 anywhere, or outside the interval |
| P4 | Calibrated column: C/R at `p = 16411` is in [0.7, 2.0] | outside the interval |
| P5 | The decision rule fails (no `p` with C below R at all three prices) | any such `p` |
| P6 | Exponents, fit over `p >= 500`: C group ops 1.15 (range 1.0 to 1.4); C total at `w = 1` 1.4 (1.25 to 1.55); R total at `w = 1` 1.45 (1.35 to 1.6); A total at `w = 0.1` 2.0 (1.8 to 2.15); A group ops 1.0 (0.95 to 1.1) | outside the stated range |
| P7 | Fitted `w = 0.1` crossover of C and R (extrapolated from the two fits) is above `10^6` or does not exist | below `10^6` |
| P8 | Every arm's every log certifies | any failure |

Why these are the expected numbers: the smoothness test (`x^p mod u`, about 270
multiplications per trial at `p ~ 10^4`) costs far more native operations than one
Cantor operation costs group operations when a native multiplication is priced at
`w = 0.1` or `1`; C trims linear algebra, not the per-trial test. The asymptotic
gap between `p^{4/3}` and `p^{3/2}` is `p^{1/6}`, a factor of about 5 at `p = 10^4`,
well below that per-trial constant.

**Prices on every arm (AGENTS.md 5a).** Unpriced counters listed per arm in
`control.json`: IC `rel_mul`, `rel_add`, `la_mul`, `la_add` (with `la_graph`,
`la_peel`, `la_wied` splits); rho `native_mul` (canonicalisations, at least one per
step). Not priced, listed: memory (relation store, row store, rho table), the
factor-base column map, the DSU parent array. These are bytes, which the band does
not price.

## 8. Smoke tests (not data)

Before this freeze the binary was run at `p = 257, 521` (1 curve, seed 1000),
`p = 1031, 2053, 4099` (1 curve), `p = 8209` (`curve_seed0 = 5001`, arms B and C,
`c in {1, 2, 4, 8}`) and `p = 16411` (`curve_seed0 = 5001`, one run of R, B, C). They
checked that every log certifies, that `splits/trials` is near `1/6`, that the
cycle and peeling bookkeeping stops the collection, and they set `c_B`, `c_C` and the
predictions above. They are not used as data in `RESULTS.md`; the data runs repeat
those primes with `curve_seed0 = 1`.

Unit tests (`cargo test --release --lib chained_transfer::ic_control`): `mulmod`
equals the `u128` product; roots and smoothness of known cubics; `#J` kills random
divisors, `rG = 0`, `[t]G = T`, and for A, B, C every accepted relation equals the
sum of its places in the group (`verify_relations`), the logs certify and match the
planted secret, and B and C each produce at least one large-prime relation; the rho
log matches the IC log on a small instance.
