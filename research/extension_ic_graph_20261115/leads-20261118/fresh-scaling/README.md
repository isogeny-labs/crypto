# Fresh targets at growing sizes: a draft protocol for graph IC against matched rho

Lane key `fresh-scaling`, 2026-11-18. This is a **design, not a run**. Nothing
was dispatched, no paid compute was used, and no pcc claim was registered.
Every number below is a producer observation or a model, labelled
[pcc:unverified].

## Question

Every graph-IC result in this study so far uses one fixed, known-answer
instance at `p = 1009` (stage41/nagao-e2e: seed389, `l = 263807419614139553`).
One instance cannot show how the cost grows. This lane designs a
preregistered experiment that can. The experiment draws **fresh public
targets** from a committed nonce at **six growing sizes** of the same
weak-cover `E(F_{p^6})` family. It runs the frozen graph IC and the
registered rho portfolio on every target. It reports per-size ratios as the
distance remaining, and fits exponents with intervals. The draft also
handles critic finding F4 (the negative-binomial descent residual count).

## What is here

| File | What it is |
|---|---|
| [`protocol.draft.json`](protocol.draft.json) | `experiment.protocol/v1` draft. Status `draft`: no hash, commit, nonce, worker build or analysis program is pinned. |
| [`gen/`](gen/) | `fresh-scaling-gen`, a zero-dependency Rust target generator, public-input checker, nonce/seed deriver and runtime estimator. |
| [`estimate.csv`](estimate.csv) | Per-size runtime and CPU-hour model (`fresh-scaling-gen estimate --n 32`). |
| [`dry-run/`](dry-run/) | 18 dry-run targets (3 per size) under the placeholder nonce `DRYRUN-NOT-A-BEACON-20261118`, with their manifests, the independent check output and the public inputs (tar.gz). **These are not evidence and not measured targets.** |

## Design summary

**The family and path.** The family is the existing `jv_cover.rs` weak
family, `y^2 = x(x - α)(x - σα)` over `F_{q^3}`, `q = p^2`, with
`#E = 4l` and `l` prime. The size ladder is
`p = 127, 223, 383, 661, 1153, 2017`, about ×1.74 per step, so
`b = log2 l ≈ 39.9, 44.8, 49.5, 54.2, 59.0, 63.9`. Across the ladder:

- The ladder spans 24 bits and stays in band B1 (`b < 64`) and bounds tier `medium`.
- The cofactor is fixed at 4. The genus-3 cover over `F_{p^2}`, the
  `F_p`-indexed factor base and the Nagao parameters are those of the
  nagao-e2e baseline.
- The cap is `p ≤ 2039`, because the frozen library holds `l` in a u64.
- `l` is tied to `p`: `l = (p^6 + 1 - t)/4` with `|t| ≤ 2p^3`. The
  dry-run `l` values agree to 9 digits within each size, as lesson
  L-12d575623c2f requires.

**Targets** (`gen/`). Each target is a pure function of
`(scheme, nonce, p, index, kind)`. Every draw comes from SHA-256 counter
streams with separate roles (`curve`, `generator`, `target`), and every
target has its own fresh curve.

- **Public targets.** `Q` is hashed to the curve and multiplied by the
  cofactor, so nobody knows `log_G Q` (SUCCESS_CRITERIA §1).
- **Planted targets.** These exist only for controls. Their scalar goes
  to a separate `owner/` file and never into `public.json`.

A candidate curve is accepted only when all of these hold:

- `α ∉ F_{p^2}`;
- the `Cover::new` polynomials lie in `F_q`;
- `j(E)` is in neither `F_{p^2}` nor `F_{p^3}`, so negation is the only
  automorphism and no Frobenius fold applies to rho;
- BSGS in the library's Hasse window gives `#E = 4l` with `l` prime, and
  two more points are killed by `4l`;
- there is no embedding degree `k ≤ 1000`.

The pipeline's own public validation then checks the transfer to `Jac_H` in
an admission pass. That pass reads no answer and runs no solve.

**Nonce scheme.** The measured nonce is SHA-256 over:

- the frozen protocol hash;
- the commit id of the first `main` commit that contains `COMMITMENT.json`;
- a drand mainnet round at least one hour after that commit, and its
  randomness.

The command is `fresh-scaling-gen nonce ...`. The pilot and planted-control
nonce depends on the protocol hash alone. Worker seeds are derived per
target and role with `fresh-scaling-gen seed`. The measured set is the first
`n` admitted indices per size. Every refused index is listed with its
reason.

**Arms.** The arms run on every target:

- `ic-graph`: the frozen quadratic-remainder graph IC with the six-variable
  Nagao finder.
- `ic-graph-aa`: the same target with an independent worker seed.
- The four registered rho configurations: plain or negation, each with
  batch 1 or 16. The reference is the cheapest at each size (and at each
  weight once counts are complete).
- `rho-aa`: `rho-neg-b16` with an independent seed.

An optional `descent-probe` arm needs a new opt-in entry point and a
separate amendment before any data.

**Controls.**

- Positive, correctness: two planted targets per size, run first, must
  return the owner scalar in IC and in rho. The committed p1009 bridge
  fixture must reproduce its 1506 residuals and its scalar.
- Positive, fit: rho's fitted exponent must lie in `[0.47, 0.53]`.
- Negative: the plain-versus-negation rho exponent difference must contain 0.
- A/A: the IC A/A and rho A/A ratio intervals must contain 1.

**Metrics.**

- **M2** is the primary metric that can run today: complete-process CPU
  seconds on an isolab-eks L2 host, single thread, fresh process. It
  covers everything from public-input parse to the scalar replay on E.
- **M1** is complete counted GAE across the §5a band. It is **blocked by
  gate G-cov**: the descent linear stage (sparse FGLM, `gcd(f, x^p - x)`,
  read-out) is outside the `jv_bitops` hooks, so recorded descent
  subtotals omit at least about 25% (descent-finder README). M1 is reported
  only if G-cov passes on the bridge fixture before measurement.

**n and stopping.** Each size gets `n = 32` admitted public targets
(SUCCESS_CRITERIA T1: at least 32 below b = 64), plus 2 planted controls.
`n` is fixed, with no sequential adding. The rules are:

- Sizes run in ascending order.
- The global budget is 80 CPU-hours.
- More than 2 of 32 IC failures at a size stops the larger sizes.
- IC runs are capped at max(1800 s, 20× the model).
- Rho runs are capped at 20× the expected number of steps. A capped run is
  charged as a failure at the work it consumed.

**Analysis.**

- Per size: `R_s = Σ C_IC / Σ C_rho_ref`, with a paired bootstrap over
  targets (2000 resamples, seed from the nonce).
- Exponents: OLS of `ln mean C` on `ln mean l`, and also on `ln p^6`, for
  every arm and every IC phase. The interval is a stratified bootstrap
  (targets resampled within size).
- Local slopes between neighbouring sizes and a quadratic-in-`ln l` fit are
  reported beside the line.
- Holm correction over the 6 ratio tests plus the slope test.
- Arm labels stay masked until the frozen Rust analysis program has run.

**Critic F4 (negative-binomial descent).** A run stops after 2 accepted
descents. The residual count is therefore NB(2, π), with π ≈ 1/1100 observed
(2 accepted in 1506, 1728 and 3415 residuals at p1009). The per-run CV of the
descent part is about `sqrt((1 - π)/2) ≈ 0.71`, and IC's CV is about
`0.71 × f_d`, where `f_d` is the descent share of the cost. The draft
handles this in four ways:

1. Every cost is measured complete, every residual included, over 32
   targets, so `Σ C_IC` is the sum of 64 successes' worth of NB draws.
2. A descent-normalised secondary reading is
   `E[C] = mean(C_nd) + (2/π̂)·mean(c_res)`, with `π̂ = ΣA/ΣR` pooled per
   size. This normalisation separates the acceptance rate from the cost per
   residual. **It does not shrink the variance.** π̂ rests on the same 64
   successes, a relative SE of about 12.5%.
3. An IC-versus-IC comparison is paired on the worker seed, so both arms
   consume the identical residual stream, checked by stream signature. This
   removes the NB term. Where a candidate changes the stream, a descent
   difference is read only with at least 100 pooled successes per arm per
   size, from n = 50 targets or the probe.
4. The optional probe (M = 200,000 residuals per size, about 200 successes,
   about 13 CPU-minutes per size at the modelled 3.8 ms per residual) is the
   only cheap way to tighten π̂.

**Expected precision** (model). Per-size ratio half-width is about ±30% at
p127, where descent dominates, and about ±20% at p2017. The statistical SE
of the exponent is about 0.009. The fit's uncertainty is therefore dominated
by model misfit (the fixed descent cost flattens the small sizes), which is
why local slopes and curvature are reported.

## Results of the local work

All are [pcc:unverified]. All are local, finite and deterministic. None
times the IC or rho.

1. **Generator compatibility.** `cargo test --offline --release` passes
   8/8. The committed p101 (stage32 native-rho) and p1009 (nagao-e2e)
   public inputs pass the independent checker under this re-implemented
   tower: their points lie on the curve, `l` is prime, `[l]G = [l]Q = O`,
   `#E = 4l` by fresh BSGS, the cover admits α, `j` is outside the proper
   subfields, and there is no small embedding degree. Those points satisfy
   the curve equation only if `w` and `s` match the library's tower, so the
   test confirms that emitted coordinates are in the library's basis. Also
   tested: determinism, planted `[d]G = Q`, and refusal of tampered `l`,
   answer-like keys and off-curve points.
2. **Dry run** (placeholder nonce, not evidence). 18/18 targets generated
   and checked. Generation took 0.01–29 s per target, 143 s in total, single
   thread. Accepted α per attempt was 3/46, 3/113, 3/146, 3/221, 3/277 and
   3/125 for p = 127 … 2017. All 910 rejections were `#E ≠ 4·prime`.
3. **Runtime and cost model** ([`estimate.csv`](estimate.csv)). Rho's
   per-step times come from the Run50 Runpod rho runs at p1009, one walk
   each: plain-b16 148 ns, plain-b1 204 ns, neg-b16 167 ns, neg-b1 220 ns
   per step. The IC model is `13 s·(p/1009)^e + 2000 residuals × 3.8 ms`
   with `e ∈ {2, 3}`. This model is an assumption: **no IC run exists at
   any size other than p1009.**

   For all arms with n = 32, the model gives about 0.15, 0.19, 0.40, 1.4,
   6.8 and 35 CPU-hours per size, and **about 45 CPU-hours in total**.
   Rho is about 42 of those, because the four configurations plus A/A at
   p2017 cost about 610–1050 s per run. With the pilot and overheads the
   total is about 55 CPU-hours. Under the model, IC is above rho at p127
   and p223 (rho under 1.5 s, IC about 8 s) and well below rho at p1153 and
   p2017. This is prediction P2/P1, not a result.

## Failures, gaps and blockers

- **research-kb unavailable.** No MCP tool was found, and
  `scripts/research-kb` reports "no executable found". No KB source ids are
  recorded.
- **ecdb unavailable.** `ECDB_TOKEN` is unset, there is no AWS CLI to read
  secret `ecdb/api`, and the function URL returns 403. The curve facts were
  computed by the checker instead.
- **No `experiment.protocol/v1` checker** exists in this checkout
  (`tools/experiment_protocol.py` is absent, and so is the sibling
  crypto-autoresearcher). The draft parses as JSON and follows the field
  layout of `research/arity_completion_20261104/prime_tower_m5/protocol.json`.
  It is not schema-checked.
- **Capacity risk at small p.** Amendment 16's relation-capacity model
  flags `p < 1009` for the older sieve collector. Gate G-pilot (planted
  pilot through IC at every size, before the measured nonce exists) decides
  which sizes survive. At least 4 sizes spanning 16 bits are needed;
  otherwise the result is inconclusive.
- **Distance remaining to a tier** (from the protocol's `distance_remaining`):
  - M1 is blocked by the unhooked descent linear stage.
  - This family has no ecbench family or `challenge commit`.
  - There is no entry point for the T1 negative control (`n = 2` Weil
    restriction or a random-base oracle).
  - The T2 held-out size needs `b ≥ 72`, which the u64 scalar path
    prevents.
  - A cross-host audit, VALIDATION.json and pcc verified are absent.
- The near-2^64 `l` at p2017 (`b = 63.87`) has never been run through the
  library's IC or rho. The pilot covers it.

## Commands run

```
df -h /                                   # 15 GB free before the build
cd gen && CARGO_TARGET_DIR=/home/user/cargo-target-round2/fresh-scaling \
  cargo build --offline --release        # zero dependencies; 1.9 MB target dir
cargo test --offline --release           # 8 passed
fresh-scaling-gen batch --nonce DRYRUN-NOT-A-BEACON-20261118 \
  --sizes 127,223,383,661,1153,2017 --n 3 --out <scratch>   # dry-run/generate.log
fresh-scaling-gen check <scratch>/p*/public-*/public.json # dry-run/check.jsonl, 18 ok
fresh-scaling-gen estimate --n 32 > estimate.csv
scripts/ecdb.sh get /v1/meta             # refused: ECDB_TOKEN unset
scripts/research-kb search ...           # no executable found
```

Build identity: rustc 1.97.0. Binary sha256
`65d642d943b8fe7f97e06ad682d53fb81ce0a5041651645f305c0a55a0ee1ede`, built
from `gen/src/*.rs` with sha256s `main.rs b2a1e0bc…`, `gen.rs e54e0f46…`,
`field.rs d1139f52…`, `curve.rs 657aa0a0…`, `sha256.rs 3ce19a92…` and
`json.rs 2c003d60…`. Freezing pins the full hashes again.

## Sources read

- `docs/ic/SUCCESS_CRITERIA.md`, in full.
- `docs/VALIDATION_GATE.md`, the VALIDATION.json section.
- `docs/bounds/README.md`: §3.1 tiers and §3.4 challenge/beacon.
- The `controlled-experiment` skill and `tools/lessons.py brief`
  (L-12d575623c2f applied).
- nagao-e2e `README.md`, `PLAN.md`, `native/worker.rs` and
  `results/summary.json`.
- Run50 `*.time.txt` and rho `raw.json` step counts.
- descent-finder `README.md` and `breakdown.json`.
- run51-outcome `README.md`.
- AMENDMENT_16 and `validation/cover-validation-capacity-model.json`.
- `src/cryptanalysis/jv_cover.rs`: tower, `EllE`, `group_order_4_prime`,
  `Cover::new`, `generate_spec`, `spec_from_curve`.
- Joux–Vitse 2012, cited through the lane documents. It was not re-read
  here.

## Next steps and new leads

1. **Pilot (cheap, local-eligible).** Run the planted pilot targets through
   the nagao-e2e worker at every ladder size, one each, counts plus
   indicative wall. This settles G-pilot and the IC scaling exponent `e`
   before any paid run. It needs the historical library build, about 1–2 GB
   on disk.
2. **G-cov, which unblocks M1.** Route the descent linear stage
   (zero_dim_la sparse FGLM, gcd, read-out) through the `jv_bitops` hooks.
   The descent-finder already measured the gap. This is the single change
   that turns M2-only into a GAE comparison.
3. **u128 scalar path.** Carry `l` past 2^64 (p up to about 2^21 in the
   generator's u128 tower) so that a T2 held-out size (`b ≥ 72`) becomes
   feasible. The generator's field already uses u128 exponents. Its
   u64-specific parts are `l`, the Miller–Rabin test and the embedding
   check.
4. **ecbench family registration** (`ecbench-extend`). Adding this
   generator as an ecbench curve construction would let
   `ecbench challenge commit` replace the hand nonce scheme.
5. **Descent-probe entry point.** An opt-in descent-only mode that reuses
   the Nagao decomposer on independent residuals. It is cheap and tightens
   π̂ to about 7%. It also tests whether π depends on `p`, which matters for
   the extrapolation.
6. **Negative-control entry point.** A decomposition oracle on a random
   (non-subfield) base, expected to yield no relations, run on the same
   targets.
7. Freeze: write the Rust analysis program, pin binaries, commit
   `COMMITMENT.json`, then draw the drand round. Only after that, the
   measured nonce.
