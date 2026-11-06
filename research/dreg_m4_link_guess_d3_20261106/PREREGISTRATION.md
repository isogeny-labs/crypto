# Pre-registration: one invariant guess per link at m = 4, guess-then-degree-3, k = 10

Frozen 2026-11-06 before any study run. Disclosed pre-freeze data is listed
under "Pilot data seen before freezing".

## Question

At m = 3, (22, 8), three guesses and degree 4, an invariant guess on the
second link (`e₁ + x₃`) did as well as the raw guess on x₃
(`research/dreg_second_link_guess_20261105/`, #3504), and an invariant guess on
the first link beat a raw one (#2883). The m = 3 three-guess regime does not
exist at m = 4 on this host: every m = 4 cell that fits refutes at degree 4
with no guess (`research/dreg_closure_m4l6_20261018/RESULTS.md`).

So the question is asked at degree 3, where m = 4 needs about 13–15 guesses
(pilot below). **Question: at m = 4 with k = 10 guesses, does placing the
guesses on the three links' swap invariants (arm I) make degree 3 refute on a
larger share of branches than the same number of raw coordinate guesses
(arm R)?** The owner chose k = 10 (2026-11-06).

## System and arms

The chain `S₃(x₁, x₂, e₁)`, `S₃(e₁, x₃, e₂)`, `S₃(e₂, x₄, x(R))`, ℓ = 5.
Variables: `x₁ = 0–4`, `x₂ = 5–9`, `x₃ = 10–14`, `x₄ = 15–19`, then `e₁` and
`e₂` (n each) from 20. Branch `b` ∈ 0..1023 sets `c_i` = bit `i` of `b`.

- **Arm I (one invariant per link):**
  - link 1 (`s = x₁ + x₂`), 4 guesses: `x₉ := x₄ + c₀`, `x₈ := x₃ + c₁`,
    `x₇ := x₂ + c₂`, `x₆ := x₁ + c₃`;
  - link 2 (`e₁ + x₃`), 3 guesses `--link 2:top1=c₄,2:top2=c₅,2:top3=c₆`;
  - link 3 (`e₂ + x₄`), 3 guesses `--link 3:top1=c₇,3:top2=c₈,3:top3=c₉`.
  - `topI` is the I-th largest coordinate k with |T_k| ≥ 2 on the draw, where
    T_k = {t : v_t has bit k}. The rule uses the draw's basis only.
- **Arm R (raw, same places):** `x₉, x₈, x₇, x₆ := c₀..c₃` (x₂, top four),
  `x₁₄, x₁₃, x₁₂ := c₄..c₆` (x₃, top three), `x₁₉, x₁₈, x₁₇ := c₇..c₉`
  (x₄, top three).
- Both arms remove 10 variables.

## Draws (all committed, every unsatisfiable draw of each file)

- (n, ℓ) = (12, 5): draws 24, 26, 47, 52 of
  `research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl`.
- (n, ℓ) = (11, 5): draws 8, 67, 107, 143 of
  `research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl`.
- `topI` resolves on all 8 draws (checked with `--to 2`, which builds no
  closure).

## Measurement (`run.sh`)

- Every branch of both arms on every draw: 2 × 8 × 1024 = 16,384 runs of
  `dreg_linear_guess --to 3 --max-cols 140000 --check`, one process each,
  under `prlimit --as=8e9 timeout 600`. No early stop.
- One file per arm and draw, line `b` for branch `b`. A run that leaves no
  line stops the study; the file is moved to `runs/attempts/` before a retry.
- **Identity control N:** the new build (`runs/binary.sha256`) reruns m = 3
  `S3.d5.b0` and must equal the committed
  `research/dreg_invariant_guess_20261029/runs/S3.d5.b0.jsonl` on every scan
  field except `secs`.

## Validity

- **V1:** every record's draw, n, ℓ and guesses match its file and line,
  including arm I's link sources and coordinates, re-derived from the draw's
  basis by the scorer. `identity_check` is true.
- **V2:** N matches.
- **V3:** all 1024 branches of each arm and draw are present.

## Metric and verdict

- Per arm and draw: `r` = branches whose closure refutes at degree 3, out of
  1024.
- A draw is *discordant* when `r_I ≠ r_R`.
- With k_I draws where `r_I > r_R` and k_R where `r_R > r_I` (d = k_I + k_R),
  a two-sided exact sign test: `p = min(1, 2·P(Bin(d, ½) ≥ max(k_I, k_R)))`.
  - **I_better:** p ≤ 0.05 and k_I > k_R.
  - **R_better:** p ≤ 0.05 and k_R > k_I.
  - **No signal:** `r_I = r_R = 0` on every draw.
  - **No detected difference:** anything else.
- With 8 draws, p ≤ 0.05 needs at least 7 of 7 or 8 of 8 discordant draws
  one way (7 of 8 gives p = 0.070).
- Descriptive: `r` per arm and draw, the pooled share, and per cell.

## Pilot data seen before freezing

- Feasibility pilot (scratch, unregistered, old build): on (12,5) draws 24 and
  26, branch 0 of a placement with 5 pair sums and 8–10 raw fixes. Degree 3
  first refutes at 13 guesses on draw 24 and 15 on draw 26; 3 guesses never
  refute.
- Timing check (new build): draw 24, branch 0 of both arms. Arm I refuted at
  degree 3 and arm R did not. These two runs are not study records; the study
  reruns them.

## Predictions (written before any study run)

- P(I_better) ≈ 0.45, P(no detected difference) ≈ 0.4, P(R_better) ≈ 0.1,
  P(no signal) ≈ 0.05. The timing check moved I_better up.

## What it would mean for ECC2K-130

- **I_better:** invariant placement helps at m = 4 at equal guess count, and
  the per-link rule carries along the chain in the degree-3 regime.
- **Not I_better:** at m = 4 the advantage seen at m = 3 is not reproduced in
  this regime.
- Either way, a stage-level reading of guess choice: no `S`, no rho ratio, no
  leaderboard row. It does not test the m = 3 three-guess degree-4 regime at
  m = 4. Unverified until a pcc verdict exists.

## Scope

m = 4, two cells at ℓ = 5, 8 draws, the 4-core 16 GB container, not isolated.
Iso Lab EKS is not reachable from it, and Conductor is not installed.
