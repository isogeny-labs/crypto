# L4 protocol: missing baselines (a) gcd with the subspace polynomial, (b) FES, (c) polynomial method

Lane L4 of `research/ecc2k130_open7_20261114/` (rules: `../COMMON.md`, including
Amendment 1: Rust only). Frozen before any measured run, 2026-11-14.
Every claim in this lane is `[pcc:unverified]`.

## 0. Prior-art audit (done before this file was frozen)

research-kb and the lead atlas are unavailable in this container; the audit is
a repository grep, so no novelty is claimed.

* **(a) "fix m-1 factor-base points, gcd with L_V"** is implemented:
  `crypto:src/cryptanalysis/semaev_decomp.rs` (`subspace_poly`,
  `roots_in_subspace`, `roots_in_subspace_quartic`, the S4 pairs-and-solve
  `decompose`; module doc: "gcd(quartic, L_V mod quartic) then has exactly the
  subspace roots as its roots"). It is the `baseline` arm of
  `crypto:research/pdp_root_finder_20261012` (timed in ns only, *not*
  counted), the label certifier of `crypto:docs/ic/corpus/n15l5`, `n19l6`, and
  the comparator of the MITM ladder (`docs/ic/factor-base-encyclopedia/README.md`
  lines 1144-1160: 8,390,656 quartic solves vs <=16,092 point additions at
  l = 12, zero disagreements). The head-to-heads of "root finding" against
  `inherited-f4` (`research/ecbench_rootfind_engine_20261012`,
  `research/semaev_root_vs_groebner_20261012`, `pdp_root_finder` (23,8) row) use
  the **pair table / group law** (`PairSumTable`, `rf-mitm`), not the gcd with
  `L_V`. **Residual:** no counted (field-op vs word-XOR), same-instance
  head-to-head of the literal gcd-with-`L_V` oracle against the Koblitz
  descent default `inherited-f4` and `fes-f2`, and no crossover in (n, l, m).
* **(b) libFES-style Gray-code FES** is implemented (`crypto:src/cryptanalysis/mq_fes.rs`,
  `cryptanalysis:suite/src/cryptanalysis/mq_fes.rs`, ic_framework `fes-f2`,
  `fes-f2-wide`) and measured head-to-head against `inherited-f4` (the Koblitz
  oracle default, `ic_framework/solvers.rs` describe string) on identical
  seeded targets: `crypto:research/inherited_f4_next_20261112` Run 1 (14-32
  unknowns, word XORs and wall, 0 disagreements) and Run 2 (end-to-end S),
  `crypto:research/fes_then_batched_20261112` (end-to-end S, n = 13..23);
  validated Iso Lab run frozen but blocked by budget
  (`crypto:research/fes_k13_validated_20261112`). Counted bit operations FES vs
  Boolean F4 on descended S3 systems: `crypto:research/blindspots_20261018/03-polynomial-method`.
  **CLOSED-BY-PRIOR.** FES is re-used here only as an arm (its counts come from
  the repository engine).
* **(c) polynomial method**: `crypto:research/blindspots_20261018/03-polynomial-method`
  implements Dinur's MultParityCount (arXiv 2005.04800, Alg. 2) and the
  single-level LPTWY scheme, real calls at N = 12, 16 (S3) and 13 (S4), a
  counted model validated against the real counter on 8,856 calls, real FES
  and Boolean F4 arms on the same descended draws. Model crossover below FES at
  N = 80 (Dinur, 2^71.3-bit working set) and N = 128 (LPTWY); none under a
  2^43-bit memory cap. Verdict INCONCLUSIVE only because a sensitivity control
  moved 1.36%/3.99% against a frozen 1%. BKW 2019 is not a separate arm (its
  d = 2 exponent 0.804 lies between Dinur 0.694 and LPTWY 0.877).
  **CLOSED-BY-PRIOR**; not re-run.

So only (a) is measured here.

## 1. Question

On identical Weil-descent decomposition instances (Koblitz family `K` of
`ic descent`, `S_{m+1}(x_1..x_m, x_R) = 0`, `x_i` in `V = span{1, z, ..., z^{l-1}}`
in the smallest-mask irreducible polynomial basis, b = 1), what does the
literal "fix m-1 points, gcd(f, L_V mod f)" oracle (arm **G**) cost in counted
field operations, against the counted word XORs of the descent engines
`inherited-f4` (arm **I**, the Koblitz oracle default) and `fes-f2` (arm **F**,
m = 2 only), and where in (n, l, m) does G cross each?

## 2. Arms

* **G (test)**: lane-local Rust `l4bench gcd`. m = 2: Gray-code walk over all
  `x_1` in V with `x_1^2`, `x_1 x_R`, `(x_1 x_R)^2` updated by one addition each
  (all F_2-linear in `x_1`); `f(X) = S3(x_1, X, x_R)`; leading coefficients
  inverted in batches of 1024 (Montgomery: 3 mul each + 1 inversion a batch);
  `L_V mod f` by the Horner form in `phi(u) = u^2 mod f` (the same form as
  `semaev_decomp::LvHorner`); gcd with the degree-1 remainder decided without
  inversion; roots extracted only on success. m = 3: all ordered pairs
  `(x_1, x_2)` in V^2, `f(X) = Res_U(S3(x_1,x_2,U), S3(U,X,x_R))` (degree 4),
  monic by a per-row batch inversion, `L_V mod f` by Horner with the
  `t^4, t^6 mod f` table, pseudo-remainder gcd, roots of a non-trivial gcd found
  by evaluation over V (charged). Precomputation (`L_V`, Horner `b_j`, basis
  tables) charged once per cell, reported separately.
* **I (baseline, incumbent default)**: `inherited-f4` run by the repository's
  `ic descent --solver inherited-f4 --solver fes-f2` (crypto `HEAD` at the
  time of the run, release build), its `ops` in "word XORs (elimination,
  specialisation and linear elimination only)": a partial count, the matrix
  builds are not counted, so it is a lower bound on I and favours I.
* **F (baseline)**: `fes-f2` from the same invocation, `ops = 2*2^N + setup`
  word XORs (one word holds all equations).
* **S (cited, not run)**: the pair-table/subtract family that the current
  baseline rows use; its counted per-target work is quoted from
  `pdp_root_finder_20261012` (`subtract2`: `5|F| - 3` mul; `rf-mitm`:
  `5|F| - 3` mul, `2|F|` sqr, 1 inv, `2|F|` lookups).

Instances: the targets `ic descent` draws (seed 1, `--targets 8`,
`--repeats 2`); G reads `x_R` from that JSON, so every arm solves the same
inputs. Arm order inside `ic` rotates per repetition; G runs after (its counts
are deterministic, so order cannot move them).

## 3. Cells

* m = 2, descent regime (n' = ceil(n/2)): `K:11:6`, `K:13:7`, `K:19:10`,
  `K:29:15`, `K:37:19` (2 primitive mod n for all five; F declines above 32
  unknowns).
* m = 2, l-sweep at n = 29: n' = 6, 8, 10, 12, 14 (with 15 above).
* m = 3: `K:11:4`, `K:13:4`, `K:13:5`, `K:19:6` (I only; F declines cubic).
* Budget 120 s per call for I; a budget hit is recorded, never a verdict.
  Cells beyond what I finishes are reported as such.

## 4. Unit and pricing band

G counts field multiplications, squarings, additions and inversions. An
inversion is priced by Itoh-Tsujii as `(n-1)` sqr + `(bitlen(n-1) - 1 +
popcount(n-1) - 1)` mul. I and F count 64-bit word XORs. Field operations are
converted to word operations at three prices (the §5a-style band for this
lane): **P1** mul = sqr = add = 1 word op (one carry-less multiply, generous to
G); **P4** mul = 4, sqr = 3, add = 1 (clmul plus two folding clmuls);
**P2n** mul = 2n, sqr = n, add = 1 (bit-serial shift-and-add). An order
between two arms is stated only when it holds at all three prices.

## 5. Controls

* **Correctness gate (exact)**: on every target, G's ordered solution set (as
  `ic` Boolean masks, `x_i` coordinates at bits `i*n'..`) equals the reference
  engine's (`fes-f2` where it applies, else `inherited-f4`) and every decided
  engine's. Any mismatch -> INCONCLUSIVE.
* **Brute force (positive control of G)**: where `|V|^m <= 2^21`, G's set
  equals direct evaluation of S_{m+1} over V^m.
* **Positive control (gate can see solutions)**: at least one target per cell
  with a non-empty solution set; if none, add 8 more targets via `--seed 2`.
* **Negative control (gate can fail)**: G with `L_V` replaced by the subspace
  polynomial of a different subspace (`span{z, ..., z^l}`) must disagree with
  the reference on at least one target of every cell with a satisfiable target.
* **A/A / determinism**: G run twice gives identical counts; `ic`'s `ops` are
  identical across its 2 repetitions per target.

## 6. Pre-registered prediction

m = 2: G costs about `2^l (2l + c)` mul + `2^l (2l + c')` sqr per target, i.e.
2^{l + log2 l + O(1)}; F costs `2^{2l+1}`; I's word XORs grow about 2^{1.08 l}
(inherited_f4_next Run 1: 49k at 14 unknowns, 42.9M at 32).
Expect: G below F from l about 8-9 at every price; G vs I unresolved (order
flips inside the band) at l >= 10, G below I at all prices only at small l if
at all. m = 3: G costs 2^{2l} (~4l mul) — expect G above I's word XORs at P4
and P2n for l >= 5. G is above S (subtract/pair table) by a factor about l at
every cell and price (S is O(1) mul per candidate).

## 7. Decision rule

* Gate or a control fails -> **INCONCLUSIVE**.
* **IMPROVES** (an improvement over the IC baseline's decomposition stage):
  G's counted cost below S's cited cost, or below I's at all three prices, at
  some m = 3 cell or at the largest m = 2 cell, with the gate passing. (The IC
  baselines at n = 41 and 83 use pair-table / root-index oracles, so beating
  I or F alone is not an improvement of a baseline row.)
* **NEGATIVE**: G at or above S at every cell and every price.
* **NO-CHANGE**: otherwise.
* Descriptive outputs whatever the verdict: per cell the ratios I/G and F/G at
  P1, P4, P2n; the smallest l (and n in the regime family) where F/G > 1 and
  where I/G > 1 at all prices; fitted log2-slopes in l.

Falsifier of the prediction: F/G < 1 at P1 for some l >= 10 at m = 2, or I/G
> 1 at all three prices for every m = 2 cell with l >= 10.

## 8. Not done, and why

No end-to-end S, no rho arm: this is a decomposition-stage cost study in which
every arm prices one target; there is no cross-method ECDLP row, so the §5a
GAE band does not apply and no gap to rho is computed beyond quoting it.

## Amendment 1 (2026-11-14, before any measured run; after a smoke run at seed 9, not part of the data)

* The `ic descent --json` report has its cells under `cells` (not `measured`); the
  harness reads either. `--out` is not used; JSON goes to stdout.
* The smoke run showed that every engine also reports a **complete** `bitop/v1` tally
  (`extra.bitop_*`, `bitop_complete = 1`; `crypto:src/cryptanalysis/bitop_tally.rs`:
  word ops including matrix builds, hash probes, field ops). It is added as a
  **secondary** I and F metric: word ops + hash probes + field adds at 1, field
  mul/sqr/inv at the same P1/P4/P2n band as G; minimum over the repetitions (the first
  repetition of a process can carry a one-time tally, e.g. 175,680 vs 144,569 at the
  smoke's first target, while the native `ops` were equal). The frozen primary (native
  `ops`) and the decision rule are unchanged; the report states whether the complete
  count changes any order.
