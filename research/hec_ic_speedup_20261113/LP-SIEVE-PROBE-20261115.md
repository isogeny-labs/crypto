# Reduced-base large-prime line sieve, feasibility probe, 2026-11-15 (not under a frozen protocol)

Follow-up (A) of `SIEVE-PROBE-20261114.md`, lead 1 of `LEADS-20261114.md`. This is orientation,
like the full-base probe: no pre-registered expectations, no `S` claim. Its job is to decide whether
the line sieve should become the relation source for track T1 (double large primes at genus 3/4),
and what a frozen slice for it would measure. Any protocol built on it must say that these numbers
were used to choose it.

- **Code:** `crates/hec-native-ic/src/lp_sieve.rs`, `examples/lp_sieve_probe.rs`.
- **Raw output:** `lp-sieve-probe-20261115/` (`probe.txt` fixed-budget runs, `target.txt`
  run-to-target, `summary.md`, `host.txt` with load, `binary.sha256`).
- **Unit:** field products (feature `count-ops`, S4's counter, PLAN §10): every 64×64 product of
  field elements, including each term of `Poly::mul`'s lazily reduced inner product. Deterministic.
  Wall clock is from a shared host at load average 55–83 and is descriptive only.

## Mechanism

Base `B` = the first `#B = κ·p^{1−1/g}` degree-1 places by `x` (genus 3: `κ·p^{2/3}`, genus 4:
`κ·p^{3/4}`, the GTTD double-large-prime balance up to κ). Every other degree-1 place is a large
prime. A relation is full, a one-partial (1P) or a two-partial (2P) by its number of distinct
large-prime `x`.

For a line `D = (u, v)` and `a_c = v + c·u`, `w_c = (f − a_c²)/u = W₀ − 2c·v − c²·u`, where
`W₀ = (f − v²)/u` is computed once per line. Only base points are bucketed, so a line costs
`O(#B)`, not `O(p)`. A `c` with `h` base hits is finished from the two top coefficients of `w_c`:

- `h = g + 1`: full.
- `h = g`: the last root is `x* = −[x^g]w_c − Σ rᵢ`: a few additions.
- `h = g − 1`: the last two roots solve `x² − S·x + P` (from `[x^g]w_c`, `[x^{g−1}]w_c` and the
  known roots): one Legendre symbol, and a square root when it is a residue.

The large prime's `y` is `a_c(x*)`. Every relation is `D + Σ (xᵢ, a_c(xᵢ)) = 0` with multiplicity,
so a root that lands in `B` at a point where `u(x) = 0` (missed by the bucket) still gives a correct
relation, classified by where its roots lie.

**Prior art.** The line sieve is Sarkar–Singh's (ePrint 2014/815, DCC 2017), who timed it against
Gaudry's decomposition in Magma. Vitse–Wallet (LATINCRYPT 2015) already add large-prime variants
to it and report about 3× on Diem/Kochinke relation search. Only the abstracts were read. So neither
the mechanism nor its large-prime use is new here; what this probe adds is a counted comparison
against this engine's walk at GTTD's 2LP balance. No novelty is claimed.

The walk arm is the engine's walk (`d_step_in = 4`, 64 step places drawn from `B`) and oracle,
with every split `u` classified the same way. That is the relation source a T1 2LP port of this
engine would use today. It is not GTTD's tuned code, and it has none of the Tier-2 per-trial levers.

## Results

### Fixed budget: cost per relation class (`summary.md`)

200,000 walk trials and 2,000 sieve lines per (row, κ, seed), 3 seeds, the same `D₁, D₂` for both
arms. Products per class charge the whole run to that class.

| g | p | κ | #B | walk products / 2P | sieve products / 2P | ratio | ratio per 1P |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 2039 | 0.5, 1 | 80, 161 | 13,424, 7,854 | 572, 349 | 23.5×, 22.5× | 17.6×, 15.8× |
| 3 | 8191 | 0.5, 1 | 203, 406 | 23,417, 13,039 | 706, 421 | 33.2×, 31.0× | 23.2×, 20.4× |
| 3 | 32749 | 0.5, 1 | 512, 1024 | 37,991, 20,404 | 1,047, 635 | 36.3×, 32.1× | 27.2×, 20.5× |
| 4 | 509 | 0.5, 1 | 54, 107 | 93,585, 48,844 | 6,599, 2,681 | 14.2×, 18.2× | 11.5×, 14.0× |
| 4 | 1021 | 0.5, 1 | 90, 181 | 113,231, 47,551 | 6,404, 2,110 | 17.7×, 22.5× | 16.5×, 16.1× |
| 4 | 2039 | 0.5, 1 | 152, 303 | 163,081, 57,784 | 6,361, 2,237 | 25.6×, 25.8× | 20.1×, 21.1× |

- Every sieve relation checked held in the Jacobian: the first 50 per class (full, 1P, 2P) per
  run, 4,550 relations here and 5,040 in the run-to-target set.
- The class mix differs a little. The sieve's partials lean further toward 2P (for example
  1:43:956 against 1:65:934 per thousand at (3, 32749), κ = 1), because its candidates come from
  base hits and a quadratic finish.
- **Partials are equally useful.** At an equal partial count, the large-prime graphs close the same
  number of cycles on 10 of the 12 rows (to within 4%). At (3, 32749), below the graph's giant
  component, the walk's larger 1P share gives it more cycles: 3 and 18 against 1 and 5. The
  run-to-target figures below include this deficit.

### Run to target (`summary-target.md`)

Each arm runs from the same `D₁, D₂` until fulls + independent graph cycles reach `#B + 10`
(GTTD's double-large-prime relation count). Medians over 3 seeds; the ratio is of medians, with
the per-seed min–max.

| g | p | κ | #B | walk products | sieve products | **ratio** | partials walk / sieve | clock ratio |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | 2039 | 0.5 | 80 | 1.00e7 | 4.41e5 | **22.7×** (20.2–23.6) | 797 / 836 | 23× |
| 3 | 2039 | 1 | 161 | 5.91e6 | 2.82e5 | **21.0×** (20.2–23.0) | 857 / 899 | 25× |
| 3 | 8191 | 0.5 | 203 | 6.74e7 | 2.04e6 | **33.1×** (31.7–33.3) | 3016 / 3014 | 33× |
| 3 | 8191 | 1 | 406 | 3.83e7 | 1.26e6 | **30.3×** (29.3–30.3) | 3208 / 3271 | 27× |
| 3 | 32749 | 0.5 | 512 | 4.17e8 | 1.18e7 | **35.5×** (35.2–35.5) | 11331 / 11581 | 34× |
| 3 | 32749 | 1 | 1024 | 2.38e8 | 7.60e6 | **31.3×** (31.1–31.7) | 12253 / 12449 | 29× |
| 3 | 131071 | 0.5 | 1290 | 2.84e9 | 6.17e7 | **46.0×** (46.0–46.6) | 43740 / 44191 | 38× |
| 3 | 131071 | 1 | 2580 | 1.54e9 | 3.60e7 | **42.9×** (42.6–43.3) | 46690 / 47216 | 38× |
| 4 | 509 | 0.5 | 54 | 1.69e7 | 1.23e6 | **13.7×** (13.2–13.7) | 219 / 229 | 12× |
| 4 | 509 | 1 | 107 | 6.76e6 | 4.45e5 | **15.2×** (14.7–16.4) | 215 / 223 | 11× |
| 4 | 1021 | 0.5 | 90 | 4.26e7 | 2.54e6 | **16.8×** (16.1–17.0) | 440 / 459 | 15× |
| 4 | 1021 | 1 | 181 | 1.59e7 | 8.21e5 | **19.4×** (18.2–20.7) | 469 / 483 | 14× |
| 4 | 2039 | 0.5 | 152 | 1.28e8 | 5.24e6 | **24.5×** (22.4–25.3) | 876 / 891 | 20× |
| 4 | 2039 | 1 | 303 | 4.24e7 | 1.87e6 | **22.7×** (22.6–24.6) | 950 / 966 | 17× |
| 4 | 4093 | 0.5 | 256 | 3.81e8 | 1.32e7 | **29.0×** (27.7–29.5) | 1752 / 1726 | 24× |
| 4 | 4093 | 1 | 512 | 1.20e8 | 4.49e6 | **26.7×** (25.5–28.1) | 1854 / 1881 | 21× |

The clock agrees with the count: 11–38× against 13.7–46.0×. That answers the worry that a
`Poly::mul` inner term is cheaper than a full product. The clock ratios are a little lower at
genus 4, where the sieve's per-line Cantor addition and its bucket traffic are relatively heavier.

At (3, 32749) with `#B = 512`, the walk needed about 736,000 trials to reach the target. GTTD report
815,000 trials for genus 3, `q ≈ 2^15`, `#B = 512`. That checks the trial count only, not cost; no
comparison to their published cost is made.

### Why the ratio grows with p, and what it is not

- **A constant factor, not an exponent.** Both arms' cost per partial scales like `p/#B`. The walk
  pays one Jacobian step and one oracle call per trial and needs about `p/#B` trials per 2P. The
  sieve pays about 15–18 products per base point per line (`summary.md`), and a line yields about `#B²/p` partials.
  The asymptotic cost of 2LP index calculus is unchanged.
- **The growth with p is the walk's oracle.** The walk's products per trial rise from 456 to 567
  across genus 3, `p = 2039 … 32749` (`x^p mod u` grows with `log p`), while the sieve's cost per
  base point is flat except for its Legendre symbols. The trend is reported, not extrapolated.
- **The walk is untuned.** The Tier-2 levers of `LEADS-20261114.md` (fixed-degree oracle with root
  reuse, closed-form discriminant, one-inversion Cantor) might make a walk trial about 2× cheaper.
  That would leave roughly 7–23×.
- **κ is not bracketed.** Both arms get cheaper from κ = 0.5 to κ = 1 at most rows, and the linear
  algebra, which grows with κ and with the sieve's row weight `g + 1`, is not in these counts.

### End to end: do the relations give `k`? (`summary-solve.md`)

"Reached target" counts relations; it does not show that they solve. Sieve rows from one line share
that line's `(α, β)`, a structure the walk's rows lack, so cycle independence had to be checked.
`examples/lp_sieve_solve.rs` does that on prime-order instances (`pick_instance_with_budget`,
planted `k = ⌊ℓ/3⌋ + 7`), κ = 1, 3 seeds, on (3, 2039), (3, 8191) and (4, 509). (4, 1021) was
also tried and dropped: no usable instance within the 2,000,000-step BSGS budget.

- each arm runs to `⌈μ·(#B + 10)⌉` fulls + cycles, keeping its rows;
- rows holding a large prime seen only once are dropped, repeatedly;
- each remaining large prime becomes a column, and `Solver::Fast` solves for `k`, which is checked
  against `k·D₁ = D₂`;
- μ = 1, 1.1, 1.25, 1.5, 2 is tried in turn until it works.

| g | p | ℓ | #B | k recovered (seeds) | μ needed walk / sieve | relation products walk / sieve (ratio) | matrix rows × cols walk / sieve | solve mulmods walk / sieve | relation + solve walk / sieve (ratio) |
|--:|--:|--:|--:|:-:|--:|--:|--:|--:|--:|
| 3 | 2039 | 4248123137 | 161 | 6/6 | 1 / 1 | 5.3e+06 / 2.6e+05 (20.4×) | 530 × 521 / 578 × 568 | 2.09e+05 / 1.85e+05 | 5.51e+06 / 4.45e+05 (12.4×) |
| 3 | 8191 | 274956898141 | 406 | 6/6 | 1 / 1 | 3.56e+07 / 1.29e+06 (27.5×) | 1620 × 1612 / 1721 × 1711 | 1.2e+06 / 2.87e+06 | 3.68e+07 / 4.17e+06 (8.8×) |
| 4 | 509 | 61914322373 | 107 | 6/6 | 1 / 1 | 6.21e+06 / 3.67e+05 (16.9×) | 214 × 204 / 222 × 212 | 1.37e+05 / 4.98e+04 | 6.35e+06 / 4.17e+05 (15.2×) |

- **`k` was recovered at μ = 1 by every run of both arms.** The same-line structure costs the sieve
  no extra relations at these sizes.
- **The solve is now the larger share of the sieve's cost.** The sieve's pruned matrices are 4–9%
  larger (its lower 1P share leaves more large-prime columns). With the plain sparse solver at
  (3, 8191) its solve takes 2.87M mulmods against the walk's 1.20M, so the end-to-end advantage
  there is 8.8×, down from 27.5× in the relation phase. A solver for these rows (follow-up B) and a
  κ chosen for the combined cost are the levers.

With `Solver::SgeBw` (follow-up B, `sieve-solve-20261115/`) instead of `Solver::Fast` on the same
matrices (`solve-sgebw.txt`, `summary-solve-sgebw.md`), every run still recovers `k` (one walk seed
at (4, 509) needed μ = 1.1), but the solve costs more, not less: 5.91M against 2.87M mulmods for the
sieve at (3, 8191), 0.80M against 0.19M at (3, 2039). At these sizes SGE leaves too large a
residual for Wiedemann to pay off, so B does not yet relieve the 2LP solve.

## What this decides

1. **The relation source for T1 (2LP at genus 3/4) should be the line sieve, not the walk.** To GTTD's
   relation count it needed 21–46× (genus 3) and 14–29× (genus 4) fewer counted field products than
   this engine's walk, the clock agreeing, and its relations solved for `k` with no extra margin.
   This is a constant factor against an untuned walk; prior work (Sarkar–Singh; Vitse–Wallet for
   large primes) has the mechanism.
2. **The linear algebra becomes the bottleneck sooner.** With relation collection this cheap, the
   2LP solve dominates the sieve's cost from p ≈ 2^13 at genus 3. T3 (SGE, then block Wiedemann) and
   κ balance (T4) move up.
3. **Next protocol.** A frozen slice for the large-prime sieve: rows, κ grid including κ = 2 (to
   bracket the optimum), run-to-`k` end to end as the gate, relation-phase and solve products as the
   primary statistics, and the walk with the cheapest Tier-2 levers as the comparator.

## Limits

- Not under protocol; these numbers chose the next protocol and must be declared there.
- The walk arm is this engine's untuned walk. A 2× faster walk would leave roughly 7–23×.
- Wall clock is from a shared host at load average 55–215, descriptive only.
- Full texts of Sarkar–Singh and Vitse–Wallet were not read; nothing here claims novelty.
- The large-prime graph counts cycles as GTTD do; the end-to-end check covers three rows only, at
  κ = 1.
